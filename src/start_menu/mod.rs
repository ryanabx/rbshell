use std::{path::Path, rc::Rc};

use freedesktop_desktop_entry::{get_languages_from_env, DesktopEntry};
use iced::{
    border::Radius,
    widget::{
        button, row,
        scrollable::{Direction, Scrollbar},
        text,
    },
    Background, Border, Element, Length, Task, Theme,
};

use crate::{
    component_theme::button_style, components::app_icon, desktop_entry::DesktopEntryCache,
};

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
            iced::widget::column(self.de_cache.0.values().filter_map(view_menu_item))
                .height(Length::Shrink)
                .width(Length::Fill)
                .spacing(10),
        )
        .direction(Direction::Vertical(Scrollbar::new()))
        .height(Length::Fill)
        .width(Length::Fill)
        .into()
    }
}

fn view_menu_item<'a>(
    desktop_entry: &DesktopEntry<'a>,
) -> Option<iced::Element<'a, StartMenuMessage>> {
    if desktop_entry.no_display()
        || desktop_entry.name(&get_languages_from_env()).is_none()
        || desktop_entry.terminal()
        || desktop_entry.exec().is_none()
    {
        return None;
    }
    let icon_path = desktop_entry
        .icon()
        .and_then(|icon| freedesktop_icons::lookup(icon).with_cache().find());
    Some(
        iced::widget::button(row![
            iced::widget::container(match icon_path {
                Some(path) => {
                    app_icon(&path)
                }
                None => {
                    // log::warn!("No icon for {}", desktop_entry.appid);
                    Element::from(iced::widget::horizontal_space())
                }
            })
            .width(32)
            .height(32),
            text!("{}", desktop_entry.name(&get_languages_from_env()).unwrap()),
        ])
        .style(|theme, status| button_style(theme, status, false, 0))
        .on_press(StartMenuMessage::Launch(desktop_entry.appid.to_string()))
        .width(Length::Fill)
        .into(),
    )
}
