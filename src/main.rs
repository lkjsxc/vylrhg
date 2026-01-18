use std::env;
use std::time::{SystemTime, UNIX_EPOCH};
use tokio::fs::{self, OpenOptions};
use tokio::io::{stdin, AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::time::{interval, Duration};

mod core;
mod assembly;
mod layout;
mod markup;
mod renderer;
mod session;
mod tabs;

use crate::core::commands::{parse_line, Command};
use crate::core::event_bus::{Event, EventBus};
use crate::layout::bindings::TileBindings;
use crate::layout::{LayoutTree, SplitDir};
use crate::renderer::pipeline::{RenderOp, Renderer};
use crate::session::snapshot::SessionSnapshot;
use crate::tabs::TabManager;

fn now_epoch_secs() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs()
}

async fn write_status(data_dir: &str, message: &str) -> std::io::Result<()> {
    fs::create_dir_all(data_dir).await?;
    let log_path = format!("{}/boot.log", data_dir.trim_end_matches('/'));
    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(log_path)
        .await?;
    let line = format!("{} {}\n", now_epoch_secs(), message);
    file.write_all(line.as_bytes()).await
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let data_dir = env::var("DATA_DIR").unwrap_or_else(|_| "/data".to_string());
    write_status(&data_dir, "vylrhg start").await?;

    let mut bus = EventBus::new(128);
    let tx = bus.sender();
    let mut ticker = interval(Duration::from_secs(5));
    let mut tabs = TabManager::new();
    let mut renderer = Renderer::new();
    let mut layout = LayoutTree::new();
    let mut bindings = TileBindings::new(tabs.active_id().unwrap_or(1));

    tokio::spawn(async move {
        loop {
            ticker.tick().await;
            if tx.send(Event::Tick).await.is_err() {
                break;
            }
        }
    });

    let shutdown_tx = bus.sender();
    tokio::spawn(async move {
        let _ = tokio::signal::ctrl_c().await;
        let _ = shutdown_tx.send(Event::Shutdown).await;
    });

    let input_tx = bus.sender();
    tokio::spawn(async move {
        let stdin = stdin();
        let mut reader = BufReader::new(stdin).lines();
        while let Ok(Some(line)) = reader.next_line().await {
            if let Some(command) = parse_line(&line) {
                match command {
                    Command::Help => {
                        let _ = input_tx
                            .send(Event::Input("sys:help".to_string()))
                            .await;
                    }
                    Command::Input(command_line) => {
                        if input_tx.send(Event::Input(command_line)).await.is_err() {
                            break;
                        }
                    }
                }
            }
        }
    });

    while let Some(event) = bus.recv().await {
        if let Some(message) = tabs.handle_event(&event) {
            let _ = write_status(&data_dir, &message).await;
        }
        let mut layout_changed = false;
        let mut bindings_changed = false;
        if let Event::Input(command) = &event {
            if let Some(dir) = command.strip_prefix("layout:split ") {
                let split = match dir.trim() {
                    "h" => Some(SplitDir::Horizontal),
                    "v" => Some(SplitDir::Vertical),
                    _ => None,
                };
                if let Some(split) = split {
                    let id = layout.split_active(split);
                    let _ = write_status(&data_dir, &format!("layout split id={}", id)).await;
                    layout_changed = true;
                }
            } else if let Some(id) = command.strip_prefix("layout:focus ") {
                if let Ok(parsed) = id.trim().parse::<u64>() {
                    let ok = layout.focus(parsed);
                    let _ = write_status(&data_dir, &format!("layout focus ok={}", ok)).await;
                    layout_changed = ok;
                }
            } else if let Some(rest) = command.strip_prefix("tile:bind ") {
                let mut parts = rest.split_whitespace();
                let tile = parts.next().and_then(|v| v.parse::<u64>().ok());
                let tab = parts.next().and_then(|v| v.parse::<u64>().ok());
                if let (Some(tile), Some(tab)) = (tile, tab) {
                    if tabs.has_tab(tab) && layout.leaf_ids().contains(&tile) {
                        let ok = bindings.bind(tile, tab);
                        let _ = write_status(&data_dir, &format!("tile bind ok={}", ok)).await;
                        bindings_changed = ok;
                    } else {
                        let _ = write_status(&data_dir, "tile bind invalid").await;
                    }
                }
            } else if let Some(id) = command.strip_prefix("tile:unbind ") {
                if let Ok(tile) = id.trim().parse::<u64>() {
                    let ok = bindings.unbind(tile);
                    let _ = write_status(&data_dir, &format!("tile unbind ok={}", ok)).await;
                    bindings_changed = ok;
                }
            } else if command == "tile:map" {
                let _ = write_status(&data_dir, &format!("tile map {}", bindings.describe())).await;
            }
        }
        let tiles = layout.leaf_ids();
        bindings.ensure_tiles(&tiles, tabs.active_id().unwrap_or(1));
        let mut ops = renderer.handle_event(&event);
        if layout_changed {
            ops.push(RenderOp::Text(format!("render layout {}", layout.describe())));
        }
        if bindings_changed {
            ops.push(RenderOp::Text(format!("render bindings {}", bindings.describe())));
        }
        for op in &ops {
            let _ = write_status(&data_dir, &format!("{:?}", op)).await;
        }

        let snapshot = SessionSnapshot::from_state(&tabs, renderer.frame(), &layout, &bindings, &ops);
        let _ = write_snapshot(&data_dir, &snapshot.to_json()).await;
        if matches!(event, Event::Shutdown) {
            break;
        }
    }

    Ok(())
}

async fn write_snapshot(data_dir: &str, payload: &str) -> std::io::Result<()> {
    fs::create_dir_all(data_dir).await?;
    let path = format!("{}/session.json", data_dir.trim_end_matches('/'));
    fs::write(path, payload).await
}
