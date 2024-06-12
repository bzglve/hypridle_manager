extern crate ron;
extern crate serde;
extern crate tokio;
extern crate upower_dbus;
extern crate zbus;

use futures::stream::StreamExt;
use log::{debug, error, info, LevelFilter};
use serde::Deserialize;
use simplelog::{Config as LogConfig, SimpleLogger};
use std::env;
use std::fs::File;
use std::path::PathBuf;
use std::sync::Arc;
use tokio::process::{Child, Command};
use tokio::sync::Mutex;
use upower_dbus::UPowerProxy;

#[derive(Deserialize)]
struct Config {
    hypridle_ac_conf: String,
    hypridle_bat_conf: String,
    log_level: String,
    hypridle_quiet: bool,
}

#[tokio::main]
async fn main() -> zbus::Result<()> {
    // Locate and load configuration
    let config_path = find_config_file("hypr", "hypridle_manager.ron").expect("Failed to locate config file");
    let config: Config = load_config(&config_path).expect("Failed to load configuration");

    // Initialize logging
    let log_level = match config.log_level.as_str() {
        "trace" => LevelFilter::Trace,
        "debug" => LevelFilter::Debug,
        "info" => LevelFilter::Info,
        "warn" => LevelFilter::Warn,
        "error" => LevelFilter::Error,
        _ => LevelFilter::Info,
    };
    SimpleLogger::init(log_level, LogConfig::default()).expect("Failed to initialize logger");

    let connection = zbus::Connection::system().await?;
    let upower = UPowerProxy::new(&connection).await?;

    let mut current_state = upower.on_battery().await?;
    debug!("Initial On Battery: {:?}", current_state);

    let hypridle_process = Arc::new(Mutex::new(start_hypridle(current_state, &config).await));

    let mut stream = upower.receive_on_battery_changed().await;

    while let Some(event) = stream.next().await {
        let new_state = event.get().await?;
        debug!("On Battery: {:?}", new_state);

        if new_state != current_state {
            current_state = new_state;

            // Restart the hypridle process with the new configuration
            let process_guard = hypridle_process.clone();
            let mut process = process_guard.lock().await;

            if let Some(ref mut p) = *process {
                let _ = p.kill().await;
                let _ = p.wait().await;
            }

            *process = start_hypridle(current_state, &config).await;
        }
    }

    Ok(())
}

async fn start_hypridle(on_battery: bool, config: &Config) -> Option<Child> {
    let conf_path = if on_battery {
        &config.hypridle_bat_conf
    } else {
        &config.hypridle_ac_conf
    };

    let mut cmd = Command::new("hypridle");
    cmd.arg("-c").arg(conf_path);

    if config.hypridle_quiet {
        cmd.arg("-q");
    }

    match cmd.spawn() {
        Ok(child) => {
            info!("Started hypridle with config: {:?}", conf_path);
            Some(child)
        }
        Err(e) => {
            error!("Failed to start hypridle: {:?}", e);
            None
        }
    }
}

fn find_config_file(app_name: &str, file_name: &str) -> Option<PathBuf> {
    // Check XDG_CONFIG_HOME
    if let Ok(config_home) = env::var("XDG_CONFIG_HOME") {
        let mut path = PathBuf::from(config_home);
        path.push(app_name);
        path.push(file_name);
        if path.exists() {
            return Some(path);
        }
    }

    // Fallback to ~/.config
    if let Some(home_dir) = dirs::home_dir() {
        let mut path = home_dir;
        path.push(".config");
        path.push(app_name);
        path.push(file_name);
        if path.exists() {
            return Some(path);
        }
    }

    None
}

fn load_config(path: &PathBuf) -> Result<Config, ron::de::SpannedError> {
    let file = File::open(path).expect("Failed to open config file");
    ron::de::from_reader(file)
}
