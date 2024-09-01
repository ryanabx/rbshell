pub trait Component {
    type Message;

    fn view(&self) -> iced::Element<Self::Message>;

    fn handle_message(&mut self, message: Self::Message) -> iced::Task<Self::Message>;

    fn subscription(&self) -> iced::Subscription<Self::Message>;
}
