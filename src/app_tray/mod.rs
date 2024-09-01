use std::{collections::HashMap, path::PathBuf};

use cctk::wayland_client::protocol::wl_seat::WlSeat;
use compositor::{
    CompositorBackend, CompositorToplevelInfo, ToplevelHandle, WaylandIncoming, WaylandOutgoing,
};
use desktop_entry::DesktopEntryCache;
use freedesktop_desktop_entry::DesktopEntry;
use iced::{
    border::Radius,
    widget::{button, column, Container},
    window::Id,
    Background, Border, Element, Length, Task, Theme,
};

use crate::config::AppTrayConfig;

pub mod compositor;
pub mod desktop_entry;

#[derive(Clone, Debug)]
pub struct AppTray<'a> {
    pub de_cache: DesktopEntryCache<'a>,
    backend: CompositorBackend,
    config: AppTrayConfig,
    context_menu: Option<Id>,
}

#[derive(Clone, Debug)]
pub enum AppTrayMessage {
    WaylandIn(WaylandIncoming),
    WaylandOut(WaylandOutgoing),
    NewSeat(WlSeat),
    RemovedSeat(WlSeat),
    ContextMenu(String),
}

impl<'a> AppTray<'a> {
    pub fn new(config: AppTrayConfig) -> Self {
        Self {
            de_cache: DesktopEntryCache::new(),
            backend: CompositorBackend::new(),
            config,
            context_menu: None,
        }
    }

    pub fn handle_message(&mut self, app_tray_message: AppTrayMessage) -> Task<AppTrayMessage> {
        match app_tray_message {
            AppTrayMessage::WaylandIn(evt) => {
                self.backend.handle_incoming(evt).unwrap_or(Task::none())
            }
            AppTrayMessage::WaylandOut(evt) => {
                self.backend.handle_outgoing(evt).unwrap_or(Task::none())
            }
            AppTrayMessage::NewSeat(_) => {
                log::trace!("New seat!");
                Task::none()
            }
            AppTrayMessage::RemovedSeat(_) => {
                log::trace!("Removed seat!");
                Task::none()
            }
            AppTrayMessage::ContextMenu(app_id) => {
                log::trace!("App id requested: {}", &app_id);
                Task::none()
            }
        }
    }

    pub fn view(&self) -> iced::Element<AppTrayMessage> {
        let active_window = self.backend.active_window();
        // Get app tray apps
        let app_tray_apps = self
            .config
            .favorites
            .iter()
            .map(|x| {
                let app_id = x.clone();
                (
                    app_id,
                    self.backend
                        .active_toplevels
                        .get(x)
                        .cloned()
                        .unwrap_or_default(),
                )
            })
            .chain(
                self.backend
                    .active_toplevels
                    .iter()
                    .filter_map(|(app_id, info)| {
                        if self.config.favorites.contains(app_id) {
                            None
                        } else {
                            Some((app_id.clone(), info.clone()))
                        }
                    }),
            )
            .map(|(app_id, group)| {
                let entry = &self.de_cache.fuzzy_match(&app_id);

                self.view_tray_item(
                    &app_id,
                    entry.as_ref(),
                    group,
                    active_window.as_ref().map(|f| f.clone()),
                )
            })
            .map(|x| {
                Element::from(
                    iced::widget::container(x)
                        // .width(Length::Fill)
                        .width(48)
                        .height(48)
                        .padding(4.0),
                )
            });
        iced::widget::row(app_tray_apps).into()
    }

