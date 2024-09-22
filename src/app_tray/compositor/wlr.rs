use std::collections::HashSet;

use wayland_client::{event_created_child, Connection, Dispatch, Proxy, QueueHandle};
use wayland_protocols_wlr::foreign_toplevel::v1::client::{
    zwlr_foreign_toplevel_handle_v1, zwlr_foreign_toplevel_manager_v1,
};

use super::{AppData, ToplevelHandle, ToplevelHandleEvent, ToplevelManagerEvent, ToplevelState};

impl From<zwlr_foreign_toplevel_handle_v1::State> for ToplevelState {
    fn from(value: zwlr_foreign_toplevel_handle_v1::State) -> Self {
        match value {
            zwlr_foreign_toplevel_handle_v1::State::Maximized => Self::Maximized,
            zwlr_foreign_toplevel_handle_v1::State::Minimized => Self::Minimized,
            zwlr_foreign_toplevel_handle_v1::State::Activated => Self::Activated,
            zwlr_foreign_toplevel_handle_v1::State::Fullscreen => Self::Fullscreen,
            _ => todo!(),
        }
    }
}

impl From<zwlr_foreign_toplevel_handle_v1::Event> for ToplevelHandleEvent {
    fn from(value: zwlr_foreign_toplevel_handle_v1::Event) -> Self {
        match value {
            zwlr_foreign_toplevel_handle_v1::Event::Title { title } => Self::Title { title },
            zwlr_foreign_toplevel_handle_v1::Event::AppId { app_id } => Self::AppId { app_id },
            zwlr_foreign_toplevel_handle_v1::Event::OutputEnter { output } => {
                Self::OutputEnter { output }
            }
            zwlr_foreign_toplevel_handle_v1::Event::OutputLeave { output } => {
                Self::OutputLeave { output }
            }
            zwlr_foreign_toplevel_handle_v1::Event::State { state } => {
                let mut r_state = HashSet::new();
                for value in state.chunks_exact(4) {
                    if let Ok(state) = zwlr_foreign_toplevel_handle_v1::State::try_from(
                        u32::from_ne_bytes(value[0..4].try_into().unwrap()),
                    ) {
                        r_state.insert(ToplevelState::from(state));
                    }
                }
                Self::State { state: r_state }
            }
            zwlr_foreign_toplevel_handle_v1::Event::Done => Self::Done,
            zwlr_foreign_toplevel_handle_v1::Event::Closed => Self::Closed,
            zwlr_foreign_toplevel_handle_v1::Event::Parent { .. } => Self::None, // TODO: Not implemented
            _ => todo!(),
        }
    }
}

impl From<zwlr_foreign_toplevel_manager_v1::Event> for ToplevelManagerEvent {
    fn from(value: zwlr_foreign_toplevel_manager_v1::Event) -> Self {
        match value {
            zwlr_foreign_toplevel_manager_v1::Event::Toplevel { toplevel } => {
                Self::Toplevel(ToplevelHandle::Zwlr(toplevel))
            }
            zwlr_foreign_toplevel_manager_v1::Event::Finished => Self::Finished,
            _ => todo!(),
        }
    }
}

// WLR Foreign Toplevel Management

impl Dispatch<zwlr_foreign_toplevel_handle_v1::ZwlrForeignToplevelHandleV1, ()> for AppData {
    fn event(
        state: &mut Self,
        toplevel: &zwlr_foreign_toplevel_handle_v1::ZwlrForeignToplevelHandleV1,
        event: <zwlr_foreign_toplevel_handle_v1::ZwlrForeignToplevelHandleV1 as Proxy>::Event,
        _data: &(),
        _conn: &Connection,
        _qhandle: &QueueHandle<Self>,
    ) {
        state.handle_toplevel_handle_event(
            ToplevelHandle::Zwlr(toplevel.clone()),
            ToplevelHandleEvent::from(event),
        );
    }
}

impl Dispatch<zwlr_foreign_toplevel_manager_v1::ZwlrForeignToplevelManagerV1, ()> for AppData {
    fn event(
        state: &mut Self,
        _proxy: &wayland_protocols_wlr::foreign_toplevel::v1::client::zwlr_foreign_toplevel_manager_v1::ZwlrForeignToplevelManagerV1,
        event: wayland_protocols_wlr::foreign_toplevel::v1::client::zwlr_foreign_toplevel_manager_v1::Event,
        _data: &(),
        _conn: &Connection,
        _qhandle: &QueueHandle<Self>,
    ) {
        // println!("Toplevel event {:?}", event);
        state.handle_toplevel_manager_event(ToplevelManagerEvent::from(event));
    }

    event_created_child!(
        AppData, zwlr_foreign_toplevel_manager_v1::ZwlrForeignToplevelManagerV1,
        [
            zwlr_foreign_toplevel_manager_v1::EVT_TOPLEVEL_OPCODE => (zwlr_foreign_toplevel_handle_v1::ZwlrForeignToplevelHandleV1, ())
        ]
    );
}
