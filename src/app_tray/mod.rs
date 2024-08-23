use std::{collections::HashMap, path::PathBuf};

use cctk::wayland_client::protocol::wl_seat::WlSeat;
use compositor::WaylandIncoming;
use desktop_entry::DesktopEntryCache;
use freedesktop_desktop_entry::DesktopEntry;
use iced::{
    event::{self, listen_with},
    widget::{button, column, Container},
    window::Id,
    Background, Border, Element, Length, Radius, Theme,
};

use crate::{
    app_tray::compositor::{CompositorBackend, WaylandOutgoing, WindowHandle, WindowInfo},
    config::AppTrayConfig,
};

mod compositor;
pub mod desktop_entry;

#[derive(Clone, Debug)]
pub struct AppTray<'a> {
    pub de_cache: DesktopEntryCache<'a>,
    pub active_toplevels: HashMap<String, ApplicationGroup>,
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
    pub fn new(config: AppTrayConfig, compositor: &str) -> Self {
        Self {
            de_cache: DesktopEntryCache::new(),
            active_toplevels: HashMap::new(),
            backend: CompositorBackend::new(compositor),
            config,
            context_menu: None,
        }
    }

    pub fn handle_message(
        &mut self,
        app_tray_message: AppTrayMessage,
    ) -> iced::Command<AppTrayMessage> {
        match app_tray_message {
            AppTrayMessage::WaylandIn(evt) => self
                .backend
                .handle_message(&mut self.active_toplevels, evt)
                .unwrap_or(iced::Command::none()),
            AppTrayMessage::WaylandOut(evt) => self
                .backend
                .handle_outgoing_message(&mut self.active_toplevels, evt)
                .unwrap_or(iced::Command::none()),
            AppTrayMessage::NewSeat(_) => {
                println!("New seat!");
                iced::Command::none()
            }
            AppTrayMessage::RemovedSeat(_) => {
                println!("Removed seat!");
                iced::Command::none()
            }
            AppTrayMessage::ContextMenu(app_id) => {
                println!("App id requested: {}", &app_id);
                iced::Command::none()
            }
        }
    }

    pub fn view(&self) -> iced::Element<AppTrayMessage> {
        // Get app tray apps
        let app_tray_apps = self
            .config
            .favorites
            .iter()
            .map(|x| {
                let app_id = x.clone();
                (
                    app_id,
                    self.active_toplevels
                        .get(x)
                        .cloned()
                        .unwrap_or(ApplicationGroup::default()),
                )
            })
            .chain(self.active_toplevels.iter().filter_map(|(app_id, info)| {
                if self.config.favorites.contains(app_id) {
                    None
                } else {
                    Some((app_id.clone(), info.clone()))
                }
            }))
            .map(|(app_id, group)| {
                let entry = &self.de_cache.0.get(&app_id);
                let active_window = self.backend.active_window(&self.active_toplevels);

                get_tray_widget(&app_id, *entry, group, active_window.map(|f| f.clone()))
            })
            .map(|x| {
                Element::from(
                    iced::widget::container(x)
                        .width(48.0)
                        .height(48.0)
                        .padding(4.0),
                )
            });
        iced::widget::row(app_tray_apps).into()
    }

    pub fn subscription(&self) -> iced::Subscription<AppTrayMessage> {
        iced::Subscription::batch(vec![
            self.backend
                .wayland_subscription()
                .map(AppTrayMessage::WaylandIn),
            listen_with(|e, _, _| match e {
                iced::Event::PlatformSpecific(event::PlatformSpecific::Wayland(
                    event::wayland::Event::Seat(e, seat),
                )) => match e {
                    event::wayland::SeatEvent::Enter => Some(AppTrayMessage::NewSeat(seat)),
                    event::wayland::SeatEvent::Leave => Some(AppTrayMessage::RemovedSeat(seat)),
                },
                _ => None,
            }),
        ])
    }
}

#[derive(Clone, Debug, Default)]
pub struct ApplicationGroup {
    pub toplevels: HashMap<WindowHandle, WindowInfo>,
}

