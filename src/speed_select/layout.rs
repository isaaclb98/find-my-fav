use bevy::prelude::*;
use bevy::window::PrimaryWindow;

use crate::speed_select::components::*;
use crate::styles::*;

pub fn spawn_speed_select(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    let speed_select_entity =
        build_speed_select(&mut commands, &asset_server, &window_query, false);
}

pub fn despawn_speed_select(
    mut commands: Commands,
    speed_select_query: Query<Entity, With<SpeedSelectComponent>>,
) {
    if let Ok(speed_select_entity) = speed_select_query.get_single() {
        commands.entity(speed_select_entity).despawn_recursive();
    }
}

pub fn build_speed_select(
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    window_query: &Query<&Window, With<PrimaryWindow>>,
    enable_speed_select: bool,
) -> Entity {
    let speed_select_entity = commands
        .spawn((
            NodeBundle {
                style: Style {
                    flex_direction: FlexDirection::Column,
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    row_gap: Val::Px(32.0),
                    column_gap: Val::Px(8.0),
                    ..default()
                },
                background_color: Color::WHITE.into(),
                ..default()
            },
            SpeedSelectComponent {},
        ))
        .with_children(|parent| {
            // begin button
            parent
                .spawn((
                    ButtonBundle {
                        style: BUTTON_STYLE,
                        background_color: NORMAL_BUTTON_COLOR.into(),
                        ..default()
                    },
                    BeginButton {},
                ))
                .with_children(|parent| {
                    parent.spawn(TextBundle {
                        text: Text {
                            sections: vec![TextSection::new(
                                "Begin",
                                get_begin_button_text_style(&asset_server),
                            )],
                            justify: JustifyText::Center,
                            ..default()
                        },
                        ..default()
                    });
                });

            if enable_speed_select {
                parent
                    .spawn(NodeBundle {
                        style: Style {
                            flex_direction: FlexDirection::Row,
                            justify_content: JustifyContent::Center,
                            align_items: AlignItems::Center,
                            width: Val::Percent(100.0),
                            height: Val::Px(60.0),
                            row_gap: Val::Px(8.0),
                            column_gap: Val::Px(8.0),
                            ..default()
                        },
                        background_color: Color::WHITE.into(),
                        ..default()
                    })
                    .with_children(|parent| {
                        //button 1
                        parent
                            .spawn((
                                ButtonBundle {
                                    style: BUTTON_STYLE,
                                    background_color: NORMAL_BUTTON_COLOR.into(),
                                    border_color: NORMAL_BUTTON_COLOR.into(),
                                    ..default()
                                },
                                SlowButton {},
                            ))
                            .with_children(|parent| {
                                parent.spawn(TextBundle {
                                    text: Text {
                                        sections: vec![TextSection::new(
                                            "Slow",
                                            get_button_text_style(&asset_server),
                                        )],
                                        justify: JustifyText::Center,
                                        ..default()
                                    },
                                    ..default()
                                });
                            });

                        // button 2
                        parent
                            .spawn((
                                ButtonBundle {
                                    style: BUTTON_STYLE,
                                    background_color: NORMAL_BUTTON_COLOR.into(),
                                    border_color: NORMAL_BUTTON_COLOR.into(),
                                    ..default()
                                },
                                NormalButton {},
                            ))
                            .with_children(|parent| {
                                parent.spawn(TextBundle {
                                    text: Text {
                                        sections: vec![TextSection::new(
                                            "Normal",
                                            get_button_text_style(&asset_server),
                                        )],
                                        justify: JustifyText::Center,
                                        ..default()
                                    },
                                    ..default()
                                });
                            });

                        // button x3
                        parent
                            .spawn((
                                ButtonBundle {
                                    style: BUTTON_STYLE,
                                    background_color: NORMAL_BUTTON_COLOR.into(),
                                    border_color: NORMAL_BUTTON_COLOR.into(),
                                    ..default()
                                },
                                FastButton {},
                            ))
                            .with_children(|parent| {
                                parent.spawn(TextBundle {
                                    text: Text {
                                        sections: vec![TextSection::new(
                                            "Fast",
                                            get_button_text_style(&asset_server),
                                        )],
                                        justify: JustifyText::Center,
                                        ..default()
                                    },
                                    ..default()
                                });
                            });
                    });
            }
        })
        .id();

    speed_select_entity
}
