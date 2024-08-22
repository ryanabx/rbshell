use app_tray::{AppTray, AppTrayMessage};
use clap::Parser;
use config::{AppTrayConfig, PanelConfig};
use env_logger::Env;
use iced::{
    application::{
        actions::layer_surface::SctkLayerSurfaceSettings, layer_surface::Anchor, InitialSurface,
    },
    widget::{column, container::Style, row},
    Application, Background, Color, Command, Padding, Radius, Settings, Subscription, Theme,
};
use settings_tray::{SettingsTray, SettingsTrayMessage};

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

fn main() -> Result<(), iced::Error> {
    env_logger::Builder::from_env(Env::default().default_filter_or("warn")).init();
    let args = CliArgs::parse();
    let layer_surface_settings = SctkLayerSurfaceSettings {
        anchor: Anchor::BOTTOM.union(Anchor::LEFT).union(Anchor::RIGHT),
        size: Some((None, Some(48))),
        layer: iced::application::layer_surface::Layer::Top,
        exclusive_zone: 48,
        ..Default::default()
    };
    let mut panel_settings = Settings::with_flags(args);
    panel_settings.initial_surface = InitialSurface::LayerSurface(layer_surface_settings);
    panel::Panel::run(panel_settings)
}
