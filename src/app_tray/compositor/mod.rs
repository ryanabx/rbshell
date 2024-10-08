use std::collections::{HashMap, HashSet};

use cosmic_protocols::{
    toplevel_info::v1::client::{
        zcosmic_toplevel_handle_v1::ZcosmicToplevelHandleV1, zcosmic_toplevel_info_v1,
    },
    toplevel_management::v1::client::zcosmic_toplevel_manager_v1,
    workspace::v1::client::zcosmic_workspace_handle_v1,
};
use iced::{
    futures::{
        self,
        channel::mpsc::{UnboundedReceiver, UnboundedSender},
        lock::Mutex,
        SinkExt, StreamExt,
    },
    Subscription, Task,
};

use once_cell::sync::Lazy;
use smithay_client_toolkit::{
    output::{OutputHandler, OutputInfo, OutputState},
    reexports::{
        calloop::{
            channel::{self, Channel, Sender},
            EventLoop,
        },
        calloop_wayland_source::WaylandSource,
    },
    seat::{SeatHandler, SeatState},
};
use wayland_client::{
    globals::{registry_queue_init, GlobalListContents},
    protocol::{
        wl_output::{self, WlOutput},
        wl_registry::{self},
    },
    Connection, QueueHandle,
};
use wayland_protocols_plasma::plasma_window_management::client::org_kde_plasma_window_management;
use wayland_protocols_wlr::foreign_toplevel::v1::client::{
    zwlr_foreign_toplevel_handle_v1::ZwlrForeignToplevelHandleV1, zwlr_foreign_toplevel_manager_v1,
};

use crate::app_tray::AppTrayMessage;

pub mod cosmic;
pub mod kde;
pub mod wlr;

struct AppData {
    exit: bool,
    tx: UnboundedSender<WaylandIncoming>,
    output_state: OutputState,
    seat_state: SeatState,
    toplevel_state: ToplevelManager,
    zwlr_toplevel_manager: Option<zwlr_foreign_toplevel_manager_v1::ZwlrForeignToplevelManagerV1>,
    zcosmic_toplevel_info: Option<zcosmic_toplevel_info_v1::ZcosmicToplevelInfoV1>,
    zcosmic_toplevel_manager: Option<zcosmic_toplevel_manager_v1::ZcosmicToplevelManagerV1>,
    kde_window_manager: Option<org_kde_plasma_window_management::OrgKdePlasmaWindowManagement>,
}

impl AppData {
    fn handle_toplevel_handle_event(&mut self, toplevel: ToplevelHandle, evt: ToplevelHandleEvent) {
        let data = &mut self
            .toplevel_state
            .toplevels
            .iter_mut()
            .find(|(x, _)| *x == toplevel)
            .expect("Received event for dead toplevel")
            .1;
        match evt {
            ToplevelHandleEvent::Title { title } => {
                data.pending_info.title = title;
            }
            ToplevelHandleEvent::AppId { app_id } => {
                data.pending_info.app_id = app_id;
            }
            ToplevelHandleEvent::OutputEnter { output } => {
                data.pending_info.output.insert(output);
            }
            ToplevelHandleEvent::OutputLeave { output } => {
                data.pending_info.output.remove(&output);
            }
            ToplevelHandleEvent::State { state } => {
                log::debug!(
                    "{} STATE CHANGE! new_pending: {:?} -> pending: {:?} :: current: {:?}",
                    data.pending_info.app_id,
                    state,
                    data.pending_info.state,
                    data.current_info.as_ref().map(|info| info.state.clone())
                );
                data.pending_info.state = state;
            }
            ToplevelHandleEvent::Done => {
                log::debug!("{} commit!", data.pending_info.app_id);
                let is_new = data.current_info.is_none();
                data.current_info = Some(data.pending_info.clone());
                if is_new {
                    let _ = self
                        .tx
                        .unbounded_send(WaylandIncoming::Toplevel(ToplevelUpdate::Add(
                            toplevel.clone(),
                            data.current_info.as_ref().unwrap().clone(),
                        )));
                } else {
                    let _ =
                        self.tx
                            .unbounded_send(WaylandIncoming::Toplevel(ToplevelUpdate::Update(
                                toplevel.clone(),
                                data.current_info.as_ref().unwrap().clone(),
                            )));
                }
            }
            ToplevelHandleEvent::Closed => {
                let _ = self
                    .tx
                    .unbounded_send(WaylandIncoming::Toplevel(ToplevelUpdate::Remove(
                        toplevel.clone(),
                    )));

                let toplevels = &mut self.toplevel_state.toplevels;
                if let Some(idx) = toplevels.iter().position(|(handle, _)| *handle == toplevel) {
                    toplevels.remove(idx);
                }
            }
            ToplevelHandleEvent::None => {}
        }
    }

