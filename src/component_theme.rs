use iced::{
    border::Radius,
    widget::button::{self},
    Background, Border, Theme,
};

pub const PANEL_SIZE: u32 = 48;

pub fn button_style(
    theme: &Theme,
    status: button::Status,
    is_active: bool,
    num_toplevels: usize,
) -> button::Style {
    let mut border_color = theme.palette().primary;
    let mut background_color = theme.palette().primary;
    (border_color.a, background_color.a) = if num_toplevels == 0 {
        if matches!(status, button::Status::Hovered | button::Status::Pressed) {
            (0.11, 0.1)
        } else {
            (0.0, 0.0)
        }
    } else if is_active {
        if matches!(status, button::Status::Hovered | button::Status::Pressed) {
            (0.26, 0.25)
        } else {
            (0.21, 0.20)
        }
    } else if matches!(status, button::Status::Hovered | button::Status::Pressed) {
        (0.11, 0.1)
    } else {
        (0.06, 0.05)
    };

    let mut button_theme = iced::widget::button::primary(theme, status);
    button_theme.background = Some(Background::Color(background_color));
    button_theme.border = Border {
        radius: Radius::from(8.0),
        color: border_color,
        width: 1.0,
    };
    button_theme
}
