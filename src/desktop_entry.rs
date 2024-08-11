use std::collections::HashMap;

use freedesktop_desktop_entry::{default_paths, get_languages_from_env, DesktopEntry, Iter};

pub struct DesktopEntryCache<'a>(pub HashMap<String, DesktopEntry<'a>>);

impl<'a> DesktopEntryCache<'a> {
    pub fn new() -> Self {
        let locales = get_languages_from_env();
        let mut entries = HashMap::<String, DesktopEntry<'a>>::new();
        for path in Iter::new(default_paths()) {
            if let Ok(entry) = DesktopEntry::from_path(path, Some(&locales)) {
                entries.insert(entry.appid.to_string(), entry);
            }
        }
        Self(entries)
    }
}
