use std::{path::Path, rc::Rc};

use freedesktop_desktop_entry::{get_languages_from_env, DesktopEntry, Locale};
use iced::{
    alignment::{Horizontal, Vertical},
    border::Radius,
    widget::{button, row, scrollable::{Direction, Scrollbar}, text},
    Background, Border, Element, Length, Task, Theme,
};

use crate::{components::app_icon, desktop_entry::DesktopEntryCache};

#[derive(Clone, Debug)]
pub enum StartMenuMessage {
    MenuToggle,
    Launch(String),
}

#[derive(Clone, Debug)]
pub struct StartMenu<'a> {
    de_cache: Rc<DesktopEntryCache<'a>>,
}

impl<'a> StartMenu<'a> {
    pub fn new(de_cache: Rc<DesktopEntryCache<'a>>) -> Self {
        Self { de_cache }
    }

    pub fn view(&self) -> iced::Element<StartMenuMessage> {
        iced::widget::container(
            iced::widget::button(iced::widget::horizontal_space())
                .width(Length::Fill)
                .height(Length::Fill)
                .style(Self::tray_button_style)
                .on_press(StartMenuMessage::MenuToggle),
        )
        .width(48)
        .height(48)
        .padding(4.0)
        .into()
    }

    pub fn handle_message(&mut self, message: StartMenuMessage) -> Task<StartMenuMessage> {
        match message {
            StartMenuMessage::MenuToggle => unreachable!(),
            StartMenuMessage::Launch(app_id) => {
                log::info!("Requested to launch {}", app_id);
                Task::none() // TODO: Actually handle this
            }
        }
    }

    pub fn subscription(&self) -> iced::Subscription<StartMenuMessage> {
        todo!()
    }

    fn tray_button_style(theme: &Theme, status: button::Status) -> button::Style {
        let mut border_color = theme.palette().primary;
        let mut background_color = theme.palette().primary;
        (border_color.a, background_color.a) =
            if matches!(status, button::Status::Hovered | button::Status::Pressed) {
                (0.80, 0.20)
            } else {
                (0.26, 0.10)
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

    pub fn view_popup(&self) -> iced::Element<StartMenuMessage> {
        iced::widget::scrollable(
            iced::widget::column(self.de_cache.0.values().map(view_menu_item))
                .height(Length::Shrink)
                .width(Length::Fill),
        ).direction(Direction::Vertical(Scrollbar::new()))
        .height(Length::Fill)
        .width(Length::Fill)
        .into()
    }
}

fn view_menu_item<'a>(desktop_entry: &DesktopEntry<'a>) -> iced::Element<'a, StartMenuMessage> {
    let icon_path = desktop_entry.icon();
    row![
        match icon_path {
            Some(path) => {
                app_icon(Path::new(path))
            }
            None => Element::from(iced::widget::horizontal_space()),
        },
        text!(
            "{}",
            desktop_entry
                .name(&get_languages_from_env())
                .unwrap_or("".into())
        ),
        text!("{}", desktop_entry.appid)
    ]
    .into()
}
