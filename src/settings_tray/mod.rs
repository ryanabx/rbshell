use clock_subscription::{Clock, ClockMessage};
use iced::widget::row;

mod clock_subscription;

#[derive(Clone, Debug)]
pub struct SettingsTray {
    clock: Clock,
}

#[derive(Clone, Debug)]
pub enum SettingsTrayMessage {
    Clock(ClockMessage),
}

impl SettingsTray {
    pub fn new() -> Self {
        Self {
            clock: Clock::new(),
        }
    }

    pub fn handle_message(
        &mut self,
        message: SettingsTrayMessage,
    ) -> iced::Command<SettingsTrayMessage> {
        match message {
            SettingsTrayMessage::Clock(clock_msg) => self
                .clock
                .handle_message(clock_msg)
                .map(SettingsTrayMessage::Clock),
        }
    }

    pub fn view(&self) -> iced::Element<SettingsTrayMessage> {
        iced::widget::container(row![self.clock.view().map(SettingsTrayMessage::Clock)]).into()
    }

    pub fn subscription(&self) -> iced::Subscription<SettingsTrayMessage> {
        self.clock.subscription().map(SettingsTrayMessage::Clock)
    }
}
