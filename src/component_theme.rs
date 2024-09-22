use iced::{
    border::Radius,
    widget::{
        button::{self},
        rule,
    },
    Background, Border, Theme,
};

pub const PANEL_SIZE: u32 = 48;

pub const BUTTON_RADIUS: u16 = 8;

pub const APP_TRAY_RULE_THICKNESS: u16 = 3;

pub fn button_style(
    theme: &Theme,
    status: button::Status,
    is_active: bool,
    num_toplevels: usize,
) -> button::Style {
    let mut border_color = theme.palette().primary;
    let mut background_color = theme.palette().primary;
    (border_color.a, background_color.a) = if is_active {
        if matches!(status, button::Status::Hovered | button::Status::Pressed) {
            (0.26, 0.25)
        } else {
            (0.21, 0.20)
        }
    } else if num_toplevels > 0 {
        if matches!(status, button::Status::Hovered | button::Status::Pressed) {
            (0.11, 0.1)
        } else {
            (0.0, 0.0)
        }
    } else {
        if matches!(status, button::Status::Hovered | button::Status::Pressed) {
            (0.11, 0.1)
        } else {
            (0.0, 0.0)
        }
    };

    let mut button_theme = iced::widget::button::primary(theme, status);
    button_theme.background = Some(Background::Color(background_color));
    button_theme.border = Border {
        radius: Radius::from(BUTTON_RADIUS),
        color: border_color,
        width: 0.0,
    };
    button_theme
}

pub fn app_tray_icon_rule(theme: &Theme, transparent: bool) -> rule::Style {
    iced::widget::rule::Style {
        color: if transparent {
            iced::Color::TRANSPARENT
        } else {
            theme.palette().primary
        },
        width: APP_TRAY_RULE_THICKNESS,
        radius: 4.into(),
        // radius: 0.into(),
        fill_mode: iced::widget::rule::FillMode::Full,
    }
}