    fn handle_toplevel_manager_event(&mut self, evt: ToplevelManagerEvent) {
        match evt {
            ToplevelManagerEvent::Toplevel(toplevel) => {
                self.toplevel_state
                    .toplevels
                    .push((toplevel, ToplevelData::default()));
            }
            ToplevelManagerEvent::Finished => {}
        }
    }
}

#[derive(Clone, Debug, Default)]
struct ToplevelManager {
    toplevels: Vec<(ToplevelHandle, ToplevelData)>,
}

#[derive(Clone, Debug, Default)]
struct ToplevelData {
    current_info: Option<CompositorToplevelInfo>,
    pending_info: CompositorToplevelInfo,
}

#[derive(Clone, Debug, Default)]
pub struct ToplevelInfo {
    pub title: String,
    pub app_id: String,
    pub state: HashSet<ToplevelState>,
    pub output: HashSet<wl_output::WlOutput>,
    pub workspace: HashSet<WorkspaceHandle>,
}

#[derive(Clone, Debug)]
pub enum WorkspaceHandle {
    Zcosmic(zcosmic_workspace_handle_v1::ZcosmicWorkspaceHandleV1),
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum ToplevelState {
    Maximized,
    Minimized,
    Fullscreen,
    Activated,
}

impl SeatHandler for AppData {
    fn seat_state(&mut self) -> &mut smithay_client_toolkit::seat::SeatState {
        &mut self.seat_state
    }

    fn new_seat(
        &mut self,
        _conn: &Connection,
        _qh: &QueueHandle<Self>,
        _seat: wayland_client::protocol::wl_seat::WlSeat,
    ) {
    }

    fn new_capability(
        &mut self,
        _conn: &Connection,
        _qh: &QueueHandle<Self>,
        _seat: wayland_client::protocol::wl_seat::WlSeat,
        _capability: smithay_client_toolkit::seat::Capability,
    ) {
    }

    fn remove_capability(
        &mut self,
        _conn: &Connection,
        _qh: &QueueHandle<Self>,
        _seat: wayland_client::protocol::wl_seat::WlSeat,
        _capability: smithay_client_toolkit::seat::Capability,
    ) {
    }

