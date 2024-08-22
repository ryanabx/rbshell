use std::{env, path::Path};

use clap::Parser;
use config::{ConfigError, PanelConfig};
use env_logger::Env;
use iced::{
    application::{
        actions::layer_surface::SctkLayerSurfaceSettings, layer_surface::Anchor, InitialSurface,
    },
    Application, Settings,
};
use panel::PanelFlags;

mod app_tray;
mod config;
mod panel;
mod settings_tray;

/// ryanabx desktop shell for wayland desktops
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct CliArgs {
    /// Manually specify the assumed compositor to run under.
    /// Defaults to XDG_CURRENT_DESKTOP
    #[arg(long)]
    compositor: Option<String>,
    /// Specify the configuration directory for the config file
    /// Defaults to ~/.config/ryanabx-shell/config.json
    #[arg(long)]
    config: Option<String>,
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
    let layer_surface_settings = SctkLayerSurfaceSettings {
        anchor: Anchor::BOTTOM.union(Anchor::LEFT).union(Anchor::RIGHT),
        size: Some((None, Some(48))),
        layer: iced::application::layer_surface::Layer::Top,
        exclusive_zone: 48,
        ..Default::default()
    };
    let mut panel_settings = Settings::with_flags(PanelFlags {
        compositor: args.compositor.unwrap_or(compositor_default()),
        config: PanelConfig::from_file_or_default(Path::new(&args.config.unwrap_or(format!(
            "{}/.config/ryanabx-shell/config.json",
            env::var("HOME").unwrap()
        ))))?,
    });
    panel_settings.initial_surface = InitialSurface::LayerSurface(layer_surface_settings);
    panel::Panel::run(panel_settings).map_err(PanelError::Iced)
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
