use iced::widget::horizontal_space;

use crate::components::{app_icon, default_icon_path};

use super::SettingsTrayMessage;

#[derive(Clone, Debug)]
pub struct StatusIcons {}

impl StatusIcons {
    pub fn new() -> Self {
        Self {}
    }

    pub fn view(&self) -> iced::Element<'static, SettingsTrayMessage> {
        let icon_path = freedesktop_icons::lookup("start-here")
            .with_cache()
            .find()
            .unwrap();

        // .unwrap_or(default_icon_path());
        iced::widget::row![app_icon(&icon_path)].into()
    }
}
