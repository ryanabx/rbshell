use std::{
    collections::HashMap,
    os::{
        fd::{FromRawFd, RawFd},
        unix::net::UnixStream,
    },
};

use cctk::{
    sctk::{
        activation::{ActivationHandler, ActivationState, RequestData, RequestDataExt},
        output::{OutputHandler, OutputInfo, OutputState},
        reexports::{
            calloop::{
                channel::{self, Channel, Sender},
                EventLoop,
            },
            calloop_wayland_source::WaylandSource,
        },
        registry::{ProvidesRegistryState, RegistryState},
        seat::{SeatHandler, SeatState},
    },
    toplevel_info::{ToplevelInfo, ToplevelInfoHandler, ToplevelInfoState},
    toplevel_management::{ToplevelManagerHandler, ToplevelManagerState},
    wayland_client::{
        globals::registry_queue_init, protocol::wl_output::WlOutput, Connection, QueueHandle, WEnum,
    },
    workspace::{WorkspaceHandler, WorkspaceState},
};
use cosmic_protocols::{
    toplevel_info::v1::client::zcosmic_toplevel_handle_v1::ZcosmicToplevelHandleV1,
    workspace::v1::client::zcosmic_workspace_handle_v1::State as WorkspaceUpdateState,
    workspace::v1::client::zcosmic_workspace_handle_v1::ZcosmicWorkspaceHandleV1,
};
use iced::{
    futures::{
        self,
        channel::mpsc::{UnboundedReceiver, UnboundedSender},
        lock::Mutex,
        SinkExt, StreamExt,
    },
    subscription,
};
use once_cell::sync::Lazy;

use crate::{
    app_tray::{self, AppTray},
    compositor::{WindowHandle, WindowInfo},
};

use super::WaylandOutgoing;

struct WaylandData {
    _conn: Connection,
    queue_handle: QueueHandle<Self>,
    output_state: OutputState,
    workspace_state: WorkspaceState,
    toplevel_info_state: ToplevelInfoState,
    toplevel_manager_state: ToplevelManagerState,
    activation_state: Option<ActivationState>,
    registry_state: RegistryState,
    seat_state: SeatState,
    tx: UnboundedSender<CosmicIncoming>,
    exit: bool,
}

impl OutputHandler for WaylandData {
    fn output_state(&mut self) -> &mut OutputState {
        &mut self.output_state
    }

    fn new_output(
        &mut self,
        _conn: &Connection,
        _qh: &QueueHandle<Self>,
        output: cctk::wayland_client::protocol::wl_output::WlOutput,
    ) {
        if let Some(info) = self.output_state.info(&output) {
            let _ = self
                .tx
                .unbounded_send(CosmicIncoming::Output(OutputUpdate::Add(
                    output.clone(),
                    info.clone(),
                )));
        }
    }

    fn update_output(
        &mut self,
        _conn: &Connection,
        _qh: &QueueHandle<Self>,
        output: cctk::wayland_client::protocol::wl_output::WlOutput,
    ) {
        if let Some(info) = self.output_state.info(&output) {
            let _ = self
                .tx
                .unbounded_send(CosmicIncoming::Output(OutputUpdate::Update(
                    output.clone(),
                    info.clone(),
                )));
        }
    }

    fn output_destroyed(
        &mut self,
        _conn: &Connection,
        _qh: &QueueHandle<Self>,
        output: cctk::wayland_client::protocol::wl_output::WlOutput,
    ) {
        let _ = self
            .tx
            .unbounded_send(CosmicIncoming::Output(OutputUpdate::Remove(output.clone())));
    }
}

impl WorkspaceHandler for WaylandData {
    fn workspace_state(&mut self) -> &mut WorkspaceState {
        &mut self.workspace_state
    }

    fn done(&mut self) {
        let active_workspaces = self
            .workspace_state
            .workspace_groups()
            .iter()
            .filter_map(|x| {
                x.workspaces.iter().find(|w| {
                    w.state
                        .contains(&WEnum::Value(WorkspaceUpdateState::Active))
                })
            })
            .map(|workspace| workspace.handle.clone())
            .collect::<Vec<_>>();
        let _ = self
            .tx
            .unbounded_send(CosmicIncoming::Workspace(active_workspaces.clone()));
    }
}

impl ProvidesRegistryState for WaylandData {
    fn registry(&mut self) -> &mut cctk::sctk::registry::RegistryState {
        &mut self.registry_state
    }

    cctk::sctk::registry_handlers!();
}

struct ExecRequestData {
    data: RequestData,
    exec: String,
    gpu_idx: Option<usize>,
}

