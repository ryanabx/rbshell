use std::env;

use cosmic_comp::CosmicCompBackend;
use cosmic_protocols::toplevel_info::v1::client::zcosmic_toplevel_handle_v1::ZcosmicToplevelHandleV1;

use crate::app_tray::AppTray;

pub mod cosmic_comp;
pub mod wlr;

#[derive(Clone, Debug)]
pub enum CompositorBackend {
    Cosmic(CosmicCompBackend),
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
        }
    }
}

#[derive(Clone, Debug)]
pub enum WaylandEvent {
    Cosmic(cosmic_comp::CosmicWaylandMessage),
}

#[derive(Clone, Debug)]
pub enum WindowHandle {
    Cosmic(ZcosmicToplevelHandleV1),
}