    fn remove_seat(
        &mut self,
        _conn: &Connection,
        _qh: &QueueHandle<Self>,
        _seat: wayland_client::protocol::wl_seat::WlSeat,
    ) {
    }
}

impl OutputHandler for AppData {
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
                .unbounded_send(WaylandIncoming::Output(OutputUpdate::Add(
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
                .unbounded_send(WaylandIncoming::Output(OutputUpdate::Update(
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
            .unbounded_send(WaylandIncoming::Output(OutputUpdate::Remove(
                output.clone(),
            )));
    }
}

#[derive(Debug, Clone)]
enum ToplevelHandleEvent {
    Title { title: String },
    AppId { app_id: String },
    OutputEnter { output: WlOutput },
    OutputLeave { output: WlOutput },
    State { state: HashSet<ToplevelState> },
    Done,
    Closed,
    None,
}

#[derive(Debug, Clone)]
enum ToplevelManagerEvent {
    Toplevel(ToplevelHandle),
    Finished,
}

// WL REGISTRY

// You need to provide a Dispatch<WlRegistry, GlobalListContents> impl for your app
impl wayland_client::Dispatch<wl_registry::WlRegistry, GlobalListContents> for AppData {
    fn event(
        _state: &mut AppData,
        _proxy: &wl_registry::WlRegistry,
        _event: wl_registry::Event,
        // This mutex contains an up-to-date list of the currently known globals
        // including the one that was just added or destroyed
        _data: &GlobalListContents,
        _conn: &Connection,
        _qhandle: &QueueHandle<AppData>,
    ) {
        // Left empty
    }
}

fn wayland_client_listener(tx: UnboundedSender<WaylandIncoming>, rx: Channel<WaylandRequest>) {
    let conn = Connection::connect_to_env().unwrap();

    // Retrieve the WlDisplay Wayland object from the connection. This object is
    // the starting point of any Wayland program, from which all other objects will
    // be created.
    let _display = conn.display();

    // Create an event queue for our event processing

    let (globals, event_queue) = registry_queue_init(&conn).unwrap();

    let mut event_loop = EventLoop::<AppData>::try_new().unwrap();
    let qh = event_queue.handle();
    let wayland_source = WaylandSource::new(conn.clone(), event_queue);
    let handle = event_loop.handle();
    wayland_source
        .insert(handle.clone())
        .expect("Failed to insert wayland source.");
    if handle
        .insert_source(rx, |event, _, state| match event {
            channel::Event::Msg(req) => {
                log::trace!("WaylandRequest: {:?}", req);
                match req {
                    WaylandRequest::Toplevel(req) => match req {
                        WaylandToplevelRequest::Activate(handle) => match handle {
                            ToplevelHandle::Zwlr(zwlr_foreign_toplevel_handle_v1) => todo!(),
                            ToplevelHandle::Zcosmic(handle) => {
                                log::debug!("Activating toplevel!");
                                if let Some(seat) = state.seat_state.seats().next() {
                                    let manager = state.zcosmic_toplevel_manager.as_ref().unwrap();
                                    manager.activate(&handle, &seat);
                                }
                            }
                        },
                        WaylandToplevelRequest::Minimize(handle) => match handle {
                            ToplevelHandle::Zwlr(zwlr_foreign_toplevel_handle_v1) => todo!(),
                            ToplevelHandle::Zcosmic(handle) => {
                                log::debug!("Minimizing toplevel!");
                                let manager = state.zcosmic_toplevel_manager.as_ref().unwrap();
                                manager.set_minimized(&handle);
                            }
                        },
                        WaylandToplevelRequest::Quit(handle) => match handle {
                            ToplevelHandle::Zwlr(zwlr_foreign_toplevel_handle_v1) => todo!(),
                            ToplevelHandle::Zcosmic(handle) => {
                                let manager = state.zcosmic_toplevel_manager.as_ref().unwrap();
                                manager.close(&handle);
                            }
                        },
                    },
                    WaylandRequest::TokenRequest {
                        app_id: _,
                        exec: _,
                        gpu_idx: _,
                    } => {
                        // if let Some(activation_state) = state.activation_state.as_ref() {
                        //     let seat_and_serial = state.seat_state.seats().next().map(|seat| (seat, 0));
                        //     activation_state.request_token_with_data(
                        //         &state.queue_handle,
                        //         ExecRequestData {
                        //             data: RequestData {
                        //                 app_id: Some(app_id),
                        //                 seat_and_serial,
                        //                 surface: None,
                        //             },
                        //             exec,
                        //             gpu_idx,
                        //         },
                        //     );
                        // } else {
                        //     // let _ = state.tx.unbounded_send(WaylandIncoming::ActivationToken {
                        //     //     _token: None,
                        //     //     _app_id: Some(app_id),
                        //     //     _exec: exec,
                        //     //     _gpu_idx: gpu_idx,
                        //     // });
                        // }
                    }
                }
            }
            channel::Event::Closed => {
                state.exit = true;
            }
        })
        .is_err()
    {
        return;
    }

    globals.contents().with_list(|list| {
        for item in list {
            log::trace!("{} @ {}", item.interface, item.version);
        }
    });

    // now you can bind the globals you need for your app
    let zwlr_toplevel_manager = match globals
        .bind::<zwlr_foreign_toplevel_manager_v1::ZwlrForeignToplevelManagerV1, _, _>(
        &qh,
        3..=3,
        (),
    ) {
        Ok(manager) => Some(manager),
        Err(e) => {
            log::info!(
                "[PROTOCOL] zwlr_foreign_toplevel_manager_v1 could not be bound: {}",
                e
            );
            None
        }
    };

    let zcosmic_toplevel_info = match globals
        .bind::<zcosmic_toplevel_info_v1::ZcosmicToplevelInfoV1, _, _>(&qh, 1..=1, ())
    {
        Ok(manager) => Some(manager),
        Err(e) => {
            log::info!(
                "[PROTOCOL] zcosmic_toplevel_info_v1 could not be bound: {}",
                e
            );
            None
        }
    };

    let zcosmic_toplevel_manager = match globals
        .bind::<zcosmic_toplevel_manager_v1::ZcosmicToplevelManagerV1, _, _>(
        &qh,
        1..=1,
        (),
    ) {
        Ok(manager) => Some(manager),
        Err(e) => {
            log::info!(
                "[PROTOCOL] zcosmic_toplevel_manager_v1 could not be bound: {}",
                e
            );
            None
        }
    };

    let kde_window_manager = match globals
        .bind::<org_kde_plasma_window_management::OrgKdePlasmaWindowManagement, _, _>(
        &qh,
        15..=16,
        (),
    ) {
        Ok(manager) => Some(manager),
        Err(e) => {
            log::info!(
                "[PROTOCOL] org_kde_plasma_window_management could not be bound: {}",
                e
            );
            None
        }
    };

    // let zwlr_toplevel_handle: zwlr_foreign_toplevel_handle_v1::ZwlrForeignToplevelHandleV1 =
    //     globals.bind(&qh, 3..=3, ()).unwrap();

    // let toplevel_manager: zwlr_foreign_toplevel_manager_v1::ZwlrForeignToplevelManagerV1 =
    // globals.bind(&qh, 3..=3, ()).unwrap();

    let mut app_data = AppData {
        exit: false,
        tx,
        output_state: OutputState::new(&globals, &qh),
        seat_state: SeatState::new(&globals, &qh),
        toplevel_state: ToplevelManager::default(),
        zcosmic_toplevel_info,
        zcosmic_toplevel_manager,
        zwlr_toplevel_manager,
        kde_window_manager,
    };

    loop {
        if app_data.exit {
            log::debug!("Exiting wayland loop...");
            break;
        }
        event_loop.dispatch(None, &mut app_data).unwrap();
    }
}

smithay_client_toolkit::delegate_seat!(AppData);
smithay_client_toolkit::delegate_output!(AppData);

#[derive(Clone, Debug)]
pub enum WaylandIncoming {
    Init(channel::Sender<WaylandRequest>),
    Finished,
    Toplevel(ToplevelUpdate),
    Output(OutputUpdate),
}

#[derive(Clone, Debug)]
pub enum ToplevelUpdate {
    Add(ToplevelHandle, CompositorToplevelInfo),
    Update(ToplevelHandle, CompositorToplevelInfo),
    Remove(ToplevelHandle),
}

#[derive(Clone, Debug, Default)]
pub struct CompositorToplevelInfo {
    pub title: String,
    pub app_id: String,
    pub state: HashSet<ToplevelState>,
    pub output: HashSet<wl_output::WlOutput>,
}

#[derive(Clone, Debug)]
pub enum OutputUpdate {
    Add(WlOutput, OutputInfo),
    Update(WlOutput, OutputInfo),
    Remove(WlOutput),
}

#[derive(Clone, Debug)]
pub enum WaylandOutgoing {
    Exec(String, String),
    Toggle(ToplevelHandle),
    #[allow(unused)]
    Activate(ToplevelHandle),
}

#[derive(Debug, Clone)]
pub enum ToplevelRequest {
    Activate(ToplevelHandle),
    Minimize(ToplevelHandle),
    #[allow(unused)]
    Quit(ToplevelHandle),
}

#[derive(Clone, Debug)]
pub enum WaylandRequest {
    Toplevel(WaylandToplevelRequest),
    TokenRequest {
        app_id: String,
        exec: String,
        gpu_idx: Option<usize>,
    },
}

#[derive(Debug, Clone)]
pub enum WaylandToplevelRequest {
    Activate(ToplevelHandle),
    Minimize(ToplevelHandle),
    #[allow(unused)]
    Quit(ToplevelHandle),
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum ToplevelHandle {
    Zwlr(ZwlrForeignToplevelHandleV1),
    Zcosmic(ZcosmicToplevelHandleV1),
}

#[derive(Debug, Clone)]
pub struct CompositorBackend {
    wayland_sender: Option<Sender<WaylandRequest>>,
    // active_workspaces: Vec<ZcosmicWorkspaceHandleV1>,
    pub active_toplevels: HashMap<String, HashMap<ToplevelHandle, CompositorToplevelInfo>>,
    output_list: HashMap<WlOutput, OutputInfo>,
    _current_output: String, // TODO: Get current output
}

pub enum State {
    Waiting,
    Finished,
}

pub static WAYLAND_RX: Lazy<Mutex<Option<UnboundedReceiver<WaylandIncoming>>>> =
    Lazy::new(|| Mutex::new(None));

async fn start_listening(
    state: State,
    output: &mut futures::channel::mpsc::Sender<WaylandIncoming>,
) -> State {
    match state {
        State::Waiting => {
            let mut guard = WAYLAND_RX.lock().await;
            let rx = {
                if guard.is_none() {
                    let (calloop_tx, calloop_rx) = channel::channel();
                    let (toplevel_tx, toplevel_rx) = iced::futures::channel::mpsc::unbounded();
                    let _ = std::thread::spawn(move || {
                        wayland_client_listener(toplevel_tx, calloop_rx);
                    });
                    *guard = Some(toplevel_rx);
                    _ = output.send(WaylandIncoming::Init(calloop_tx)).await;
                }
                guard.as_mut().unwrap()
            };
            match rx.next().await {
                Some(u) => {
                    _ = output.send(u).await;
                    State::Waiting
                }
                None => {
                    _ = output.send(WaylandIncoming::Finished).await;
                    State::Finished
                }
            }
        }
        State::Finished => iced::futures::future::pending().await,
    }
}

impl Default for CompositorBackend {
    fn default() -> Self {
        Self::new()
    }
}

impl CompositorBackend {
    pub fn new() -> Self {
        Self {
            wayland_sender: None,
            active_toplevels: HashMap::new(),
            output_list: HashMap::new(),
            _current_output: "".to_string(),
        }
    }

    pub fn wayland_subscription(&self) -> Subscription<WaylandIncoming> {
        Subscription::run(|| {
            iced::stream::channel(50, move |mut output| async move {
                let mut state = State::Waiting;

                loop {
                    state = start_listening(state, &mut output).await;
                }
            })
        })
    }

    pub fn handle_outgoing(&mut self, outgoing: WaylandOutgoing) -> Option<Task<AppTrayMessage>> {
        match outgoing {
            WaylandOutgoing::Exec(app_id, exec) => {
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
                if let Some(tx) = self.wayland_sender.as_ref() {
                    let _ = tx.send(WaylandRequest::Toplevel(
                        if self
                            .active_window()
                            .is_some_and(|active_window| active_window == window)
                        {
                            WaylandToplevelRequest::Minimize(window)
                        } else {
                            WaylandToplevelRequest::Activate(window)
                        },
                    ));
                }
                // if let Some(p) = self.popup.take() {
                //     return destroy_popup(p.id);
                // }
                None
            }
            WaylandOutgoing::Activate(window) => {
                if let Some(tx) = self.wayland_sender.as_ref() {
                    let _ = tx.send(WaylandRequest::Toplevel(WaylandToplevelRequest::Activate(
                        window,
                    )));
                }
                // if let Some(p) = self.popup.take() {
                //     return destroy_popup(p.id);
                // }
                None
            }
        }
    }

    pub fn handle_incoming(&mut self, incoming: WaylandIncoming) -> Option<Task<AppTrayMessage>> {
        match incoming {
            WaylandIncoming::Init(sender) => {
                self.wayland_sender.replace(sender);
                None
            }
            WaylandIncoming::Finished => None,
            WaylandIncoming::Toplevel(toplevel_update) => match toplevel_update {
                ToplevelUpdate::Add(handle, info) => {
                    let app_id = info.app_id.clone();
                    if self.active_toplevels.contains_key(&app_id) {
                        self.active_toplevels
                            .get_mut(&info.app_id)
                            .unwrap()
                            .insert(handle, info);
                    } else {
                        self.active_toplevels
                            .insert(app_id.clone(), HashMap::from([(handle, info.clone())]));
                    }
                    None
                }
                ToplevelUpdate::Update(handle, info) => {
                    // TODO probably want to make sure it is removed
                    if info.app_id.is_empty() {
                        return Some(Task::none());
                    } else if !self.active_toplevels.contains_key(&info.app_id) {
                        return Some(Task::none());
                    }

                    for (t_handle, t_info) in self.active_toplevels.get_mut(&info.app_id).unwrap() {
                        if &handle == t_handle {
                            *t_info = info;
                            break;
                        }
                    }

                    None
                }
                ToplevelUpdate::Remove(handle) => {
                    let mut target_app_id: Option<String> = None;
                    for (app_id, app_info) in self.active_toplevels.iter_mut() {
                        if app_info.contains_key(&handle.clone()) {
                            app_info.remove(&handle);
                            if app_info.is_empty() {
                                target_app_id = Some(app_id.clone());
                            }
                            break;
                        }
                    }
                    if let Some(app_id) = target_app_id {
                        self.active_toplevels.remove(&app_id);
                    }
                    None
                }
            },
            WaylandIncoming::Output(output_update) => match output_update {
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
        }
    }

    pub fn active_window(&self) -> Option<ToplevelHandle> {
        // if self.active_workspaces.is_empty() {
        //     return None;
        // }
        let mut focused_toplevels: Vec<ToplevelHandle> = Vec::new();
        // let active_workspaces = self.active_workspaces.clone();
        for (_, app_group) in self.active_toplevels.iter() {
            for (t_handle, t_info) in app_group.iter() {
                if t_info.state.contains(&ToplevelState::Activated)
                        // && active_workspaces
                        //     .iter()
                        //     .any(|workspace| t_info.workspace.contains(workspace))
                        && t_info.output.iter().any(|x| {
                            self.output_list.get(x).is_some_and(|_val| {
                                true // TODO: Output stuff
                                // val.name.as_ref().is_some_and(|n| *n == self.current_output)
                            })
                        })
                {
                    focused_toplevels.push(t_handle.clone());
                }
            }
        }
        focused_toplevels.first().cloned()
    }
}