    fn view_tray_item(
        &self,
        app_id: &str,
        desktop_entry: Option<&DesktopEntry<'a>>,
        app_info: HashMap<ToplevelHandle, CompositorToplevelInfo>,
        active_window: Option<ToplevelHandle>,
    ) -> iced::widget::MouseArea<'a, AppTrayMessage> {
        let is_active = active_window.is_some_and(|window| app_info.contains_key(&window));
        let num_toplevels = app_info.len();
        let icon_name = desktop_entry.and_then(|entry| entry.icon());
        let icon_path = icon_name
            .and_then(|icon| freedesktop_icons::lookup(icon).with_cache().find())
            .or_else(|| Self::get_default_icon());
        iced::widget::mouse_area(
            match icon_path {
                Some(path) => {
                    if path.extension().is_some_and(|x| x == "svg") {
                        iced::widget::button(column![
                            Self::get_horizontal_rule(is_active, num_toplevels, true),
                            iced::widget::svg(path)
                                .content_fit(iced::ContentFit::Contain)
                                .width(Length::Fill)
                                .height(Length::Fill),
                            Self::get_horizontal_rule(is_active, num_toplevels, false)
                        ])
                    } else {
                        iced::widget::button(column![
                            Self::get_horizontal_rule(is_active, num_toplevels, true),
                            iced::widget::image(path)
                                .content_fit(iced::ContentFit::Contain)
                                .width(Length::Fill)
                                .height(Length::Fill),
                            Self::get_horizontal_rule(is_active, num_toplevels, false)
                        ])
                    }
                }
                None => {
                    // log::trace!(
                    //     "Icon for app_id {} does not exist {:?} {:?}",
                    //     app_id,
                    //     icon_name,
                    //     desktop_entry
                    // );
                    iced::widget::button(iced::widget::Space::new(Length::Fill, Length::Fill))
                        .width(Length::Fill)
                        .height(Length::Fill)
                }
            }
            .width(Length::Fill)
            .height(Length::Fill)
            .padding(4)
            .on_press_maybe(if num_toplevels == 0 {
                desktop_entry.and_then(|entry| entry.exec()).map(|exec| {
                    AppTrayMessage::WaylandOut(WaylandOutgoing::Exec(
                        app_id.to_string(),
                        exec.to_string(),
                    ))
                })
            } else if num_toplevels == 1 {
                Some(AppTrayMessage::WaylandOut(WaylandOutgoing::Toggle(
                    app_info.keys().next().unwrap().clone(),
                )))
            } else {
                None
                // TODO
            })
            .style(move |theme, status| {
                Self::tray_button_style(theme, status, is_active, num_toplevels)
            }),
        )
        .on_right_press(AppTrayMessage::ContextMenu(app_id.to_string()))
        // .on_press_maybe(if toplevels.is_empty() {
        //     launch_on_preferred_gpu(desktop_info, gpus)
        // } else if toplevels.len() == 1 {
        //     toplevels.first().map(|t| Message::Toggle(t.0.clone()))
        // } else {
        //     Some(Message::TopLevelListPopup((*id).into(), window_id))
        // })
    }

    fn get_horizontal_rule(
        is_active: bool,
        num_toplevels: usize,
        force_transparent: bool,
    ) -> Container<'a, AppTrayMessage> {
        let transparent = force_transparent || num_toplevels == 0;
        iced::widget::container(
            iced::widget::horizontal_rule(1).style(move |theme: &Theme| {
                iced::widget::rule::Style {
                    color: if transparent {
                        iced::Color::TRANSPARENT
                    } else {
                        theme.palette().primary
                    },
                    width: (2.0) as u16,
                    radius: 4.into(),
                    fill_mode: iced::widget::rule::FillMode::Full,
                }
            }),
        )
        .width(Length::Fixed(if is_active { 12.0 } else { 6.0 }))
        .center_x(Length::Fill)
    }

    fn get_default_icon() -> Option<PathBuf> {
        freedesktop_icons::lookup("wayland").with_cache().find()
    }

    fn tray_button_style(
        theme: &Theme,
        status: button::Status,
        is_active: bool,
        num_toplevels: usize,
    ) -> button::Style {
        let mut border_color = theme.palette().primary;
        let mut background_color = theme.palette().primary;
        (border_color.a, background_color.a) = if num_toplevels == 0 {
            if matches!(status, button::Status::Hovered | button::Status::Pressed) {
                (0.11, 0.1)
            } else {
                (0.0, 0.0)
            }
        } else if is_active {
            if matches!(status, button::Status::Hovered | button::Status::Pressed) {
                (0.26, 0.25)
            } else {
                (0.21, 0.20)
            }
        } else {
            if matches!(status, button::Status::Hovered | button::Status::Pressed) {
                (0.11, 0.1)
            } else {
                (0.06, 0.05)
            }
        };

        button::Style {
            background: Some(Background::Color(background_color)),
            border: Border {
                radius: Radius::from(8.0),
                color: border_color,
                width: 1.0,
            },
            ..Default::default()
        }
    }

    pub fn subscription(&self) -> iced::Subscription<AppTrayMessage> {
        self.backend
            .wayland_subscription()
            .map(AppTrayMessage::WaylandIn)
        // iced::Subscription::none()
    }
}
