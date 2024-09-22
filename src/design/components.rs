use iced::{
    widget::{column, Container},
    Length, Theme,
};

use crate::freedesktop::icons::ImageHandle;

use super::component_theme::app_tray_icon_rule;

pub fn app_tray_button<'a, T: 'a>(
    icon_path: Option<ImageHandle>,
    is_active: bool,
    num_toplevels: usize,
    is_start_menu: bool,
) -> iced::widget::Button<'a, T> {
    match icon_path {
        Some(path) => iced::widget::button(if is_start_menu {
            column![crate::design::components::app_icon(path)]
        } else {
            column![
                app_tray_horizontal_rule(is_active, num_toplevels, true),
                crate::design::components::app_icon(path),
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

pub fn app_icon<'a, T>(image_handle: ImageHandle) -> iced::widget::Container<'a, T> {
    match image_handle {
        ImageHandle::Svg(handle) => iced::widget::container(
            iced::widget::svg(handle)
                .content_fit(iced::ContentFit::Contain)
                .width(Length::Fill)
                .height(Length::Fill),
        ),
        ImageHandle::Image(handle) => iced::widget::container(
            iced::widget::image(handle)
                .content_fit(iced::ContentFit::Contain)
                .width(Length::Fill)
                .height(Length::Fill),
        ),
    }
}
