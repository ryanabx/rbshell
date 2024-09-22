// KDE Window Management

use wayland_client::{Connection, Dispatch, Proxy, QueueHandle};
use wayland_protocols_plasma::plasma_window_management::client::{
    org_kde_plasma_window, org_kde_plasma_window_management,
};

use super::AppData;

impl Dispatch<org_kde_plasma_window::OrgKdePlasmaWindow, ()> for AppData {
    fn event(
        _state: &mut Self,
        _proxy: &org_kde_plasma_window::OrgKdePlasmaWindow,
        event: <org_kde_plasma_window::OrgKdePlasmaWindow as Proxy>::Event,
        _data: &(),
        _conn: &Connection,
        _qhandle: &QueueHandle<Self>,
    ) {
        println!("{:?}", event);
    }
}

impl Dispatch<org_kde_plasma_window_management::OrgKdePlasmaWindowManagement, ()> for AppData {
    fn event(
        _state: &mut Self,
        _proxy: &org_kde_plasma_window_management::OrgKdePlasmaWindowManagement,
        event: <org_kde_plasma_window_management::OrgKdePlasmaWindowManagement as Proxy>::Event,
        _data: &(),
        _conn: &Connection,
        _qhandle: &QueueHandle<Self>,
    ) {
        println!("{:?}", event);
    }
}
