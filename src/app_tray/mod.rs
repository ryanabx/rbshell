use std::{
    collections::HashMap,
    rc::Rc,
    sync::{Arc, Mutex},
};

use super::freedesktop::desktop_entry::DesktopEntryCache;
use cctk::wayland_client::protocol::wl_seat::WlSeat;
use compositor::{
    CompositorBackend, CompositorToplevelInfo, ToplevelHandle, WaylandIncoming, WaylandOutgoing,
};
use iced::{Element, Task};

use crate::{
    config::PanelConfig,
    design::{
        component_theme::{button_style, PANEL_SIZE},
        components::app_tray_button,
    },
    freedesktop::desktop_entry::EntryInfo,
};

pub mod compositor;

#[derive(Clone, Debug)]
pub struct AppTray<'a> {
    de_cache: Rc<DesktopEntryCache<'a>>,
    backend: CompositorBackend,
    config: Arc<Mutex<PanelConfig>>,
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
    pub fn new(config: Arc<Mutex<PanelConfig>>, de_cache: Rc<DesktopEntryCache<'a>>) -> Self {
        Self {
            de_cache,
            backend: CompositorBackend::new(),
            config,
        }
    }

    pub fn view(&self) -> iced::Element<AppTrayMessage> {
        let active_window = self.backend.active_window();
        // Get app tray apps
        let panel_config = self.config.lock().unwrap().clone();
        let app_tray_apps = panel_config
            .inner
            .app_tray
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
                        if panel_config.inner.app_tray.favorites.contains(app_id) {
                            None
                        } else {
                            Some((app_id.clone(), info.clone()))
                        }
                    }),
            )
            .filter_map(|(app_id, group)| {
                let entry = &self.de_cache.fuzzy_match(&app_id);

                self.view_tray_item(&app_id, entry.as_ref(), group, active_window.clone())
            })
            .map(|x| {
                Element::from(
                    iced::widget::container(x)
                        // .width(Length::Fill)
                        .width(PANEL_SIZE as u16)
                        .height(PANEL_SIZE as u16)
                        .padding(4.0),
                )
            });
        iced::widget::row(app_tray_apps).into()
    }

    pub fn handle_message(&mut self, message: AppTrayMessage) -> iced::Task<AppTrayMessage> {
        match message {
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
            AppTrayMessage::ContextMenu(_) => unreachable!(),
        }
    }

    pub fn subscription(&self) -> iced::Subscription<AppTrayMessage> {
        self.backend
            .wayland_subscription()
            .map(AppTrayMessage::WaylandIn)
    }

    fn view_tray_item(
        &self,
        app_id: &str,
        entry: Option<&EntryInfo<'a>>,
        app_info: HashMap<ToplevelHandle, CompositorToplevelInfo>,
        active_window: Option<ToplevelHandle>,
    ) -> Option<iced::widget::MouseArea<'a, AppTrayMessage>> {
        if entry.is_none() || entry.is_some_and(|e| e.invisible) {
            return None;
        }
        let is_active = active_window.is_some_and(|window| app_info.contains_key(&window));
        let num_toplevels = app_info.len();
        let image_handle = entry.and_then(|e| e.entry_image.clone());
        Some(
            iced::widget::mouse_area(
                app_tray_button(image_handle, is_active, num_toplevels, false)
                    .on_press_maybe(if num_toplevels == 0 {
                        entry
                            .and_then(|entry| entry.desktop_entry.exec())
                            .map(|exec| {
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
                        button_style(theme, status, is_active, num_toplevels)
                    }),
            )
            .on_right_press(AppTrayMessage::ContextMenu(app_id.to_string())),
        )
        // .on_press_maybe(if toplevels.is_empty() {
        //     launch_on_preferred_gpu(desktop_info, gpus)
        // } else if toplevels.len() == 1 {
        //     toplevels.first().map(|t| Message::Toggle(t.0.clone()))
        // } else {
        //     Some(Message::TopLevelListPopup((*id).into(), window_id))
        // })
    }
}
