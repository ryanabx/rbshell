use std::collections::HashMap;

use cctk::toplevel_info::ToplevelInfo;
use cosmic_comp::CosmicCompBackend;
use cosmic_protocols::toplevel_info::v1::client::zcosmic_toplevel_handle_v1::ZcosmicToplevelHandleV1;
use iced::Subscription;

use super::{AppTrayMessage, ApplicationGroup};

pub mod cosmic_comp;

#[derive(Clone, Debug)]
pub enum CompositorBackend {
    Cosmic(CosmicCompBackend),
    None,
    #[allow(dead_code)]
    NotSupported,
}

impl CompositorBackend {
    pub fn new(compositor: &str) -> Self {
        match compositor {
            "COSMIC" => Self::Cosmic(CosmicCompBackend::new()),
            "none" => Self::None,
            desktop => panic!("Unsupported desktop {desktop}. Specify a backend with the env variable RYANABX_SHELL_DESKTOP"),
        }
    }

    pub fn wayland_subscription(&self) -> iced::Subscription<WaylandIncoming> {
        match self {
            Self::Cosmic(backend) => backend.wayland_subscription().map(WaylandIncoming::Cosmic),
            Self::None => Subscription::none(),
            Self::NotSupported => panic!("Not supported"),
        }
    }

    pub fn handle_message(
        &mut self,
        active_toplevels: &mut HashMap<String, ApplicationGroup>,
        incoming: WaylandIncoming,
    ) -> Option<iced::Command<AppTrayMessage>> {
        match (self, incoming) {
            (Self::Cosmic(backend), WaylandIncoming::Cosmic(evt)) => {
                backend.handle_incoming(active_toplevels, evt)
            }
            (Self::None, _) => None,
            (Self::NotSupported, _) | (_, WaylandIncoming::NotSupported) => panic!("Not supported"),
        }
    }

    pub fn handle_outgoing_message(
        &mut self,
        active_toplevels: &mut HashMap<String, ApplicationGroup>,
        outgoing: WaylandOutgoing,
    ) -> Option<iced::Command<AppTrayMessage>> {
        match self {
            Self::Cosmic(backend) => backend.handle_outgoing(active_toplevels, outgoing),
            Self::None => None,
            Self::NotSupported => panic!("Not supported"),
        }
    }

    pub fn active_window<'a>(
        &self,
        active_toplevels: &HashMap<String, ApplicationGroup>,
    ) -> Option<WindowHandle> {
        match self {
            Self::Cosmic(backend) => backend.active_window(active_toplevels),
            Self::None => None,
            Self::NotSupported => panic!("Not supported"),
        }
    }
}

#[derive(Clone, Debug)]
pub enum WaylandIncoming {
    Cosmic(cosmic_comp::CosmicIncoming),
    #[allow(dead_code)]
    NotSupported,
}

#[derive(Clone, Debug)]
pub enum WaylandOutgoing {
    Exec(String, String),
    Toggle(WindowHandle),
    #[allow(unused)]
    Activate(WindowHandle),
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum WindowHandle {
    Cosmic(ZcosmicToplevelHandleV1),
    #[allow(dead_code)]
    NotSupported,
}

#[derive(Clone, Debug)]
pub enum WindowInfo {
    Cosmic(ToplevelInfo),
    #[allow(dead_code)]
    NotSupported,
}
