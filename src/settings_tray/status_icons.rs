use crate::{
    design::components::app_icon,
    freedesktop::icons::{default_icon_path, IconTheme},
};

use super::SettingsTrayMessage;

#[derive(Clone, Debug)]
pub struct StatusIcons {}

impl StatusIcons {
    pub fn new() -> Self {
        Self {}
    }

    pub fn view(&self, icon_theme: &IconTheme) -> iced::Element<'static, SettingsTrayMessage> {
        let icon_path = freedesktop_icons::lookup("network-wireless")
            .with_theme("AdwaitaLegacy")
            .with_cache()
            .find()
            .or_else(|| default_icon_path(icon_theme));
        iced::widget::row![app_icon(&icon_path.unwrap())].into()
    }
}