impl RequestDataExt for ExecRequestData {
    fn app_id(&self) -> Option<&str> {
        self.data.app_id()
    }

    fn seat_and_serial(&self) -> Option<(&cctk::wayland_client::protocol::wl_seat::WlSeat, u32)> {
        self.data.seat_and_serial()
    }

    fn surface(&self) -> Option<&cctk::wayland_client::protocol::wl_surface::WlSurface> {
        self.data.surface()
    }
}

impl ActivationHandler for WaylandData {
    type RequestData = ExecRequestData;

    fn new_token(&mut self, token: String, data: &Self::RequestData) {
        let _ = self.tx.unbounded_send(CosmicIncoming::ActivationToken {
            token: Some(token),
            app_id: data.app_id().map(|x| x.to_owned()),
            exec: data.exec.clone(),
            gpu_idx: data.gpu_idx,
        });
    }
}

impl SeatHandler for WaylandData {
    fn seat_state(&mut self) -> &mut cctk::sctk::seat::SeatState {
        &mut self.seat_state
    }

    fn new_seat(
        &mut self,
        _conn: &Connection,
        _qh: &QueueHandle<Self>,
        _seat: cctk::wayland_client::protocol::wl_seat::WlSeat,
    ) {
        // Intentionally empty for now
    }

    fn new_capability(
        &mut self,
        _conn: &Connection,
        _qh: &QueueHandle<Self>,
        _seat: cctk::wayland_client::protocol::wl_seat::WlSeat,
        _capability: cctk::sctk::seat::Capability,
    ) {
        // Intentionally empty for now
    }

    fn remove_capability(
        &mut self,
        _conn: &Connection,
        _qh: &QueueHandle<Self>,
        _seat: cctk::wayland_client::protocol::wl_seat::WlSeat,
        _capability: cctk::sctk::seat::Capability,
    ) {
        // Intentionally empty for now
    }

    fn remove_seat(
        &mut self,
        _conn: &Connection,
        _qh: &QueueHandle<Self>,
        _seat: cctk::wayland_client::protocol::wl_seat::WlSeat,
    ) {
        // Intentionally empty for now
    }
}

impl ToplevelManagerHandler for WaylandData {
    fn toplevel_manager_state(&mut self) -> &mut ToplevelManagerState {
        &mut self.toplevel_manager_state
    }

    fn capabilities(
        &mut self,
        _conn: &Connection,
        _qh: &QueueHandle<Self>,
        _capabilities: Vec<
            cctk::wayland_client::WEnum<cosmic_protocols::toplevel_management::v1::client::zcosmic_toplevel_manager_v1::ZcosmicToplelevelManagementCapabilitiesV1>,
        >,
    ) {
        // Intentionally empty for now
    }
}

impl ToplevelInfoHandler for WaylandData {
    fn toplevel_info_state(&mut self) -> &mut ToplevelInfoState {
        &mut self.toplevel_info_state
    }

    fn new_toplevel(
        &mut self,
        _conn: &Connection,
        _qh: &QueueHandle<Self>,
        toplevel: &cosmic_protocols::toplevel_info::v1::client::zcosmic_toplevel_handle_v1::ZcosmicToplevelHandleV1,
    ) {
        if let Some(info) = self.toplevel_info_state.info(toplevel) {
            let _ = self
                .tx
                .unbounded_send(CosmicIncoming::Toplevel(ToplevelUpdate::Add(
                    toplevel.clone(),
                    info.clone(),
                )));
        } else {
            panic!("Not sure how this would happen. cosmic-client-toolkit did something wrong.");
        }
    }

    fn update_toplevel(
        &mut self,
        _conn: &Connection,
        _qh: &QueueHandle<Self>,
        toplevel: &cosmic_protocols::toplevel_info::v1::client::zcosmic_toplevel_handle_v1::ZcosmicToplevelHandleV1,
    ) {
        if let Some(info) = self.toplevel_info_state.info(toplevel) {
            let _ = self
                .tx
                .unbounded_send(CosmicIncoming::Toplevel(ToplevelUpdate::Update(
                    toplevel.clone(),
                    info.clone(),
                )));
        }
    }

    fn toplevel_closed(
        &mut self,
        _conn: &Connection,
        _qh: &QueueHandle<Self>,
        toplevel: &cosmic_protocols::toplevel_info::v1::client::zcosmic_toplevel_handle_v1::ZcosmicToplevelHandleV1,
    ) {
        let _ = self
            .tx
            .unbounded_send(CosmicIncoming::Toplevel(ToplevelUpdate::Remove(
                toplevel.clone(),
            )));
    }
}

