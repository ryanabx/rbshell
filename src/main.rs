use std::{
    env,
    path::{Path, PathBuf},
    sync::{Arc, Mutex},
};

use clap::Parser;
use config::{ConfigError, PanelConfig};
use env_logger::Env;
// use iced::{
//     application::{
//         actions::layer_surface::SctkLayerSurfaceSettings, layer_surface::Anchor, InitialSurface,
//     },
//     Application, Settings,
// };

use panel::Panel;

pub mod design;
pub mod freedesktop;

mod config;

mod panel;

pub mod app_tray;
mod settings_tray;
pub mod start_menu;

/// ryanabx desktop shell for wayland desktops
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct CliArgs {
    /// Specify the configuration directory for the config file
    /// Defaults to ~/.config/rbshell/config.json
    #[arg(long)]
    config: Option<PathBuf>,
    /// The scale to bring all the components up by
    #[arg(long)]
    scale: Option<f32>,
}

#[derive(Debug, thiserror::Error)]
enum PanelError {
    #[error("Config: {0}")]
    Config(#[from] ConfigError),
    #[error("Iced: {0}")]
    Iced(#[from] iced::Error),
    #[error("Mutex poison on config")]
    MutexPoison,
}

fn main() -> anyhow::Result<()> {
    env_logger::Builder::from_env(Env::default().default_filter_or("warn")).init();
    let args = CliArgs::parse();
    log::trace!("Received args: {:?}", args);
    let config_path = args
        .config
        .unwrap_or(Path::new(&env::var("HOME").unwrap()).join(".config/rbshell/config.json"));
    let config = Arc::new(Mutex::new(PanelConfig::from_file_or_default(&config_path)));
    let config_handle = config.clone();
    let res = iced::daemon(Panel::title, Panel::update, Panel::view)
        // iced::application(Panel::title, Panel::update, Panel::view)
        .subscription(Panel::subscription)
        //     // .window_size((1280.0, 48.0))
        .theme(Panel::theme)
        // .decorations(false)
        .run_with(|| Panel::new(config_handle))
        .map_err(PanelError::Iced);
    let _ = config
        .lock()
        .map_err(|_| PanelError::MutexPoison)?
        .save_to_file();
    Ok(res?)
}
