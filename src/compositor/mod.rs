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

    pub fn wayland_subscription(&self) -> iced::Subscription<WaylandEvent> {
        match self {
            Self::Cosmic(backend) => backend.wayland_subscription().map(WaylandEvent::Cosmic),
            Self::NotSupported => unreachable!(),
        }
    }

    pub fn handle_message(
        &mut self,
        app_tray: &mut AppTray,
        event: WaylandEvent,
    ) -> Option<iced::Command<crate::Message>> {
        match (self, event) {
            (Self::Cosmic(backend), WaylandEvent::Cosmic(evt)) => {
                backend.handle_message(app_tray, evt)
            }
            (Self::NotSupported, _) => unreachable!(),
            (_, WaylandEvent::NotSupported) => unreachable!(),
        }
    }
}

#[derive(Clone, Debug)]
pub enum WaylandEvent {
    Cosmic(cosmic_comp::CosmicWaylandMessage),
    #[allow(dead_code)]
    NotSupported,
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
