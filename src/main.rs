use std::env;
use std::time::{SystemTime, UNIX_EPOCH};
use tokio::fs::{self, OpenOptions};
use tokio::io::AsyncWriteExt;
use tokio::time::{interval, Duration};

mod core;
mod assembly;
mod markup;
mod renderer;
mod tabs;

use crate::core::event_bus::{Event, EventBus};
use crate::renderer::pipeline::Renderer;
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

    while let Some(event) = bus.recv().await {
        if let Some(message) = tabs.handle_event(&event) {
            let _ = write_status(&data_dir, &message).await;
        }
        for op in renderer.handle_event(&event) {
            let _ = write_status(&data_dir, &format!("{:?}", op)).await;
        }
        if matches!(event, Event::Shutdown) {
            break;
        }
    }

    Ok(())
}
