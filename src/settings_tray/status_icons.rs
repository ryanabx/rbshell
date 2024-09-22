use crate::{components::app_icon, desktop_entry::default_icon_path};

use super::SettingsTrayMessage;

#[derive(Clone, Debug)]
pub struct StatusIcons {}

impl StatusIcons {
    pub fn new() -> Self {
        Self {}
    }

    pub fn view(&self) -> iced::Element<'static, SettingsTrayMessage> {
        let icon_path = freedesktop_icons::lookup("network-wireless")
            .with_theme("AdwaitaLegacy")
            .with_cache()
            .find()
            .or_else(default_icon_path);
        iced::widget::row![app_icon(&icon_path.unwrap())].into()
    }
}
