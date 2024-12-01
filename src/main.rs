use notify::{RecommendedWatcher, Watcher, Event, EventKind, RecursiveMode, Config};
use std::fs;
use std::path::Path;
use std::process::Command;
use log::{info, error};
use tokio::runtime::Runtime;

fn main() {
    env_logger::init();

    // Get the current working directory
    let watch_dir = std::env::current_dir().expect("Failed to get current working directory");

    info!("Starting Redis file monitor in {:?}", watch_dir);

    let runtime = Runtime::new().unwrap();

    runtime.block_on(async {
        if let Err(err) = watch_directory(&watch_dir).await {
            error!("Error while monitoring directory: {}", err);
        }
    });
}

/// Watch a directory for new `.redis` files
async fn watch_directory(dir: &Path) -> Result<(), Box<dyn std::error::Error>> {
    let (tx, mut rx) = tokio::sync::mpsc::channel(100);

    let mut watcher = RecommendedWatcher::new(
        move |res| {
            if let Ok(event) = res {
                if let Err(err) = tx.blocking_send(event) {
                    eprintln!("Error sending event: {}", err);
                }
            } else if let Err(err) = res {
                eprintln!("Watcher error: {}", err);
            }
        },
        Config::default(),
    )?;

    watcher.watch(dir, RecursiveMode::NonRecursive)?;

    // Handle file events
    tokio::select! {
        _ = process_events(&mut rx) => {},
        _ = tokio::signal::ctrl_c() => {
            info!("Received shutdown signal, exiting...");
        }
    }

    Ok(())
}

async fn process_events(rx: &mut tokio::sync::mpsc::Receiver<Event>) {
    while let Some(event) = rx.recv().await {
        println!("DEBUG: Received event: {:?}", event); // Debug log for all events
        if let Some(path) = event.paths.get(0) {
            println!("DEBUG: Event path: {:?}", path); // Debug log for path
            if let EventKind::Modify(notify::event::ModifyKind::Data(
                notify::event::DataChange::Content,
            )) = event.kind
            {
                println!("DEBUG: ModifyKind::Data(Content) matched"); // Debug log for kind
                if path.extension().and_then(|ext| ext.to_str()) == Some("redis") {
                    info!("Detected .redis file: {:?}", path);
                    if let Err(err) = process_redis_file(path) {
                        error!("Failed to process {:?}: {}", path, err);
                    }
                }
            }
        }
    }
}

/// Process the `.redis` file by piping its contents to `redis-cli`
fn process_redis_file(path: &Path) -> Result<(), Box<dyn std::error::Error>> {
    let file_path = path.to_str().ok_or("Invalid file path")?;
    let redis_cli = std::env::var("REDIS_CLI_PATH").unwrap_or_else(|_| "redis-cli".to_string());

    // Validate file is not empty
    let metadata = fs::metadata(file_path)?;
    if metadata.len() == 0 {
        error!("Skipping empty file {:?}", path);
        return Ok(()); // Skip empty files
    }

    // Log the full command for debugging
    let command = format!("cat {} | {}", file_path, redis_cli);
    info!("Executing: {}", command);

    // Execute the `cat file | redis-cli` equivalent
    let output = Command::new("sh")
        .arg("-c")
        .arg(&command)
        .output()?;

    // Handle success or failure
    if output.status.success() {
        info!("Successfully processed file {:?}:\n{}", path, String::from_utf8_lossy(&output.stdout));
    } else {
        if output.status.code() == Some(127) {
            error!("Failed to process file {:?}: redis-cli not found. Ensure it is installed and in the PATH.", path);
        } else {
            error!(
                "Failed to process file {:?}:\n{}",
                path,
                String::from_utf8_lossy(&output.stderr)
            );
        }
    }

    Ok(())
}
