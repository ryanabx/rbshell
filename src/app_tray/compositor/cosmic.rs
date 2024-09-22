use std::collections::HashSet;

use cosmic_protocols::{
    toplevel_info::v1::client::{zcosmic_toplevel_handle_v1, zcosmic_toplevel_info_v1},
    toplevel_management::v1::client::zcosmic_toplevel_manager_v1,
};
use wayland_client::{Connection, Dispatch, Proxy, QueueHandle};

use super::{AppData, ToplevelHandle, ToplevelHandleEvent, ToplevelManagerEvent, ToplevelState};

impl From<zcosmic_toplevel_handle_v1::State> for ToplevelState {
    fn from(value: zcosmic_toplevel_handle_v1::State) -> Self {
        match value {
            zcosmic_toplevel_handle_v1::State::Maximized => Self::Maximized,
            zcosmic_toplevel_handle_v1::State::Minimized => Self::Minimized,
            zcosmic_toplevel_handle_v1::State::Activated => Self::Activated,
            zcosmic_toplevel_handle_v1::State::Fullscreen => Self::Fullscreen,
            _ => todo!(),
        }
    }
}

impl From<zcosmic_toplevel_handle_v1::Event> for ToplevelHandleEvent {
    fn from(value: zcosmic_toplevel_handle_v1::Event) -> Self {
        match value {
            zcosmic_toplevel_handle_v1::Event::Closed => ToplevelHandleEvent::Closed,
            zcosmic_toplevel_handle_v1::Event::Done => ToplevelHandleEvent::Done,
            zcosmic_toplevel_handle_v1::Event::Title { title } => {
                ToplevelHandleEvent::Title { title }
            }
            zcosmic_toplevel_handle_v1::Event::AppId { app_id } => {
                ToplevelHandleEvent::AppId { app_id }
            }
            zcosmic_toplevel_handle_v1::Event::OutputEnter { output } => {
                ToplevelHandleEvent::OutputEnter { output }
            }
            zcosmic_toplevel_handle_v1::Event::OutputLeave { output } => {
                ToplevelHandleEvent::OutputLeave { output }
            }
            zcosmic_toplevel_handle_v1::Event::WorkspaceEnter { .. } => todo!(),
            zcosmic_toplevel_handle_v1::Event::WorkspaceLeave { .. } => todo!(),
            zcosmic_toplevel_handle_v1::Event::State { state } => {
                let mut r_state = HashSet::new();
                for value in state.chunks_exact(4) {
                    if let Ok(state) = zcosmic_toplevel_handle_v1::State::try_from(
                        u32::from_ne_bytes(value[0..4].try_into().unwrap()),
                    ) {
                        r_state.insert(ToplevelState::from(state));
                    }
                }
                Self::State { state: r_state }
            }
            _ => todo!(),
        }
    }
}

impl From<zcosmic_toplevel_info_v1::Event> for ToplevelManagerEvent {
    fn from(value: zcosmic_toplevel_info_v1::Event) -> Self {
        match value {
            zcosmic_toplevel_info_v1::Event::Toplevel { toplevel } => {
                Self::Toplevel(ToplevelHandle::Zcosmic(toplevel))
            }
            zcosmic_toplevel_info_v1::Event::Finished => Self::Finished,
            _ => todo!(),
        }
    }
}

// COSMIC Foreign Toplevel Info

impl Dispatch<zcosmic_toplevel_handle_v1::ZcosmicToplevelHandleV1, ()> for AppData {
    fn event(
        state: &mut Self,
        toplevel: &zcosmic_toplevel_handle_v1::ZcosmicToplevelHandleV1,
        event: <zcosmic_toplevel_handle_v1::ZcosmicToplevelHandleV1 as Proxy>::Event,
        _data: &(),
        _conn: &Connection,
        _qhandle: &QueueHandle<Self>,
    ) {
        log::trace!("zcosmic_toplevel_handle_v1::event: {:?}", event);
        state.handle_toplevel_handle_event(
            ToplevelHandle::Zcosmic(toplevel.clone()),
            ToplevelHandleEvent::from(event),
        );
    }
}

impl Dispatch<zcosmic_toplevel_info_v1::ZcosmicToplevelInfoV1, ()> for AppData {
    fn event(
        state: &mut Self,
        _proxy: &zcosmic_toplevel_info_v1::ZcosmicToplevelInfoV1,
        event: <zcosmic_toplevel_info_v1::ZcosmicToplevelInfoV1 as Proxy>::Event,
        _data: &(),
        _conn: &Connection,
        _qhandle: &QueueHandle<Self>,
    ) {
        log::trace!("zcosmic_toplevel_info_v1::event: {:?}", event);
        state.handle_toplevel_manager_event(ToplevelManagerEvent::from(event));
    }

    wayland_client::event_created_child!(AppData, zcosmic_toplevel_info_v1::ZcosmicToplevelInfoV1, [
        zcosmic_toplevel_info_v1::EVT_TOPLEVEL_OPCODE => (zcosmic_toplevel_handle_v1::ZcosmicToplevelHandleV1, ())
    ]);
}

impl Dispatch<zcosmic_toplevel_manager_v1::ZcosmicToplevelManagerV1, ()> for AppData {
    fn event(
        state: &mut Self,
        proxy: &zcosmic_toplevel_manager_v1::ZcosmicToplevelManagerV1,
        event: <zcosmic_toplevel_manager_v1::ZcosmicToplevelManagerV1 as Proxy>::Event,
        data: &(),
        conn: &Connection,
        qhandle: &QueueHandle<Self>,
    ) {
        // println!("Event! {:?}", event);
    }
}
