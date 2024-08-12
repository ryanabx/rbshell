use std::collections::HashMap;

use cctk::{sctk::reexports::calloop::channel::Sender, toplevel_info::ToplevelInfo};
use cosmic_protocols::toplevel_info::v1::client::zcosmic_toplevel_handle_v1::ZcosmicToplevelHandleV1;
use desktop_entry::DesktopEntryCache;
use freedesktop_desktop_entry::DesktopEntry;
use iced::Command;

use crate::compositor::cosmic_comp::{ToplevelUpdate, WaylandMessage, WaylandRequest};

pub mod desktop_entry;

#[derive(Clone, Debug)]
pub struct AppTray<'a> {
    pub wayland_sender: Option<Sender<WaylandRequest>>,
    pub de_cache: DesktopEntryCache<'a>,
    pub active_toplevels: HashMap<String, ApplicationGroup>,
}

impl<'a> Default for AppTray<'a> {
    fn default() -> Self {
        Self {
            wayland_sender: None,
            de_cache: DesktopEntryCache::new(),
            active_toplevels: HashMap::new(),
        }
    }
}

#[derive(Clone, Debug)]
pub struct ApplicationGroup {
    pub toplevels: HashMap<ZcosmicToplevelHandleV1, ToplevelInfo>,
}

impl<'a> AppTray<'a> {
    pub fn wayland_event(
        &mut self,
        event: WaylandMessage,
    ) -> Option<iced::Command<crate::Message>> {
        match event {
            WaylandMessage::Init(wayland_sender) => {
                self.wayland_sender.replace(wayland_sender);
                None
            }
            WaylandMessage::Finished => {
                println!("WHY?");
                None
            }
            WaylandMessage::Toplevel(toplevel_update) => match toplevel_update {
                ToplevelUpdate::Add(handle, info) => {
                    let app_id = info.app_id.clone();
                    println!("Adding toplevel with app_id {} to list!", &app_id);
                    if self.active_toplevels.contains_key(&app_id) {
                        self.active_toplevels
                            .get_mut(&info.app_id)
                            .unwrap()
                            .toplevels
                            .insert(handle, info);
                    } else {
                        self.active_toplevels.insert(
                            app_id.clone(),
                            ApplicationGroup {
                                toplevels: HashMap::from([(handle, info.clone())]),
                            },
                        );
                    }
                    None
                }
                ToplevelUpdate::Update(handle, info) => {
                    // TODO probably want to make sure it is removed
                    if info.app_id.is_empty() {
                        self.active_toplevels.remove(&info.app_id);
                        return Some(Command::none());
                    } else if !self.active_toplevels.contains_key(&info.app_id) {
                        return Some(Command::none());
                    }

                    for (t_handle, t_info) in &mut self
                        .active_toplevels
                        .get_mut(&info.app_id)
                        .unwrap()
                        .toplevels
                    {
                        if &handle == t_handle {
                            *t_info = info;
                            break;
                        }
                    }

                    None
                }
                ToplevelUpdate::Remove(handle) => {
                    let mut target_app_id: Option<String> = None;
                    for (app_id, app_info) in self.active_toplevels.iter_mut() {
                        if app_info.toplevels.contains_key(&handle) {
                            println!("Removing toplevel with app_id {} from list!", &app_id);
                            app_info.toplevels.remove(&handle);
                            if app_info.toplevels.is_empty() {
                                target_app_id = Some(app_id.clone());
                            }
                            break;
                        }
                    }
                    if let Some(app_id) = target_app_id {
                        self.active_toplevels.remove(&app_id);
                    }
                    None
                }
            },
            _ => None,
        }
    }

    pub fn get_desktop_entry(&mut self, app_id: &str) -> Option<DesktopEntry<'a>> {
        self.de_cache.0.get(app_id).cloned()
    }
}
