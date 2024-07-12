use bevy::asset::AssetServer;
use bevy::color::Color;
use bevy::prelude::{AlignItems, JustifyContent, Res, Style, TextStyle, UiRect, Val};

pub const NORMAL_BUTTON_COLOR: Color = Color::srgb(1.0, 1.0, 1.0);
pub const HOVERED_BUTTON_COLOR: Color = Color::srgb(0.95, 0.95, 0.95);

pub const BUTTON_STYLE: Style = {
    let mut style = Style::DEFAULT;

    style.justify_content = JustifyContent::Center;
    style.align_items = AlignItems::Center;
    style.width = Val::Px(400.0);
    style.height = Val::Px(60.0);
    style.border = UiRect::all(Val::Px(2.0));

    style
};

pub fn get_button_text_style(asset_server: &Res<AssetServer>) -> TextStyle {
    TextStyle {
        font: asset_server.load("fonts/OpenSans-Regular.ttf"),
        font_size: 32.0,
        color: Color::BLACK,
    }
}

pub fn get_begin_button_text_style(asset_server: &Res<AssetServer>) -> TextStyle {
    TextStyle {
        font: asset_server.load("fonts/OpenSans-SemiBold.ttf"),
        font_size: 48.0,
        color: Color::BLACK,
    }
}
