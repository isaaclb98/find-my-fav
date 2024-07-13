use bevy::prelude::{AlignItems, FlexDirection, JustifyContent, Style, UiRect, Val};

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
