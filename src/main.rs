use config::PanelConfig;
use iced::{
    application::{
        actions::layer_surface::SctkLayerSurfaceSettings, layer_surface::Anchor, InitialSurface,
    },
    widget::{
        column,
        container::{self, Style},
        row, Row,
    },
    Application, Background, Border, Color, Command, Element, Length, Radius, Settings, Theme,
};

mod config;
mod desktop_entry;

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
    panel_config: PanelConfig<'a>,
}

impl<'a> Default for Panel<'a> {
    fn default() -> Self {
        Self {
            panel_config: PanelConfig::default(),
        }
    }
}

#[derive(Clone, Debug)]
pub enum Message {
    Panic,
}

impl<'a> Application for Panel<'a> {
    type Message = Message;
    type Theme = Theme;
    type Executor = iced::executor::Default;
    type Renderer = iced::Renderer;
    type Flags = ();

    fn new(flags: Self::Flags) -> (Self, iced::Command<Self::Message>) {
        (Panel::default(), Command::<self::Message>::none())
    }

    fn title(&self, id: iced::window::Id) -> String {
        "Window".into()
    }

    fn update(&mut self, message: Self::Message) -> iced::Command<Self::Message> {
        match message {
            Message::Panic => {
                panic!("Panic button pressed hehe");
            }
        }
        Command::none()
    }

    fn view(
        &self,
        id: iced::window::Id,
    ) -> iced::Element<'_, Self::Message, Self::Theme, Self::Renderer> {
        let favorites_images = self.panel_config.favorites.iter().filter_map(|e| {
            e.get_widget()
                .map(|x| Element::from(iced::widget::container(x).width(48).height(48).padding(2)))
        });
        let panel_items: Row<Self::Message, Self::Theme, Self::Renderer> =
            iced::widget::row(favorites_images);
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
}

impl<'a> Panel<'a> {
    fn panel_style(&self, _theme: &Theme) -> Style {
        Style {
            background: Some(Background::Color(Color {
                r: 24.0 / 256.0,
                g: 24.0 / 256.0,
                b: 24.8 / 256.0,
                a: 1.0,
            })),
            ..Default::default()
        }
    }
}
