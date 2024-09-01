use std::rc::Rc;

use crate::{component::Component, desktop_entry::DesktopEntryCache};

#[derive(Clone, Debug)]
pub enum StartMenuMessage {
    MenuOpened,
}

pub struct StartMenu<'a> {
    de_cache: Rc<DesktopEntryCache<'a>>,
}

impl<'a> StartMenu<'a> {
    pub fn new(de_cache: Rc<DesktopEntryCache<'a>>) -> Self {
        Self { de_cache }
    }
}

impl<'a> Component for StartMenu<'a> {
    type Message = StartMenuMessage;

    fn view(&self) -> iced::Element<Self::Message> {
        todo!()
    }

    fn handle_message(&mut self, message: Self::Message) -> iced::Task<Self::Message> {
        match message {
            StartMenuMessage::MenuOpened => todo!(),
        }
    }

    fn subscription(&self) -> iced::Subscription<Self::Message> {
        todo!()
    }
}
