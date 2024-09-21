use std::rc::Rc;

use iced::{
    border::Radius,
    platform_specific::{
        runtime::wayland::layer_surface::{IcedOutput, SctkLayerSurfaceSettings},
        shell::commands::layer_surface::get_layer_surface,
    },
    widget::{column, row, text},
    window::{self, Id, Settings},
    Element, Length, Padding, Size, Subscription, Task, Theme,
};
use smithay_client_toolkit::shell::wlr_layer::Anchor;
use wayland_protocols_wlr::layer_shell;

use crate::{
    app_tray::{AppTray, AppTrayMessage},
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
    AppTrayContextMenu { app_id: String },
    StartMenu,
}

impl<'a> Panel<'a> {
    pub fn new(config: PanelConfig) -> (Self, Task<Message>) {
        let id = Id::unique();
        let open: Task<Message> = get_layer_surface(SctkLayerSurfaceSettings {
            id,
            layer: smithay_client_toolkit::shell::wlr_layer::Layer::Top,
            // keyboard_interactivity: todo!(),
            // pointer_interactivity: todo!(),
            anchor: Anchor::LEFT.union(Anchor::BOTTOM).union(Anchor::RIGHT),
            output: IcedOutput::Active,
            // namespace: todo!(),
            // margin: todo!(),
            size: Some((None, Some(48))),
            exclusive_zone: 48,
            // size_limits: todo!(),
            ..Default::default()
        });
        // let (id, open) = window::open(window::Settings {
        //     size: (1280.0, 48.0).into(),
        //     ..Default::default()
        // });
        log::debug!("Window requested open {:?}", id);
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
            // open.map(Message::OpenMainWindow),
        )
    }

    pub fn title(&self, _window: window::Id) -> String {
        "Window".into()
    }

    pub fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::StartMenu(StartMenuMessage::MenuToggle) => {
                log::warn!("Start menu toggle!!");
                let (_, task) = window::open(Settings {
                    position: window::Position::Centered,
                    size: Size::new(240.0, 480.0),
                    ..Default::default()
                });
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
                            app_id: app_id.clone(),
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
                let task = if let Some((popup, _)) = self.popup_window.take() {
                    iced::window::close(popup)
                } else {
                    Task::none()
                };
                log::debug!("Popup opened! {:?}", id);
                self.popup_window = Some((id, popup_info));
                task
            }
            Message::OpenMainWindow(_) => Task::none(),
        }
    }

    pub fn theme(&self, _window: window::Id) -> Theme {
        Theme::Dark
    }

    pub fn view(&self, window: window::Id) -> Element<Message> {
        if window == self.main_window {
            // if window != self.main_window {
            let panel_items = row![
                self.start_menu.view().map(Message::StartMenu),
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
                PopupType::AppTrayContextMenu { app_id } => text!("Hey").into(),
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
    OpenMainWindow(window::Id),
}
