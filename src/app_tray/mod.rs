use std::collections::HashMap;

use desktop_entry::DesktopEntryCache;
use freedesktop_desktop_entry::DesktopEntry;

use crate::compositor::{WindowHandle, WindowInfo};

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
            active_toplevels: HashMap::new(),
        }
    }
}

#[derive(Clone, Debug)]
pub struct ApplicationGroup {
    pub toplevels: HashMap<WindowHandle, WindowInfo>,
}

impl<'a> AppTray<'a> {
    pub fn get_desktop_entry(&mut self, app_id: &str) -> Option<DesktopEntry<'a>> {
        self.de_cache.0.get(app_id).cloned()
    }
}
