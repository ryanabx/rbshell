use iced::{
    widget::{column, row},
    Application, Background, Color, Command, Padding, Radius, Subscription, Theme,
};

use crate::{
    app_tray::{AppTray, AppTrayMessage},
    config::{AppTrayConfig, PanelConfig},
    settings_tray::{SettingsTray, SettingsTrayMessage},
    CliArgs,
};

#[derive(Clone, Debug)]
pub struct Panel<'a> {
    _panel_config: PanelConfig,
    app_tray: AppTray<'a>,
    settings_tray: SettingsTray,
}

impl<'a> Default for Panel<'a> {
    fn default() -> Self {
        Self {
            _panel_config: PanelConfig::default(),
            app_tray: AppTray::new(AppTrayConfig::default()),
            settings_tray: SettingsTray::new(),
        }
    }
}

#[derive(Clone, Debug)]
pub enum Message {
    Panic,
    AppTray(AppTrayMessage),
    SettingsTray(SettingsTrayMessage),
}

impl<'a> Application for Panel<'a> {
    type Message = Message;
    type Theme = Theme;
    type Executor = iced::executor::Default;
    type Renderer = iced::Renderer;
    type Flags = CliArgs;

    fn new(_flags: Self::Flags) -> (Self, iced::Command<Self::Message>) {
        (Panel::default(), Command::<self::Message>::none())
    }

    fn title(&self, _id: iced::window::Id) -> String {
        "Window".into()
    }

    fn update(&mut self, message: Self::Message) -> iced::Command<Self::Message> {
        match message {
            Message::Panic => {
                panic!("Panic button pressed hehe");
            }
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
            iced::widget::horizontal_rule(1).style(|_| iced::widget::rule::Style {
                color: Color {
                    r: 1.0,
                    g: 1.0,
                    b: 1.0,
                    a: 0.3,
                },
                width: 1,
                radius: Radius::from(0),
                fill_mode: iced::widget::rule::FillMode::Full
            }),
            panel_items
        ])
        .style(|theme| self.panel_style(theme))
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

impl<'a> Panel<'a> {
    fn panel_style(&self, _theme: &Theme) -> iced::widget::container::Style {
        iced::widget::container::Style {
            background: Some(Background::Color(Color {
                r: 30.0 / 256.0,
                g: 30.0 / 256.0,
                b: 30.0 / 256.0,
                a: 1.0,
            })),
            ..Default::default()
        }
    }
}
