use bevy::asset::AssetServer;
use bevy::color::Color;
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

pub const NODE_BUNDLE_EMPTY_COLUMN_STYLE: Style = {
    let mut style = Style::DEFAULT;

    style.flex_direction = FlexDirection::Column;
    style.justify_content = JustifyContent::Center;
    style.align_items = AlignItems::Center;
    style.width = Val::Percent(100.0);
    style.height = Val::Percent(100.0);
    style.row_gap = Val::Px(0.0);
    style.column_gap = Val::Px(0.0);

    style
};

pub const NODE_BUNDLE_EMPTY_ROW_STYLE: Style = {
    let mut style = Style::DEFAULT;

    style.flex_direction = FlexDirection::Row;
    style.justify_content = JustifyContent::Center;
    style.align_items = AlignItems::Center;
    style.width = Val::Percent(100.0);
    style.height = Val::Percent(100.0);
    style.row_gap = Val::Px(0.0);
    style.column_gap = Val::Px(0.0);

    style
};

pub const NODE_BUNDLE_GAPS_COLUMN_STYLE: Style = {
    let mut style = Style::DEFAULT;

    style.flex_direction = FlexDirection::Column;
    style.justify_content = JustifyContent::Center;
    style.align_items = AlignItems::Center;
    style.width = Val::Percent(100.0);
    style.height = Val::Percent(100.0);
    style.row_gap = Val::Px(32.0);
    style.column_gap = Val::Px(8.0);

    style
};

pub const NODE_BUNDLE_GAPS_ROW_STYLE: Style = {
    let mut style = Style::DEFAULT;

    style.flex_direction = FlexDirection::Row;
    style.justify_content = JustifyContent::Center;
    style.align_items = AlignItems::Center;
    style.width = Val::Percent(100.0);
    style.height = Val::Px(60.0);
    style.row_gap = Val::Px(8.0);
    style.column_gap = Val::Px(8.0);

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
