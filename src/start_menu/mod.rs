use std::rc::Rc;

use iced::{
    alignment::{Horizontal, Vertical},
    border::Radius,
    widget::{button, text},
    Background, Border, Length, Theme,
};

use crate::{component::Component, desktop_entry::DesktopEntryCache};

#[derive(Clone, Debug)]
pub enum StartMenuMessage {
    MenuToggle,
}

#[derive(Clone, Debug)]
pub struct StartMenu<'a> {
    de_cache: Rc<DesktopEntryCache<'a>>,
}

impl<'a> StartMenu<'a> {
    pub fn new(de_cache: Rc<DesktopEntryCache<'a>>) -> Self {
        Self { de_cache }
    }
}

impl<'a> Component for StartMenu<'a> {
    type Message = StartMenuMessage;

    fn view(&self) -> iced::Element<Self::Message> {
        iced::widget::container(
            iced::widget::button(iced::widget::horizontal_space())
                .width(Length::Fill)
                .height(Length::Fill)
                .style(Self::tray_button_style)
                .on_press(Self::Message::MenuToggle),
        )
        .width(48)
        .height(48)
        .padding(4.0)
        .into()
    }

    fn handle_message(&mut self, message: Self::Message) -> iced::Task<Self::Message> {
        match message {
            StartMenuMessage::MenuToggle => todo!(),
        }
    }

    fn subscription(&self) -> iced::Subscription<Self::Message> {
        todo!()
    }
}

impl<'a> StartMenu<'a> {
    fn tray_button_style(theme: &Theme, status: button::Status) -> button::Style {
        let mut border_color = theme.palette().primary;
        let mut background_color = theme.palette().primary;
        (border_color.a, background_color.a) =
            if matches!(status, button::Status::Hovered | button::Status::Pressed) {
                (0.46, 0.36)
            } else {
                (0.36, 0.26)
            };

        button::Style {
            background: Some(Background::Color(background_color)),
            border: Border {
                radius: Radius::from(8.0),
                color: border_color,
                width: 8.0,
            },
            ..Default::default()
        }
    }
}
