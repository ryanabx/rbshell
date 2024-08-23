use clock::{Clock, ClockMessage};
use iced::{widget::row, Length, Task};

mod clock;

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

    pub fn handle_message(&mut self, message: SettingsTrayMessage) -> Task<SettingsTrayMessage> {
        match message {
            SettingsTrayMessage::Clock(clock_msg) => self
                .clock
                .handle_message(clock_msg)
                .map(SettingsTrayMessage::Clock),
        }
    }

    pub fn view(&self) -> iced::Element<SettingsTrayMessage> {
        iced::widget::container(row![self.clock.view().map(SettingsTrayMessage::Clock)])
            .center_y(Length::Fill)
            .width(Length::Fill)
            .align_x(iced::alignment::Horizontal::Right)
            .into()
    }

    pub fn subscription(&self) -> iced::Subscription<SettingsTrayMessage> {
        self.clock.subscription().map(SettingsTrayMessage::Clock)
    }
}
