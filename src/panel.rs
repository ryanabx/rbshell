use iced::{
    widget::{column, row},
    Application, Command, Padding, Radius, Subscription, Theme,
};

use crate::{
    app_tray::{AppTray, AppTrayMessage},
    config::PanelConfig,
    settings_tray::{SettingsTray, SettingsTrayMessage},
};

#[derive(Clone, Debug)]
pub struct PanelFlags {
    pub compositor: String,
    pub config: PanelConfig,
}

#[derive(Clone, Debug)]
pub struct Panel<'a> {
    app_tray: AppTray<'a>,
    settings_tray: SettingsTray,
}

impl<'a> Panel<'a> {
    pub fn new(flags: PanelFlags) -> Self {
        Self {
            app_tray: AppTray::new(flags.config.app_tray, &flags.compositor),
            settings_tray: SettingsTray::new(),
        }
    }
}

#[derive(Clone, Debug)]
pub enum Message {
    AppTray(AppTrayMessage),
    SettingsTray(SettingsTrayMessage),
}

impl<'a> Application for Panel<'a> {
    type Message = Message;
    type Theme = Theme;
    type Executor = iced::executor::Default;
    type Renderer = iced::Renderer;
    type Flags = PanelFlags;

    fn new(flags: Self::Flags) -> (Self, iced::Command<Self::Message>) {
        (Panel::new(flags), Command::<self::Message>::none())
    }

    fn theme(&self, _id: iced::window::Id) -> Self::Theme {
        Theme::Dark
    }

    fn title(&self, _id: iced::window::Id) -> String {
        "Window".into()
    }

    fn update(&mut self, message: Self::Message) -> iced::Command<Self::Message> {
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

    fn view(
        &self,
        _id: iced::window::Id,
    ) -> iced::Element<'_, Self::Message, Self::Theme, Self::Renderer> {
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
        .fill()
        .into()
    }

    fn subscription(&self) -> iced::Subscription<Self::Message> {
        Subscription::batch(vec![
            self.settings_tray.subscription().map(Message::SettingsTray),
            self.app_tray.subscription().map(Message::AppTray),
        ])
    }
}
