use std::{
    env,
    path::{Path, PathBuf},
};

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum IconTheme {
    Breeze,
    Cosmic,
    None,
}

#[derive(Clone, Debug)]
pub enum ImageHandle {
    Svg(iced::widget::svg::Handle),
    Image(iced::widget::image::Handle),
}

impl ImageHandle {
    pub fn from_path(path: &Path) -> Self {
        if path.extension().is_some_and(|extension| extension == "svg") {
            Self::Svg(iced::widget::svg::Handle::from_path(path))
        } else {
            Self::Image(iced::widget::image::Handle::from_path(path))
        }
    }
}

impl Default for IconTheme {
    fn default() -> Self {
        // guess icon theme
        let current_desktop = env::var_os("XDG_CURRENT_DESKTOP")
            .map(|x| x.to_ascii_lowercase().to_string_lossy().to_string());
        match current_desktop.as_deref() {
            Some("kde") => Self::Breeze,
            Some("cosmic") => Self::Cosmic,
            Some(_) => Self::None, // Don't know what desktop this is
            None => Self::None,
        }
    }
}

pub fn default_icon_path(theme: &IconTheme) -> Option<PathBuf> {
    match theme {
        IconTheme::Breeze => freedesktop_icons::lookup("wayland")
            .with_theme("breeze")
            .with_cache()
            .find(),
        IconTheme::Cosmic => freedesktop_icons::lookup("application-default")
            .with_theme("Cosmic")
            .with_cache()
            .find(),
        IconTheme::None => None,
    }
}

pub fn start_menu_icon(theme: &IconTheme) -> Option<PathBuf> {
    match theme {
        IconTheme::Breeze => freedesktop_icons::lookup("applications-all")
            .with_theme("breeze")
            .with_cache()
            .find(),
        IconTheme::Cosmic => freedesktop_icons::lookup("applications-office")
            .with_theme("Cosmic")
            .with_cache()
            .find(),
        IconTheme::None => None,
    }
}
