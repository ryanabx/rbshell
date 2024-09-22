use std::{collections::HashMap, path::PathBuf};

use freedesktop_desktop_entry::{
    default_paths, get_languages_from_env, DesktopEntry, Iter, Locale, PathSource,
};

use super::icons::{default_icon_path, IconTheme};

#[derive(Clone, Debug)]
pub struct DesktopEntryCache<'a>(pub HashMap<String, EntryInfo<'a>>);

#[derive(Clone, Debug)]
pub struct EntryInfo<'a> {
    pub desktop_entry: DesktopEntry<'a>,
    pub icon_path: Option<PathBuf>,
    pub invisible: bool,
}

impl<'a> EntryInfo<'a> {
    pub fn new(desktop_entry: DesktopEntry<'a>, icon_theme: &IconTheme) -> Self {
        let invisible = desktop_entry.no_display()
            || desktop_entry.name(&get_languages_from_env()).is_none()
            || desktop_entry.terminal()
            || desktop_entry.exec().is_none();

        let icon_path = if invisible {
            None
        } else {
            desktop_entry.icon().and_then(|icon| {
                freedesktop_icons::lookup(icon)
                    .with_theme("hicolor")
                    .with_cache()
                    .find()
                    .or_else(|| default_icon_path(&icon_theme))
            })
        };
        Self {
            desktop_entry,
            icon_path,
            invisible,
        }
    }
}

impl<'a> DesktopEntryCache<'a> {
    pub fn new(icon_theme: &IconTheme) -> Self {
        let locales = get_languages_from_env();
        log::debug!("{:?}", default_paths());
        let entries = Iter::new(default_paths())
            .filter_map(|path| {
                let path_src = PathSource::guess_from(&path);
                if let Ok(entry) = DesktopEntry::from_path(path.clone(), &locales) {
                    log::debug!("{:?}::{}", path_src, &entry.appid);
                    return Some((entry.appid.to_string(), EntryInfo::new(entry, &icon_theme)));
                }
                None
            })
            .collect::<HashMap<String, _>>();
        Self(entries)
    }

    pub fn fuzzy_match(&self, pattern: &str) -> Option<EntryInfo<'a>> {
        self.0
            .get(pattern)
            .or_else(|| {
                self.0.values().find(|entry| {
                    // log::debug!("entry: {}", entry.appid);
                    entry
                        .desktop_entry
                        .startup_wm_class()
                        .is_some_and(|wm_class| {
                            // log::trace!("Fuzzy matching wm class {} == {}", wm_class, pattern);
                            wm_class == pattern
                        })
                })
            })
            .or_else(|| {
                self.0.values().find(|entry| {
                    entry
                        .desktop_entry
                        .name(&[Locale::default()])
                        .is_some_and(|name| name == pattern)
                })
            })
            .cloned() // TODO: Can I make this more efficient?
    }
}
