use std::collections::HashMap;

use cctk::toplevel_info::ToplevelInfo;
use cosmic_comp::CosmicCompBackend;
use cosmic_protocols::toplevel_info::v1::client::zcosmic_toplevel_handle_v1::ZcosmicToplevelHandleV1;
use iced::{Subscription, Task};

use super::{AppTrayMessage, ApplicationGroup};

pub mod cosmic_comp;

#[derive(Clone, Debug)]
pub enum Compositor {
    Cosmic,
    None,
}

impl Compositor {
    pub fn new(tag: &str) -> Self {
        match tag.to_lowercase().as_str() {
            "cosmic" | "cosmic-comp" => Self::Cosmic,
            other => {
                log::warn!(
                    "Compositor or desktop {} not directly supported. Some features may not work.",
                    other
                );
                Self::None
            }
        }
    }
}

#[derive(Clone, Debug)]
pub enum CompositorBackend {
    Cosmic(CosmicCompBackend),
    None,
}

impl CompositorBackend {
    pub fn new(compositor: Compositor) -> Self {
        match compositor {
            Compositor::Cosmic => Self::Cosmic(CosmicCompBackend::new()),
            Compositor::None => Self::None,
        }
    }

    pub fn wayland_subscription(&self) -> iced::Subscription<WaylandIncoming> {
        match self {
            Self::Cosmic(backend) => backend.wayland_subscription().map(WaylandIncoming::Cosmic),
            Self::None => Subscription::none(),
        }
    }

    pub fn handle_message(
        &mut self,
        active_toplevels: &mut HashMap<String, ApplicationGroup>,
        incoming: WaylandIncoming,
    ) -> Option<Task<AppTrayMessage>> {
        match (self, incoming) {
            (Self::Cosmic(backend), WaylandIncoming::Cosmic(evt)) => {
                backend.handle_incoming(active_toplevels, evt)
            }
            (Self::None, _) => None,
        }
    }

    pub fn handle_outgoing_message(
        &mut self,
        active_toplevels: &mut HashMap<String, ApplicationGroup>,
        outgoing: WaylandOutgoing,
    ) -> Option<Task<AppTrayMessage>> {
        match self {
            Self::Cosmic(backend) => backend.handle_outgoing(active_toplevels, outgoing),
            Self::None => None,
        }
    }

    pub fn active_window<'a>(
        &self,
        active_toplevels: &HashMap<String, ApplicationGroup>,
    ) -> Option<WindowHandle> {
        match self {
            Self::Cosmic(backend) => backend.active_window(active_toplevels),
            Self::None => None,
        }
    }
}

#[derive(Clone, Debug)]
pub enum WaylandIncoming {
    Cosmic(cosmic_comp::CosmicIncoming),
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
