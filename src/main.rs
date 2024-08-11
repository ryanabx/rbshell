use config::PanelConfig;
use iced::{
    application::{
        actions::layer_surface::SctkLayerSurfaceSettings,
        layer_surface::Anchor,
        InitialSurface,
    },
    widget::{
        container::{self, Style},
        row, Row,
    },
    Application, Background, Border, Color, Command, Element, Settings, Theme,
};

mod config;
mod desktop_entry;

fn main() -> Result<(), iced::Error> {
    let settings = SctkLayerSurfaceSettings {
        anchor: Anchor::BOTTOM,
        size: Some((500.into(), 48.into())),
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
pub struct Message {}

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
        Command::none()
    }

    fn view(
        &self,
        id: iced::window::Id,
    ) -> iced::Element<'_, Self::Message, Self::Theme, Self::Renderer> {
        let favorites_images = self
            .panel_config
            .favorites
            .iter()
            .filter_map(|e| e.get_widget().map(|x| Element::new(x)));
        let panel_items: Row<Self::Message, Self::Theme, Self::Renderer> =
            iced::widget::row(favorites_images);
        iced::widget::container(panel_items)
            .style(|theme| self.panel_style(theme))
            .fill()
            .into()
    }
}

impl<'a> Panel<'a> {
    fn panel_style(&self, _theme: &Theme) -> Style {
        Style {
            background: Some(Background::Color(Color {
                r: 18.8 / 256.0,
                g: 18.8 / 256.0,
                b: 18.8 / 256.0,
                a: 1.0,
            })),
            border: Border {
                color: Color {
                    r: 1.0,
                    g: 1.0,
                    b: 1.0,
                    a: 1.0,
                },
                width: 1.0,
                radius: 0.0.into(),
            },
            ..Default::default()
        }
    }
}
