use std::{
    env,
    ffi::{OsStr, OsString},
};

pub mod cosmic_comp;
pub mod wlr;

#[derive(Clone, Debug)]
pub enum WaylandMessage {
    CosmicComp(cosmic_comp::CosmicWaylandMessage),
    Wlroots(),
}

pub(crate) fn wayland_subscription() -> iced::Subscription<WaylandMessage> {
    // set the environment variable RYANABX_SHELL_DESKTOP to set which desktop session should be inferred
    let current_compositor = match env::var("RYANABX_SHELL_DESKTOP") {
        Ok(val) => Ok(val),
        _ => env::var("XDG_CURRENT_DESKTOP"), // fall back on XDG_CURRENT_DESKTOP if not set
    };
    match current_compositor.as_deref() {
        Ok("COSMIC") => cosmic_comp::wayland_subscription().map(WaylandMessage::CosmicComp),
        _ => panic!("Unsupported desktop"),
    }
}
