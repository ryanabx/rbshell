use std::{
    env,
    path::{Path, PathBuf},
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

pub mod app_tray;
mod config;
mod panel;
mod settings_tray;

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
}

fn main() -> Result<(), PanelError> {
    env_logger::Builder::from_env(Env::default().default_filter_or("warn")).init();
    let args = CliArgs::parse();
    log::trace!("Received args: {:?}", args);
    let config = PanelConfig::from_file_or_default(
        &args
            .config
            .unwrap_or(Path::new(&env::var("HOME").unwrap()).join(".config/rbshell/config.json")),
    );
    let res = iced::daemon(Panel::title, Panel::update, Panel::view)
        // iced::application(Panel::title, Panel::update, Panel::view)
        .subscription(Panel::subscription)
        //     // .window_size((1280.0, 48.0))
        .theme(Panel::theme)
        // .decorations(false)
        .run_with(|| Panel::new(config))
        .map_err(PanelError::Iced);

    res
}

fn compositor_default() -> String {
    let current_compositor = env::var("XDG_CURRENT_DESKTOP");
    match current_compositor.as_deref() {
        Ok(val) => val.to_string(),
        _ => panic!(
            "Unsupported desktop. Specify a compositor with the argument `--compositor <COMPOSITOR>` for example, `--compositor COSMIC`"
        ),
    }
}
