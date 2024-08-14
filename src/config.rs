use freedesktop_desktop_entry::DesktopEntry;
use iced::{widget::button, Background, Border, Color, Length, Radius, Theme};

use crate::app_tray::desktop_entry::DesktopEntryCache;

#[derive(Debug, Clone)]
pub struct PanelConfig {
    pub favorites: Vec<String>,
}

impl<'a> Default for PanelConfig {
    fn default() -> Self {
        Self {
            favorites: vec![
                "com.system76.CosmicTerm".to_string(),
                "org.mozilla.firefox".to_string(),
                "org.kde.discover".to_string(),
            ],
        }
    }
}