pub fn get_tray_widget<'a>(
    app_id: &str,
    desktop_entry: Option<&DesktopEntry<'a>>,
    app_info: ApplicationGroup,
    active_window: Option<WindowHandle>,
) -> iced::widget::MouseArea<'a, AppTrayMessage> {
    let icon_path = desktop_entry
        .and_then(|entry| entry.icon())
        .and_then(|icon| freedesktop_icons::lookup(icon).with_cache().find())
        .or_else(|| get_default_icon());
    iced::widget::mouse_area(
        match icon_path {
            Some(path) => {
                if path.extension().is_some_and(|x| x == "svg") {
                    iced::widget::button(column![
                        get_horizontal_rule(&app_info, &active_window.as_ref(), true),
                        iced::widget::svg(path)
                            .content_fit(iced::ContentFit::Contain)
                            .width(Length::Fill)
                            .height(Length::Fill),
                        get_horizontal_rule(&app_info, &active_window.as_ref(), false)
                    ])
                } else {
                    iced::widget::button(column![
                        get_horizontal_rule(&app_info, &active_window.as_ref(), true),
                        iced::widget::image(path)
                            .content_fit(iced::ContentFit::Contain)
                            .width(Length::Fill)
                            .height(Length::Fill),
                        get_horizontal_rule(&app_info, &active_window.as_ref(), false)
                    ])
                }
            }
            None => iced::widget::button(iced::widget::Space::new(Length::Fill, Length::Fill))
                .width(Length::Fill)
                .height(Length::Fill),
        }
        .width(Length::Fill)
        .height(Length::Fill)
        .padding(4)
        .on_press_maybe(if app_info.toplevels.is_empty() {
            desktop_entry.and_then(|entry| entry.exec()).map(|exec| {
                AppTrayMessage::WaylandOut(WaylandOutgoing::Exec(
                    app_id.to_string(),
                    exec.to_string(),
                ))
            })
        } else if app_info.toplevels.len() == 1 {
            Some(AppTrayMessage::WaylandOut(WaylandOutgoing::Toggle(
                app_info.toplevels.keys().next().unwrap().clone(),
            )))
        } else {
            None
            // TODO
        })
        .style(move |theme, status| {
            tray_button_style(theme, status, &app_info, &active_window.as_ref())
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

fn get_horizontal_rule<'a>(
    app_info: &ApplicationGroup,
    active_window: &Option<&WindowHandle>,
    force_transparent: bool,
) -> Container<'a, AppTrayMessage> {
    let transparent = force_transparent || app_info.toplevels.is_empty();
    iced::widget::container(
        iced::widget::horizontal_rule(1)
            .style(move |theme: &Theme| iced::widget::rule::Style {
                color: if transparent {
                    iced::Color::TRANSPARENT
                } else {
                    theme.palette().primary
                },
                width: (2.0) as u16,
                radius: 4.into(),
                fill_mode: iced::widget::rule::FillMode::Full,
            })
            .width(Length::Fixed(
                if active_window.is_some_and(|w| app_info.toplevels.contains_key(w)) {
                    12.0
                } else {
                    6.0
                },
            )),
    )
    .center_x(Length::Fill)
}

fn get_default_icon() -> Option<PathBuf> {
    freedesktop_icons::lookup("wayland").with_cache().find()
}

fn tray_button_style<'a>(
    theme: &Theme,
    status: button::Status,
    app_info: &ApplicationGroup,
    active_window: &Option<&WindowHandle>,
) -> button::Style {
    let mut border_color = theme.palette().primary;
    let mut background_color = theme.palette().primary;
    (border_color.a, background_color.a) = if app_info.toplevels.is_empty() {
        if matches!(status, button::Status::Hovered | button::Status::Pressed) {
            (0.11, 0.1)
        } else {
            (0.0, 0.0)
        }
    } else if active_window.is_some_and(|x| app_info.toplevels.contains_key(x)) {
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
