use clock::{Clock, ClockMessage};
use iced::{widget::row, Length, Task};
use status_icons::StatusIcons;

use crate::freedesktop::icons::IconTheme;

mod clock;
mod status_icons;

#[derive(Clone, Debug)]
pub struct SettingsTray {
    clock: Clock,
    status_icons: StatusIcons,
}

#[derive(Clone, Debug)]
pub enum SettingsTrayMessage {
    Clock(ClockMessage),
}

impl SettingsTray {
    pub fn new() -> Self {
        Self {
            clock: Clock::new(),
            status_icons: StatusIcons::new(),
        }
    }

    pub fn handle_message(&mut self, message: SettingsTrayMessage) -> Task<SettingsTrayMessage> {
        match message {
            SettingsTrayMessage::Clock(clock_msg) => self
                .clock
                .handle_message(clock_msg)
                .map(SettingsTrayMessage::Clock),
        }
    }

    pub fn view(&self, icon_theme: &IconTheme) -> iced::Element<SettingsTrayMessage> {
        iced::widget::container(row![
            self.status_icons.view(icon_theme),
            self.clock.view().map(SettingsTrayMessage::Clock),
        ])
        .center_y(Length::Fill)
        .width(Length::Fill)
        .align_x(iced::alignment::Horizontal::Right)
        .into()
    }

    pub fn subscription(&self) -> iced::Subscription<SettingsTrayMessage> {
        self.clock.subscription().map(SettingsTrayMessage::Clock)
    }
}
