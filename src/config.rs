use freedesktop_desktop_entry::DesktopEntry;
use iced::{widget::button, Background, Border, Color, Length, Radius, Theme};

use crate::app_tray::desktop_entry::DesktopEntryCache;

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

    pub fn get_widget(
        app_tray_app: AppTrayApp<'a>,
    ) -> Option<iced::widget::Button<'a, crate::Message>> {
        match &app_tray_app.0.icon() {
            Some(icon) => {
                let icon_path = freedesktop_icons::lookup(icon).with_cache().find();
                println!("icon_path: {:?}", icon_path);
                icon_path.map(move |path| {
                    if path.extension().is_some_and(|x| x == "svg") {
                        iced::widget::button(
                            iced::widget::svg(path)
                                .content_fit(iced::ContentFit::Contain)
                                .width(Length::Fill)
                                .height(Length::Fill),
                        )
                    } else {
                        iced::widget::button(
                            iced::widget::image(path)
                                .content_fit(iced::ContentFit::Contain)
                                .width(Length::Fill)
                                .height(Length::Fill),
                        )
                    }
                    .width(Length::Fill)
                    .height(Length::Fill)
                    .padding(8)
                    .style(move |theme, status| app_tray_app.button_style(theme, status))
                    .on_press(crate::Message::Panic)
                })
            }
            None => None,
        }
    }

    fn button_style(&self, _theme: &Theme, status: button::Status) -> button::Style {
        button::Style {
            background: Some(Background::Color(Color {
                r: 1.0,
                g: 1.0,
                b: 1.0,
                a: if matches!(status, button::Status::Hovered | button::Status::Pressed) {
                    0.1
                } else {
                    0.0
                },
            })),
            border: Border {
                radius: Radius::from(8.0),
                color: Color {
                    r: 1.0,
                    g: 1.0,
                    b: 1.0,
                    a: if matches!(status, button::Status::Hovered | button::Status::Pressed) {
                        0.3
                    } else {
                        0.0
                    },
                },
                width: 1.0,
            },
            ..Default::default()
        }
    }
}
