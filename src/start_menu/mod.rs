use std::rc::Rc;

use freedesktop_desktop_entry::get_languages_from_env;
use iced::{
    widget::{
        row,
        scrollable::{Direction, Scrollbar},
        text,
    },
    Element, Length, Task,
};

use crate::{
    design::component_theme::{button_style, PANEL_SIZE},
    design::components::{app_icon, app_tray_button},
    freedesktop::{
        desktop_entry::{DesktopEntryCache, EntryInfo},
        icons::{start_menu_icon, IconTheme},
    },
};

#[derive(Clone, Debug)]
pub enum StartMenuMessage {
    MenuToggle,
    Launch(String),
}

pub struct StartMenu<'a> {
    pub de_cache: Rc<DesktopEntryCache<'a>>,
}

impl<'a> StartMenu<'a> {
    pub fn new(de_cache: Rc<DesktopEntryCache<'a>>) -> Self {
        Self { de_cache }
    }

    pub fn view(
        &self,
        icon_theme: &IconTheme,
        start_menu_opened: bool,
    ) -> iced::Element<StartMenuMessage> {
        let start_menu_icon_path = start_menu_icon(icon_theme);
        iced::widget::container(
            app_tray_button(start_menu_icon_path.as_deref(), start_menu_opened, 0, true)
                .on_press(StartMenuMessage::MenuToggle)
                .style(move |theme, status| button_style(theme, status, start_menu_opened, 0)),
        )
        .width(PANEL_SIZE as u16)
        .height(PANEL_SIZE as u16)
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

    pub fn view_popup(&self) -> iced::Element<StartMenuMessage> {
        let locales = get_languages_from_env();
        let mut keys = self.de_cache.0.iter().collect::<Vec<_>>();
        keys.sort_by(|e, e2| {
            e.1.desktop_entry
                .name(&locales)
                .unwrap_or("".into())
                .cmp(&e2.1.desktop_entry.name(&locales).unwrap_or("".into()))
        });
        iced::widget::scrollable(
            iced::widget::column(keys.iter().filter_map(|(_, val)| view_menu_item(val)))
                .height(Length::Shrink)
                .width(Length::Fill)
                .spacing(10),
        )
        .direction(Direction::Vertical(Scrollbar::new()))
        .height(Length::Fill)
        .width(Length::Fill)
        .into()
    }

    pub fn populate_menu_items(&mut self) {}
}

fn view_menu_item<'a>(entry: &EntryInfo<'a>) -> Option<iced::Element<'a, StartMenuMessage>> {
    if entry.invisible {
        None
    } else {
        Some(
            iced::widget::button(row![
                iced::widget::container(match &entry.icon_path {
                    Some(path) => {
                        app_icon(path)
                    }
                    None => {
                        // log::warn!("No icon for {}", desktop_entry.appid);
                        Element::from(iced::widget::horizontal_space())
                    }
                })
                .width(32)
                .height(32),
                text!(
                    "{}",
                    entry.desktop_entry.name(&get_languages_from_env()).unwrap()
                ),
            ])
            .style(|theme, status| button_style(theme, status, false, 0))
            .on_press(StartMenuMessage::Launch(
                entry.desktop_entry.appid.to_string(),
            ))
            .width(Length::Fill)
            .into(),
        )
    }
}
