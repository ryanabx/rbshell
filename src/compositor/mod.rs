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
    match env::var("XDG_CURRENT_DESKTOP").as_deref() {
        Ok("COSMIC") => cosmic_comp::wayland_subscription().map(WaylandMessage::CosmicComp),
        _ => panic!("Unsupported desktop"),
    }
}
