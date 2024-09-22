use std::path::{Path, PathBuf};

use iced::{
    widget::{column, Container},
    Element, Length, Theme,
};

use crate::component_theme::app_tray_icon_rule;

pub fn app_tray_button<'a, T: 'a>(
    icon_path: Option<&Path>,
    is_active: bool,
    num_toplevels: usize,
    is_start_menu: bool,
) -> iced::widget::Button<'a, T> {
    match icon_path {
        Some(path) => iced::widget::button(if is_start_menu {
            column![crate::components::app_icon(path)]
        } else {
            column![
                app_tray_horizontal_rule(is_active, num_toplevels, true),
                crate::components::app_icon(path),
                app_tray_horizontal_rule(is_active, num_toplevels, false)
            ]
        }),
        None => iced::widget::button(iced::widget::Space::new(Length::Fill, Length::Fill))
            .width(Length::Fill)
            .height(Length::Fill),
    }
    .width(Length::Fill)
    .height(Length::Fill)
    .padding(if is_start_menu { 2 } else { 4 })
}

fn app_tray_horizontal_rule<'a, T: 'a>(
    is_active: bool,
    num_toplevels: usize,
    force_transparent: bool,
) -> Container<'a, T> {
    let len = Length::Fixed(if is_active { 16.0 } else { 8.0 });
    let transparent = force_transparent || num_toplevels == 0;
    iced::widget::container(
        iced::widget::horizontal_rule(1)
            .style(move |theme: &Theme| app_tray_icon_rule(theme, transparent))
            .width(len),
    )
    .center_x(Length::Fill)
}

pub fn app_icon<'a, T>(icon_path: &Path) -> iced::Element<'a, T> {
    if icon_path.extension().is_some_and(|x| x == "svg") {
        Element::from(
            iced::widget::svg(icon_path)
                .content_fit(iced::ContentFit::Contain)
                .width(Length::Fill)
                .height(Length::Fill),
        )
    } else {
        Element::from(
            iced::widget::image(icon_path)
                .content_fit(iced::ContentFit::Contain)
                .width(Length::Fill)
                .height(Length::Fill),
        )
    }
}

pub fn default_icon_path() -> PathBuf {
    freedesktop_icons::lookup("wayland")
        .with_theme("breeze")
        .with_cache()
        .find()
        .unwrap()
}

pub fn start_menu_icon() -> Option<PathBuf> {
    freedesktop_icons::lookup("applications-all")
        .with_theme("breeze")
        .with_cache()
        .find()
        .or_else(|| {
            freedesktop_icons::lookup("applications-office")
                .with_theme("Cosmic")
                .with_cache()
                .find()
        })
}
