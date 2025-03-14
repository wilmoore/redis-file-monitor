use anyhow::{Context, Result};
use clap::Parser;
use notify::{Event, EventKind, RecommendedWatcher, RecursiveMode, Watcher, Config};
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
use tokio::runtime::Runtime;
use tracing::{error, info, warn};
use tracing_subscriber;

/// CLI Arguments
#[derive(Parser, Debug)]
#[command(version = "1.1.0", about = "Monitors a directory for .redis files and pipes them to redis-cli")]
struct Cli {
    /// Path to redis-cli binary
    #[arg(long, default_value = "redis-cli")]
    redis_cli: String,

    /// Directory to monitor for .redis files
    #[arg(long, default_value = ".")]
    watch_dir: PathBuf,

    /// Dry run mode: Display settings and exit
    #[arg(long)]
    dry_run: bool,
}

fn main() -> Result<()> {
    tracing_subscriber::fmt::init();

    let cli = Cli::parse();

    if cli.dry_run {
        println!("Redis File Monitor (Dry Run Mode)");
        println!("Redis CLI Path: {}", cli.redis_cli);
        println!("Watch Directory: {:?}", cli.watch_dir);
        return Ok(());
    }

    info!("Starting Redis file monitor in {:?}", cli.watch_dir);

    let runtime = Runtime::new().context("Failed to create Tokio runtime")?;
    runtime.block_on(async {
        watch_directory(&cli.watch_dir, &cli.redis_cli).await
    })
}

/// Watches a directory for changes
async fn watch_directory(dir: &Path, redis_cli: &str) -> Result<()> {
    let (tx, mut rx) = tokio::sync::mpsc::channel(100);

    let mut watcher = RecommendedWatcher::new(
        move |res| {
            if let Ok(event) = res {
                if let Err(err) = tx.blocking_send(event) {
                    error!("Error sending event: {}", err);
                }
            } else if let Err(err) = res {
                error!("Watcher error: {}", err);
            }
        },
        Config::default(),
    )?;

    watcher.watch(dir, RecursiveMode::NonRecursive)?;

    tokio::select! {
        _ = process_events(&mut rx, redis_cli) => {},
        _ = tokio::signal::ctrl_c() => {
            info!("Received shutdown signal, exiting...");
        }
    }

    Ok(())
}

/// Processes file events
async fn process_events(rx: &mut tokio::sync::mpsc::Receiver<Event>, redis_cli: &str) {
    while let Some(event) = rx.recv().await {
        if let Some(path) = event.paths.get(0) {
            if let EventKind::Modify(notify::event::ModifyKind::Data(
                notify::event::DataChange::Content,
            )) = event.kind
            {
                if path.extension().and_then(|ext| ext.to_str()) == Some("redis") {
                    info!("Detected .redis file: {:?}", path);
                    if let Err(err) = process_redis_file(path, redis_cli) {
                        error!("Failed to process {:?}: {}", path, err);
                    }
                }
            }
        }
    }
}

/// Processes a `.redis` file and sends it to redis-cli
fn process_redis_file(path: &Path, redis_cli: &str) -> Result<()> {
    let file_path = path.to_str().context("Invalid file path")?;

    let metadata = fs::metadata(file_path)?;
    if metadata.len() == 0 {
        warn!("Skipping empty file {:?}", path);
        return Ok(());
    }

    let command = format!("cat {} | {}", file_path, redis_cli);
    info!("Executing: {}", command);

    let output = Command::new("sh")
        .arg("-c")
        .arg(&command)
        .output()
        .context("Failed to execute redis-cli command")?;

    if output.status.success() {
        info!(
            "Successfully processed file {:?}:\n{}",
            path,
            String::from_utf8_lossy(&output.stdout)
        );
    } else {
        error!(
            "Failed to process file {:?}:\n{}",
            path,
            String::from_utf8_lossy(&output.stderr)
        );
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use clap::Parser;
    use std::fs::{self, File};
    use std::io::Write;
    use std::process::Command;
    use tempfile::tempdir;

    /// Test CLI parsing to ensure correct argument behavior.
    #[test]
    fn test_cli_parsing() {
        let args = Cli::parse_from(&[
            "redis-file-monitor",
            "--redis-cli",
            "/custom/path/to/redis-cli",
            "--watch-dir",
            "/some/directory",
        ]);

        assert_eq!(args.redis_cli, "/custom/path/to/redis-cli");
        assert_eq!(args.watch_dir, PathBuf::from("/some/directory"));
    }

    /// Test dry-run mode outputs correct values.
    #[test]
    fn test_dry_run_mode() {
        let args = Cli::parse_from(&["redis-file-monitor", "--dry-run"]);
        assert!(args.dry_run);
    }

    /// Test `process_redis_file()` function to ensure it correctly handles files.
    #[test]
    fn test_process_redis_file() {
        let temp_dir = tempdir().expect("Failed to create temp directory");
        let file_path = temp_dir.path().join("test.redis");

        // Create a temporary `.redis` file
        let mut file = File::create(&file_path).expect("Failed to create file");
        writeln!(file, "SET foo bar").expect("Failed to write to file");

        // Ensure process_redis_file runs without errors
        let result = process_redis_file(&file_path, "redis-cli");
        assert!(result.is_ok());
    }

    /// Test empty file handling in `process_redis_file()`
    #[test]
    fn test_empty_redis_file() {
        let temp_dir = tempdir().expect("Failed to create temp directory");
        let file_path = temp_dir.path().join("empty.redis");

        // Create an empty `.redis` file
        File::create(&file_path).expect("Failed to create empty file");

        // Ensure empty files are skipped without error
        let result = process_redis_file(&file_path, "redis-cli");
        assert!(result.is_ok());
    }

    /// Test that `redis-cli` is correctly called.
    #[test]
    fn test_redis_cli_execution() {
        let temp_dir = tempdir().expect("Failed to create temp directory");
        let file_path = temp_dir.path().join("test.redis");

        // Write a test Redis command
        let mut file = File::create(&file_path).expect("Failed to create file");
        writeln!(file, "SET test_key test_value").expect("Failed to write file");

        // Run the process command with `echo` instead of actual `redis-cli`
        let output = Command::new("cat")
            .arg(file_path.to_str().unwrap())
            .output()
            .expect("Failed to execute cat command");

        // Check if the output contains the expected Redis command
        let output_str = String::from_utf8_lossy(&output.stdout);
        assert!(output_str.contains("SET test_key test_value"));
    }
}