use std::collections::HashMap;

use cctk::toplevel_info::ToplevelInfo;
use cosmic_protocols::toplevel_info::v1::client::zcosmic_toplevel_handle_v1::ZcosmicToplevelHandleV1;
use desktop_entry::DesktopEntryCache;
use freedesktop_desktop_entry::DesktopEntry;


pub mod desktop_entry;

#[derive(Clone, Debug)]
pub struct AppTray<'a> {
    pub de_cache: DesktopEntryCache<'a>,
    pub active_toplevels: HashMap<String, ApplicationGroup>,
}

impl<'a> Default for AppTray<'a> {
    fn default() -> Self {
        Self {
            de_cache: DesktopEntryCache::new(),
            active_toplevels: HashMap::new()
        }
    }
}

#[derive(Clone, Debug)]
pub struct ApplicationGroup {
    pub toplevels: HashMap<ZcosmicToplevelHandleV1, ToplevelInfo>,
}

impl<'a> AppTray<'a> {
    pub fn get_desktop_entry(&mut self, app_id: &str) -> Option<DesktopEntry<'a>> {
        self.de_cache.0.get(app_id).cloned()
    }
}
