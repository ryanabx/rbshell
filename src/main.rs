use app_tray::{AppTray, AppTrayConfig, AppTrayMessage};
use config::PanelConfig;
use iced::{
    application::{
        actions::layer_surface::SctkLayerSurfaceSettings, layer_surface::Anchor, InitialSurface,
    },
    widget::{column, container::Style},
    Application, Background, Color, Command, Radius, Settings, Theme,
};

mod app_tray;
mod config;
mod settings_tray;

fn main() -> Result<(), iced::Error> {
    let settings = SctkLayerSurfaceSettings {
        anchor: Anchor::BOTTOM.union(Anchor::LEFT).union(Anchor::RIGHT),
        size: Some((None, Some(48))),
        layer: iced::application::layer_surface::Layer::Top,
        exclusive_zone: 48,
        ..Default::default()
    };
    Panel::run(Settings {
        initial_surface: InitialSurface::LayerSurface(settings),
        ..Settings::default()
    })
}

#[derive(Clone, Debug)]
struct Panel<'a> {
    _panel_config: PanelConfig,
    app_tray: AppTray<'a>,
}

impl<'a> Default for Panel<'a> {
    fn default() -> Self {
        Self {
            _panel_config: PanelConfig::default(),
            app_tray: AppTray::new(AppTrayConfig::default()),
        }
    }
}

#[derive(Clone, Debug)]
pub enum Message {
    Panic,
    AppTray(AppTrayMessage),
}

impl<'a> Application for Panel<'a> {
    type Message = Message;
    type Theme = Theme;
    type Executor = iced::executor::Default;
    type Renderer = iced::Renderer;
    type Flags = ();

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
        }
    }

    fn view(
        &self,
        _id: iced::window::Id,
    ) -> iced::Element<'_, Self::Message, Self::Theme, Self::Renderer> {
        let panel_items = self.app_tray.view().map(Message::AppTray);
        iced::widget::container(column![
            iced::widget::horizontal_rule(1).style(|_| iced::widget::rule::Style {
                color: Color::WHITE,
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
        self.app_tray.subscription().map(Message::AppTray)
    }
}

impl<'a> Panel<'a> {
    fn panel_style(&self, _theme: &Theme) -> Style {
        Style {
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
