use iced::{
    border::Radius,
    widget::{column, row},
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
}

impl<'a> Panel<'a> {
    pub fn new(config: PanelConfig, compositor: Compositor) -> (Self, Task<Message>) {
        (
            Self {
                app_tray: AppTray::new(config.app_tray, compositor),
                settings_tray: SettingsTray::new(),
            },
            Task::none(),
        )
    }

    pub fn title(&self) -> String {
        "Window".into()
    }

    pub fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::AppTray(app_tray_msg) => self
                .app_tray
                .handle_message(app_tray_msg)
                .map(Message::AppTray),
            Message::SettingsTray(settings_tray_msg) => self
                .settings_tray
                .handle_message(settings_tray_msg)
                .map(Message::SettingsTray),
        }
    }

    pub fn theme(&self, _id: iced::window::Id) -> Theme {
        Theme::Dark
    }

    pub fn view(&self) -> Element<Message> {
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
}
