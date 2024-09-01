use std::collections::HashMap;

use freedesktop_desktop_entry::{
    default_paths, get_languages_from_env, DesktopEntry, Iter, Locale,
};

#[derive(Clone, Debug)]
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
        log::trace!("Entries: {:?}", entries.keys().collect::<Vec<_>>());
        Self(entries)
    }

    pub fn fuzzy_match(&self, pattern: &str) -> Option<DesktopEntry<'a>> {
        self.0
            .get(pattern)
            .or_else(|| {
                self.0.values().find(|entry| {
                    // log::debug!("entry: {}", entry.appid);
                    entry.startup_wm_class().is_some_and(|wm_class| {
                        // log::trace!("Fuzzy matching wm class {} == {}", wm_class, pattern);
                        wm_class == pattern
                    })
                })
            })
            .or_else(|| {
                self.0.values().find(|entry| {
                    entry
                        .name(&[Locale::default()])
                        .is_some_and(|name| name == pattern)
                })
            })
            .cloned() // TODO: Can I make this more efficient?
    }
}
