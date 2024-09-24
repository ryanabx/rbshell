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

impl IconTheme {
    fn to_string(&self) -> String {
        match self {
            IconTheme::Breeze => "breeze".to_string(),
            IconTheme::Cosmic => "Cosmic".to_string(),
            IconTheme::None => "hicolor".to_string(),
        }
    }

    fn lookup(&self, icon: &str) -> Option<PathBuf> {
        freedesktop_icons::lookup(icon)
            .with_theme(&self.to_string())
            .with_cache()
            .find()
    }
}

pub fn default_icon_path(theme: &IconTheme) -> Option<PathBuf> {
    match theme {
        IconTheme::Breeze => theme.lookup("wayland"),
        IconTheme::Cosmic => theme.lookup("application-default"),
        IconTheme::None => None,
    }
}

pub fn start_menu_icon(theme: &IconTheme) -> Option<PathBuf> {
    match theme {
        IconTheme::Breeze => theme.lookup("applications-all"),
        IconTheme::Cosmic => theme.lookup("applications-office"),
        IconTheme::None => None,
    }
}

pub fn network_icon(theme: &IconTheme, strength: f32) -> Option<PathBuf> {
    match theme {
        IconTheme::Breeze => {
            if strength > 0.9 {
                theme.lookup("network-wireless-100")
            } else {
                theme.lookup("network-wireless-40")
            }
        }
        IconTheme::Cosmic => todo!(),
        IconTheme::None => todo!(),
    }
}
