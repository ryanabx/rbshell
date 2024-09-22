use std::{env, rc::Rc};

use iced::{
    border::Radius,
    platform_specific::{
        runtime::wayland::{
            layer_surface::{IcedOutput, SctkLayerSurfaceSettings},
            popup::{SctkPopupSettings, SctkPositioner},
        },
        shell::commands::{layer_surface::get_layer_surface, popup},
    },
    widget::{column, row, text},
    window::{self, Id, Settings},
    Element, Length, Padding, Subscription, Task, Theme,
};
use smithay_client_toolkit::shell::wlr_layer::Anchor;

use crate::{
    app_tray::{AppTray, AppTrayMessage},
    component_theme::PANEL_SIZE,
    config::PanelConfig,
    desktop_entry::DesktopEntryCache,
    settings_tray::{SettingsTray, SettingsTrayMessage},
    start_menu::{StartMenu, StartMenuMessage},
};

pub struct Panel<'a> {
    start_menu: StartMenu<'a>,
    app_tray: AppTray<'a>,
    settings_tray: SettingsTray,
    main_window: window::Id,
    popup_window: Option<(window::Id, PopupType)>,
}

#[derive(Clone, Debug)]
pub enum PopupType {
    AppTrayContextMenu { _app_id: String },
    StartMenu,
}

impl<'a> Panel<'a> {
    pub fn new(config: PanelConfig) -> (Self, Task<Message>) {
        let (id, open) =
            if env::var("RBSHELL_USE_WINIT").is_ok_and(|val| val.to_lowercase() == "true") {
                let (id, open) = window::open(window::Settings {
                    size: (1280.0, 48.0).into(),
                    ..Default::default()
                });
                (id, open.map(|_| Message::None))
            } else {
                let id = Id::unique();
                let open: Task<Message> = get_layer_surface(SctkLayerSurfaceSettings {
                    id,
                    layer: smithay_client_toolkit::shell::wlr_layer::Layer::Top,
                    // keyboard_interactivity: todo!(),
                    pointer_interactivity: true,
                    anchor: Anchor::BOTTOM.union(Anchor::LEFT).union(Anchor::RIGHT),
                    output: IcedOutput::Active,
                    // namespace: todo!(),
                    // margin: IcedMargin {
                    //     top: 5,
                    //     right: 5,
                    //     left: 5,
                    //     bottom: 5,
                    // },
                    // size: Some((None, Some(48))),
                    size: Some((None, Some(PANEL_SIZE))),
                    exclusive_zone: PANEL_SIZE as i32,
                    // size_limits: todo!(),
                    ..Default::default()
                });
                (id, open)
            };
        log::info!("Window requested open {:?}", id);
        let desktop_entry_cache = Rc::new(DesktopEntryCache::new());
        (
            Self {
                start_menu: StartMenu::new(desktop_entry_cache.clone()),
                app_tray: AppTray::new(config.app_tray, desktop_entry_cache.clone()),
                settings_tray: SettingsTray::new(),
                main_window: id,
                popup_window: None,
            },
            open,
        )
    }

    pub fn title(&self, _window: window::Id) -> String {
        "Window".into()
    }

    pub fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::StartMenu(StartMenuMessage::MenuToggle) => {
                log::debug!("Requested start menu");
                let id = Id::unique();
                let task = popup::get_popup(SctkPopupSettings {
                    parent: self.main_window,
                    id,
                    positioner: SctkPositioner {
                        size: Some((240, 480)),
                        // size_limits: todo!(),
                        // anchor_rect: todo!(),
                        // anchor: todo!(),
                        // gravity: todo!(),
                        // constraint_adjustment: todo!(),
                        // offset: todo!(),
                        // reactive: todo!(),
                        ..Default::default()
                    },
                    parent_size: None,
                    grab: true, // What does this do??
                });
                // let (_, task) = window::open(Settings {
                //     position: window::Position::Centered,
                //     size: Size::new(240.0, 480.0),
                //     ..Default::default()
                // });
                task.map(move |i| Message::OpenPopup(i, PopupType::StartMenu))
            }
            Message::StartMenu(start_menu_message) => self
                .start_menu
                .handle_message(start_menu_message)
                .map(Message::StartMenu),
            Message::AppTray(AppTrayMessage::ContextMenu(app_id)) => {
                let (_, task) = window::open(Settings {
                    position: window::Position::Centered,
                    ..Default::default()
                });
                task.map(move |i| {
                    Message::OpenPopup(
                        i,
                        PopupType::AppTrayContextMenu {
                            _app_id: app_id.clone(),
                        },
                    )
                })
            }
            Message::AppTray(app_tray_msg) => self
                .app_tray
                .handle_message(app_tray_msg)
                .map(Message::AppTray),
            Message::SettingsTray(settings_tray_msg) => self
                .settings_tray
                .handle_message(settings_tray_msg)
                .map(Message::SettingsTray),
            Message::OpenPopup(id, popup_info) => {
                log::debug!("Popup opened! {:?}", id);
                let task = if let Some((popup, popup_type)) = self.popup_window.take() {
                    match popup_type {
                        PopupType::AppTrayContextMenu { .. } => iced::window::close(popup),
                        PopupType::StartMenu => popup::destroy_popup(id),
                    }
                } else {
                    Task::none()
                };
                self.popup_window = Some((id, popup_info));
                task
            }
            Message::None => Task::none(),
        }
    }

    pub fn theme(&self, _window: window::Id) -> Theme {
        // Theme::Dark
        Theme::CatppuccinFrappe
    }

    pub fn view(&self, window: window::Id) -> Element<Message> {
        if window == self.main_window {
            // if window != self.main_window {
            let panel_items = row![
                self.start_menu
                    .view(
                        self.popup_window
                            .as_ref()
                            .is_some_and(|(_, popup_type)| matches!(
                                popup_type,
                                PopupType::StartMenu
                            ))
                    )
                    .map(Message::StartMenu),
                self.app_tray.view().map(Message::AppTray),
                self.settings_tray.view().map(Message::SettingsTray)
            ]
            .padding(Padding {
                right: 16.0,
                left: 16.0,
                top: 0.0,
                bottom: 0.0,
            });
            iced::widget::container(column![
                iced::widget::horizontal_rule(1).style(|theme: &Theme| iced::widget::rule::Style {
                    color: theme.palette().primary,
                    width: 1,
                    radius: Radius::from(0),
                    fill_mode: iced::widget::rule::FillMode::Full
                }),
                panel_items
            ])
            .width(Length::Fill)
            .height(Length::Fill)
            .into()
        } else if let Some(popup_window) = &self.popup_window.as_ref() {
            match &popup_window.1 {
                PopupType::AppTrayContextMenu { .. } => text!("Hey").into(),
                PopupType::StartMenu => self.start_menu.view_popup().map(Message::StartMenu),
            }
        } else {
            iced::widget::horizontal_space().into()
        }
    }

    pub fn subscription(&self) -> Subscription<Message> {
        Subscription::batch(vec![
            self.settings_tray.subscription().map(Message::SettingsTray),
            self.app_tray.subscription().map(Message::AppTray),
        ])
    }
}

#[derive(Clone, Debug)]
pub enum Message {
    StartMenu(StartMenuMessage),
    AppTray(AppTrayMessage),
    SettingsTray(SettingsTrayMessage),
    OpenPopup(window::Id, PopupType),
    None,
}