fn wayland_handler(tx: UnboundedSender<CosmicIncoming>, rx: Channel<WaylandRequest>) {
    let socket = std::env::var("X_PRIVILEGED_WAYLAND_SOCKET")
        .ok()
        .and_then(|fd| {
            fd.parse::<RawFd>()
                .ok()
                .map(|fd| unsafe { UnixStream::from_raw_fd(fd) })
        });

    let conn = if let Some(socket) = socket {
        Connection::from_socket(socket).unwrap()
    } else {
        Connection::connect_to_env().unwrap()
    };

    let (globals, event_queue) = registry_queue_init(&conn).unwrap();

    let mut event_loop = EventLoop::<WaylandData>::try_new().unwrap();
    let qh = event_queue.handle();
    let wayland_source = WaylandSource::new(conn.clone(), event_queue);
    let handle = event_loop.handle();
    wayland_source
        .insert(handle.clone())
        .expect("Failed to insert wayland source.");
    if handle
        .insert_source(rx, |event, _, state| match event {
            channel::Event::Msg(req) => match req {
                WaylandRequest::Toplevel(req) => match req {
                    ToplevelRequest::Activate(handle) => {
                        if let Some(seat) = state.seat_state.seats().next() {
                            let manager = &state.toplevel_manager_state.manager;
                            manager.activate(&handle, &seat);
                        }
                    }
                    ToplevelRequest::Minimize(handle) => {
                        let manager = &state.toplevel_manager_state.manager;
                        manager.set_minimized(&handle);
                    }
                    ToplevelRequest::Quit(handle) => {
                        let manager = &state.toplevel_manager_state.manager;
                        manager.close(&handle);
                    }
                },
                WaylandRequest::TokenRequest {
                    app_id,
                    exec,
                    gpu_idx,
                } => {
                    if let Some(activation_state) = state.activation_state.as_ref() {
                        activation_state.request_token_with_data(
                            &state.queue_handle,
                            ExecRequestData {
                                data: RequestData {
                                    app_id: Some(app_id),
                                    seat_and_serial: state
                                        .seat_state
                                        .seats()
                                        .next()
                                        .map(|seat| (seat, 0)),
                                    surface: None,
                                },
                                exec,
                                gpu_idx,
                            },
                        );
                    } else {
                        let _ = state.tx.unbounded_send(CosmicIncoming::ActivationToken {
                            token: None,
                            app_id: Some(app_id),
                            exec,
                            gpu_idx,
                        });
                    }
                }
            },
            channel::Event::Closed => {
                state.exit = true;
            }
        })
        .is_err()
    {
        return;
    }
    let registry_state = RegistryState::new(&globals);

    let mut app_data = WaylandData {
        exit: false,
        tx,
        _conn: conn,
        queue_handle: qh.clone(),
        output_state: OutputState::new(&globals, &qh),
        workspace_state: WorkspaceState::new(&registry_state, &qh),
        toplevel_info_state: ToplevelInfoState::new(&registry_state, &qh),
        toplevel_manager_state: ToplevelManagerState::new(&registry_state, &qh),
        registry_state,
        seat_state: SeatState::new(&globals, &qh),
        activation_state: ActivationState::bind::<WaylandData>(&globals, &qh).ok(),
    };

    loop {
        if app_data.exit {
            println!("Exiting...");
            break;
        }
        event_loop.dispatch(None, &mut app_data).unwrap();
    }
}

#[derive(Clone, Debug)]
pub enum CosmicIncoming {
    Init(channel::Sender<WaylandRequest>),
    Finished,
    Toplevel(ToplevelUpdate),
    Workspace(Vec<ZcosmicWorkspaceHandleV1>),
    Output(OutputUpdate),
    ActivationToken {
        token: Option<String>,
        app_id: Option<String>,
        exec: String,
        gpu_idx: Option<usize>,
    },
}

#[derive(Clone, Debug)]
pub enum ToplevelUpdate {
    Add(ZcosmicToplevelHandleV1, ToplevelInfo),
    Update(ZcosmicToplevelHandleV1, ToplevelInfo),
    Remove(ZcosmicToplevelHandleV1),
}

#[derive(Clone, Debug)]
pub enum OutputUpdate {
    Add(WlOutput, OutputInfo),
    Update(WlOutput, OutputInfo),
    Remove(WlOutput),
}

#[derive(Clone, Debug)]
pub enum WaylandRequest {
    Toplevel(ToplevelRequest),
    TokenRequest {
        app_id: String,
        exec: String,
        gpu_idx: Option<usize>,
    },
}

