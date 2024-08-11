use freedesktop_desktop_entry::DesktopEntry;
use iced::Length;

use crate::desktop_entry::DesktopEntryCache;

#[derive(Debug, Clone)]
pub struct PanelConfig<'a> {
    pub favorites: Vec<AppTrayApp<'a>>,
}

impl<'a> Default for PanelConfig<'a> {
    fn default() -> Self {
        let cache = DesktopEntryCache::new();
        Self {
            favorites: vec![
                AppTrayApp::new_from_appid("org.mozilla.firefox", &cache).unwrap(),
                AppTrayApp::new_from_appid("org.kde.discover", &cache).unwrap(),
            ],
        }
    }
}

#[derive(Debug, Clone)]
pub struct AppTrayApp<'a>(DesktopEntry<'a>);

impl<'a> AppTrayApp<'a> {
    pub fn new_from_appid(appid: &'a str, desktop_entry_cache: &DesktopEntryCache) -> Option<Self> {
        desktop_entry_cache
            .0
            .get(&appid.to_string())
            .map(|entry| Self(entry.to_owned()))
    }

    pub fn get_widget(&self) -> Option<iced::widget::Button<crate::Message>> {
        match self.0.icon() {
            Some(icon) => freedesktop_icons::lookup(icon)
                .with_cache()
                .find()
                .map(|path| {
                    iced::widget::button(
                        iced::widget::image(path).content_fit(iced::ContentFit::Fill),
                    )
                    .width(Length::Fill)
                    .height(Length::Fill)
                }),
            None => None,
        }
    }
}
