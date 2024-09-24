use crate::{
    design::components::app_icon,
    freedesktop::icons::{default_icon_path, network_icon, IconTheme, ImageHandle},
};

use super::SettingsTrayMessage;

#[derive(Clone, Debug)]
pub struct StatusIcons {}

impl StatusIcons {
    pub fn new() -> Self {
        Self {}
    }

    pub fn view(&self, icon_theme: &IconTheme) -> iced::Element<'static, SettingsTrayMessage> {
        let icon_path = network_icon(icon_theme, 1.0);
        iced::widget::row![app_icon(ImageHandle::from_path(&icon_path.unwrap()))].into()
    }
}
