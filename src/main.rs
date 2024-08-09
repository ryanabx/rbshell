use iced::{
    application::{actions::window::SctkWindowSettings, InitialSurface},
    widget::{text, row},
    Application, Command, Element, Settings, Theme,
};

fn main() -> Result<(), iced::Error> {
    println!("Hello, world!");
    let mut settings = SctkWindowSettings::default();
    // settings.size_limits = Limits::NONE.min_height(300.0).min_width(600.0);
    Panel::run(Settings {
        initial_surface: InitialSurface::XdgWindow(settings),
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
        return row![text!("Hi")].into();
    }
}
