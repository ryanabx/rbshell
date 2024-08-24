use iced::{
    border::Radius,
    widget::{column, row, text},
    window::{self, Settings},
    Element, Length, Padding, Subscription, Task, Theme,
};

use crate::{
    app_tray::{compositor::Compositor, AppTray, AppTrayMessage},
    config::PanelConfig,
    settings_tray::{SettingsTray, SettingsTrayMessage},
};

#[derive(Clone, Debug)]
pub struct PanelFlags {
    pub compositor: Compositor,
    pub config: PanelConfig,
}

#[derive(Clone, Debug)]
pub struct Panel<'a> {
    app_tray: AppTray<'a>,
    settings_tray: SettingsTray,
    main_window: window::Id,
    popup_window: Option<(window::Id, String)>,
}

impl<'a> Panel<'a> {
    pub fn new(config: PanelConfig, compositor: Compositor) -> (Self, Task<Message>) {
        let (id, open) = window::open(window::Settings {
            decorations: false,
            size: (1280.0, 48.0).into(),
            ..Default::default()
        });
        log::debug!("Window requested open {:?}", id);
        (
            Self {
                app_tray: AppTray::new(config.app_tray, compositor),
                settings_tray: SettingsTray::new(),
                main_window: id,
                popup_window: None,
            },
            open.map(Message::OpenMainWindow),
        )
    }

    pub fn title(&self, _window: window::Id) -> String {
        "Window".into()
    }

    pub fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::AppTray(AppTrayMessage::ContextMenu(app_id)) => {
                let (_, task) = window::open(Settings {
                    position: window::Position::Centered,
                    ..Default::default()
                });

                task.map(move |i| Message::OpenPopup(i, app_id.clone()))
            }
            Message::AppTray(app_tray_msg) => self
                .app_tray
                .handle_message(app_tray_msg)
                .map(Message::AppTray),
            Message::SettingsTray(settings_tray_msg) => self
                .settings_tray
                .handle_message(settings_tray_msg)
                .map(Message::SettingsTray),
            Message::OpenPopup(id, app_id) => {
                let task = if let Some((popup, _)) = self.popup_window.take() {
                    iced::window::close(popup)
                } else {
                    Task::none()
                };
                log::debug!("Popup opened! {:?}", id);
                self.popup_window = Some((id, app_id));
                task
            }
            Message::OpenMainWindow(_) => Task::none(),
        }
    }

    pub fn theme(&self, _window: window::Id) -> Theme {
        Theme::CatppuccinMocha
    }

    pub fn view(&self, window: window::Id) -> Element<Message> {
        if window == self.main_window {
            let panel_items = row![
                self.app_tray.view().map(Message::AppTray),
                self.settings_tray.view().map(Message::SettingsTray)
            ]
            .padding(Padding {
                right: 16.0,
                left: 16.0,
                top: 0.0,
                bottom: 0.0,
            });
            iced::widget::container(column![
                iced::widget::horizontal_rule(1).style(|theme: &Theme| iced::widget::rule::Style {
                    color: theme.palette().primary,
                    width: 1,
                    radius: Radius::from(0),
                    fill_mode: iced::widget::rule::FillMode::Full
                }),
                panel_items
            ])
            .width(Length::Fill)
            .height(Length::Fill)
            .into()
        } else {
            text!("Hey").into()
        }
    }

    pub fn subscription(&self) -> Subscription<Message> {
        Subscription::batch(vec![
            self.settings_tray.subscription().map(Message::SettingsTray),
            self.app_tray.subscription().map(Message::AppTray),
        ])
    }
}

#[derive(Clone, Debug)]
pub enum Message {
    AppTray(AppTrayMessage),
    SettingsTray(SettingsTrayMessage),
    OpenPopup(window::Id, String),
    OpenMainWindow(window::Id),
}
