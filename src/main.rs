use iced::{
    application::{
        actions::{layer_surface::SctkLayerSurfaceSettings, window::SctkWindowSettings},
        layer_surface::Anchor,
        InitialSurface,
    }, widget::{
        column, container::{self, Style}, row, text
    }, Application, Background, Border, Color, Command, Element, Settings, Size, Theme
};

mod config;

fn main() -> Result<(), iced::Error> {
    println!("Hello, world!");
    let settings = SctkLayerSurfaceSettings {
        anchor: Anchor::BOTTOM,
        size: Some((1280.into(), 48.into())),
        layer: iced::application::layer_surface::Layer::Top,
        exclusive_zone: 48,
        ..Default::default()
    };
    Panel::run(Settings {
        initial_surface: InitialSurface::LayerSurface(settings),
        ..Settings::default()
    })
}

struct Panel {}

#[derive(Clone, Debug)]
struct Message {}

impl Application for Panel {
    type Message = Message;
    type Theme = Theme;
    type Executor = iced::executor::Default;
    type Renderer = iced::Renderer;
    type Flags = ();

    fn new(flags: Self::Flags) -> (Self, iced::Command<Self::Message>) {
        (Panel {}, Command::<self::Message>::none())
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
        return column![iced::widget::container(text!("Hi"))
            .style(|_| {
                Style {
                    background: Some(Background::Color(Color {
                        r: 18.8 / 256.0,
                        g: 18.8 / 256.0,
                        b: 18.8 / 256.0,
                        a: 1.0,
                    })),
                    border: Border {
                        color: Color { r: 1.0, g: 1.0, b: 1.0, a: 1.0 },
                        width: 1.0,
                        radius: 0.0.into(),
                    },
                    ..Default::default()
                }
            })
            .fill()]
        .into();
    }
}
