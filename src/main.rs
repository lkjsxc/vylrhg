use std::env;
use std::time::{SystemTime, UNIX_EPOCH};
use tokio::fs;
use tokio::time::{interval, Duration};

fn now_epoch_secs() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs()
}

async fn write_status(data_dir: &str, message: &str) -> std::io::Result<()> {
    fs::create_dir_all(data_dir).await?;
    let log_path = format!("{}/boot.log", data_dir.trim_end_matches('/'));
    let line = format!("{} {}\n", now_epoch_secs(), message);
    fs::write(log_path, line).await
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let data_dir = env::var("DATA_DIR").unwrap_or_else(|_| "/data".to_string());
    write_status(&data_dir, "vylrhg start").await?;

    let mut ticker = interval(Duration::from_secs(5));

    loop {
        tokio::select! {
            _ = ticker.tick() => {
                let _ = write_status(&data_dir, "vylrhg heartbeat").await;
            }
            _ = tokio::signal::ctrl_c() => {
                let _ = write_status(&data_dir, "vylrhg shutdown").await;
                break;
            }
        }
    }

    Ok(())
}
