use std::env;

use cctk::toplevel_info::ToplevelInfo;
use cosmic_comp::CosmicCompBackend;
use cosmic_protocols::toplevel_info::v1::client::zcosmic_toplevel_handle_v1::ZcosmicToplevelHandleV1;

use crate::app_tray::AppTray;

pub mod cosmic_comp;
pub mod wlr;

#[derive(Clone, Debug)]
pub enum CompositorBackend {
    Cosmic(CosmicCompBackend),
    #[allow(dead_code)]
    NotSupported,
}

impl CompositorBackend {
    pub fn new() -> Self {
        // set the environment variable RYANABX_SHELL_DESKTOP to set which desktop session should be inferred
        let current_compositor = match env::var("RYANABX_SHELL_DESKTOP") {
            Ok(val) => Ok(val),
            _ => env::var("XDG_CURRENT_DESKTOP"), // fall back on XDG_CURRENT_DESKTOP if not set
        };
        match current_compositor.as_deref() {
            Ok("COSMIC") => Self::Cosmic(CosmicCompBackend::new()),
            Ok(desktop) => panic!("Unsupported desktop {desktop}. Specify a backend with the env variable RYANABX_SHELL_DESKTOP"),
            _ => panic!("Unsupported desktop. Specify a backend with the env variable RYANABX_SHELL_DESKTOP"),
        }
    }

    pub fn wayland_subscription(&self) -> iced::Subscription<WaylandIncoming> {
        match self {
            Self::Cosmic(backend) => backend.wayland_subscription().map(WaylandIncoming::Cosmic),
            Self::NotSupported => todo!(),
        }
    }

    pub fn handle_message(
        &mut self,
        app_tray: &mut AppTray,
        incoming: WaylandIncoming,
    ) -> Option<iced::Command<crate::Message>> {
        match (self, incoming) {
            (Self::Cosmic(backend), WaylandIncoming::Cosmic(evt)) => {
                backend.handle_incoming(app_tray, evt)
            }
            (Self::NotSupported, _) => todo!(),
            (_, WaylandIncoming::NotSupported) => todo!(),
        }
    }

    pub fn handle_outgoing_message(
        &mut self,
        app_tray: &mut AppTray,
        outgoing: WaylandOutgoing,
    ) -> Option<iced::Command<crate::Message>> {
        match self {
            Self::Cosmic(backend) => backend.handle_outgoing(app_tray, outgoing),
            _ => todo!(),
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