#[derive(Debug, Clone)]
pub enum ToplevelRequest {
    Activate(ZcosmicToplevelHandleV1),
    Minimize(ZcosmicToplevelHandleV1),
    Quit(ZcosmicToplevelHandleV1),
}

cctk::sctk::delegate_seat!(WaylandData);
cctk::sctk::delegate_registry!(WaylandData);
cctk::delegate_toplevel_info!(WaylandData);
cctk::delegate_workspace!(WaylandData);
cctk::delegate_toplevel_manager!(WaylandData);

cctk::sctk::delegate_activation!(WaylandData, ExecRequestData);

cctk::sctk::delegate_output!(WaylandData);

// Wayland Subscription

pub enum State {
    Waiting,
    Finished,
}

pub static WAYLAND_RX: Lazy<Mutex<Option<UnboundedReceiver<CosmicIncoming>>>> =
    Lazy::new(|| Mutex::new(None));

async fn start_listening(
    state: State,
    output: &mut futures::channel::mpsc::Sender<CosmicIncoming>,
) -> State {
    match state {
        State::Waiting => {
            let mut guard = WAYLAND_RX.lock().await;
            let rx = {
                if guard.is_none() {
                    let (calloop_tx, calloop_rx) = channel::channel();
                    let (toplevel_tx, toplevel_rx) = iced::futures::channel::mpsc::unbounded();
                    let _ = std::thread::spawn(move || {
                        wayland_handler(toplevel_tx, calloop_rx);
                    });
                    *guard = Some(toplevel_rx);
                    _ = output.send(CosmicIncoming::Init(calloop_tx)).await;
                }
                guard.as_mut().unwrap()
            };
            match rx.next().await {
                Some(u) => {
                    _ = output.send(u).await;
                    State::Waiting
                }
                None => {
                    _ = output.send(CosmicIncoming::Finished).await;
                    State::Finished
                }
            }
        }
        State::Finished => iced::futures::future::pending().await,
    }
}

#[derive(Debug, Clone)]
pub struct CosmicCompBackend {
    wayland_sender: Option<Sender<WaylandRequest>>,
    active_workspaces: Vec<ZcosmicWorkspaceHandleV1>,
    output_list: HashMap<WlOutput, OutputInfo>,
    current_output: String,
}

impl CosmicCompBackend {
    pub fn new() -> Self {
        Self {
            wayland_sender: None,
            active_workspaces: Vec::new(),
            output_list: HashMap::new(),
            current_output: "".to_string(),
        }
    }

    pub fn wayland_subscription(&self) -> iced::Subscription<CosmicIncoming> {
        subscription::channel(
            std::any::TypeId::of::<CosmicIncoming>(),
            50,
            move |mut output| async move {
                let mut state = State::Waiting;

                loop {
                    state = start_listening(state, &mut output).await;
                }
            },
        )
    }

    pub fn handle_incoming(
        &mut self,
        app_tray: &mut crate::app_tray::AppTray,
        incoming: CosmicIncoming,
    ) -> Option<iced::Command<crate::Message>> {
        match incoming {
            CosmicIncoming::Init(wayland_sender) => {
                self.wayland_sender.replace(wayland_sender);
                None
            }
            CosmicIncoming::Finished => None,
            CosmicIncoming::Toplevel(toplevel_update) => match toplevel_update {
                ToplevelUpdate::Add(handle, info) => {
                    let app_id = info.app_id.clone();
                    if app_tray.active_toplevels.contains_key(&app_id) {
                        app_tray
                            .active_toplevels
                            .get_mut(&info.app_id)
                            .unwrap()
                            .toplevels
                            .insert(WindowHandle::Cosmic(handle), WindowInfo::Cosmic(info));
                    } else {
                        app_tray.active_toplevels.insert(
                            app_id.clone(),
                            crate::app_tray::ApplicationGroup {
                                toplevels: HashMap::from([(
                                    WindowHandle::Cosmic(handle),
                                    WindowInfo::Cosmic(info.clone()),
                                )]),
                            },
                        );
                    }
                    None
                }
                ToplevelUpdate::Update(handle, info) => {
                    // TODO probably want to make sure it is removed
                    if info.app_id.is_empty() {
                        return Some(iced::Command::none());
                    } else if !app_tray.active_toplevels.contains_key(&info.app_id) {
                        return Some(iced::Command::none());
                    }

                    for (t_handle, t_info) in &mut app_tray
                        .active_toplevels
                        .get_mut(&info.app_id)
                        .unwrap()
                        .toplevels
                    {
                        if let WindowHandle::Cosmic(c_handle) = t_handle {
                            if &handle == c_handle {
                                *t_info = WindowInfo::Cosmic(info);
                                break;
                            }
                        }
                    }

                    None
                }
                ToplevelUpdate::Remove(handle) => {
                    let mut target_app_id: Option<String> = None;
                    for (app_id, app_info) in app_tray.active_toplevels.iter_mut() {
                        if app_info
                            .toplevels
                            .contains_key(&WindowHandle::Cosmic(handle.clone()))
                        {
                            app_info.toplevels.remove(&WindowHandle::Cosmic(handle));
                            if app_info.toplevels.is_empty() {
                                target_app_id = Some(app_id.clone());
                            }
                            break;
                        }
                    }
                    if let Some(app_id) = target_app_id {
                        app_tray.active_toplevels.remove(&app_id);
                    }
                    None
                }
            },
            CosmicIncoming::Workspace(workspaces) => {
                self.active_workspaces = workspaces;
                None
            }
            CosmicIncoming::Output(output_update) => match output_update {
                OutputUpdate::Add(output, info) => {
                    self.output_list.insert(output, info);
                    None
                }
                OutputUpdate::Update(output, info) => {
                    self.output_list.insert(output, info);
                    None
                }
                OutputUpdate::Remove(output) => {
                    self.output_list.remove(&output);
                    None
                }
            },
            _ => None,
        }
    }

