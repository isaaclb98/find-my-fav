use bevy::prelude::*;

pub const NORMAL_BUTTON_COLOR: Color = Color::srgb(1.0, 1.0, 1.0);
pub const HOVERED_BUTTON_COLOR: Color = Color::srgb(0.95, 0.95, 0.95);

pub const BUTTON_STYLE: Style = {
    let mut style = Style::DEFAULT;

    style.justify_content = JustifyContent::Center;
    style.align_items = AlignItems::Center;
    style.width = Val::Px(400.0);
    style.height = Val::Px(60.0);

    style
};