    pub fn handle_outgoing(
        &mut self,
        app_tray: &mut crate::app_tray::AppTray,
        outgoing: WaylandOutgoing,
    ) -> Option<iced::Command<crate::Message>> {
        match outgoing {
            WaylandOutgoing::Exec(app_id, exec) => {
                println!("Sending a tokenrequest {} {}", &app_id, &exec);
                if let Some(tx) = self.wayland_sender.as_ref() {
                    let _ = tx.send(WaylandRequest::TokenRequest {
                        app_id,
                        exec,
                        gpu_idx: None,
                    });
                }
                None
            }
            WaylandOutgoing::Toggle(window) => {
                match window {
                    WindowHandle::Cosmic(toplevel) => {
                        if let Some(tx) = self.wayland_sender.as_ref() {
                            let _ = tx.send(WaylandRequest::Toplevel(
                                if self
                                    .active_window(app_tray)
                                    .is_some_and(|x| x == WindowHandle::Cosmic(toplevel.clone()))
                                {
                                    ToplevelRequest::Minimize(toplevel)
                                } else {
                                    ToplevelRequest::Activate(toplevel)
                                },
                            ));
                        }
                        // if let Some(p) = self.popup.take() {
                        //     return destroy_popup(p.id);
                        // }
                        None
                    }
                    _ => panic!(),
                }
            }
            WaylandOutgoing::Activate(window) => {
                match window {
                    WindowHandle::Cosmic(toplevel) => {
                        if let Some(tx) = self.wayland_sender.as_ref() {
                            let _ = tx.send(WaylandRequest::Toplevel(ToplevelRequest::Activate(
                                toplevel,
                            )));
                        }
                        // if let Some(p) = self.popup.take() {
                        //     return destroy_popup(p.id);
                        // }
                        None
                    }
                    _ => panic!(),
                }
            }
        }
    }

    pub fn active_window(&self, app_tray: &AppTray) -> Option<WindowHandle> {
        if self.active_workspaces.is_empty() {
            return None;
        }
        let mut focused_toplevels: Vec<ZcosmicToplevelHandleV1> = Vec::new();
        let active_workspaces = self.active_workspaces.clone();
        for (_, app_group) in app_tray.active_toplevels.iter() {
            for (handle, info) in &app_group.toplevels {
                if let (WindowHandle::Cosmic(t_handle), WindowInfo::Cosmic(t_info)) = (handle, info)
                {
                    if t_info.state.contains(&cosmic_protocols::toplevel_info::v1::client::zcosmic_toplevel_handle_v1::State::Activated)
                        && active_workspaces
                            .iter()
                            .any(|workspace| t_info.workspace.contains(workspace))
                        && t_info.output.iter().any(|x| {
                            self.output_list.get(x).is_some_and(|val| {
                                true // TODO: Output stuff
                                // val.name.as_ref().is_some_and(|n| *n == self.current_output)
                            })
                        })
                    {
                        focused_toplevels.push(t_handle.clone());
                    }
                } else {
                    panic!("Unknown window handle or window info");
                }
            }
        }
        focused_toplevels
            .first()
            .map(|f| WindowHandle::Cosmic(f.clone()))
    }
}
