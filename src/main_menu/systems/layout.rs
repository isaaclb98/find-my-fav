use crate::main_menu::components::*;
use crate::main_menu::styles::{BUTTON_STYLE, NORMAL_BUTTON_COLOR};
use crate::styles::get_button_text_style;
use bevy::prelude::*;
use bevy::text::BreakLineOn;
use bevy::window::PrimaryWindow;

pub fn spawn_main_menu(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    let main_menu_entity = build_main_menu(&mut commands, &asset_server, &window_query);
}

pub fn despawn_main_menu(
    mut commands: Commands,
    main_menu_query: Query<Entity, With<MainMenuComponent>>,
) {
    if let Ok(main_menu_entity) = main_menu_query.get_single() {
        commands.entity(main_menu_entity).despawn_recursive();
    }
}

pub fn build_main_menu(
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    window_query: &Query<&Window, With<PrimaryWindow>>,
) -> Entity {
    let window: &Window = window_query.get_single().unwrap();

    let main_menu_entity = commands
        .spawn((
            NodeBundle {
                style: Style {
                    flex_direction: FlexDirection::Column,
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    row_gap: Val::Px(8.0),
                    column_gap: Val::Px(8.0),
                    ..default()
                },
                background_color: Color::WHITE.into(),
                ..default()
            },
            MainMenuComponent {},
        ))
        .with_children(|parent| {
            // title
            parent.spawn(TextBundle {
                text: Text {
                    sections: vec![TextSection::new(
                        "FindMyFav",
                        TextStyle {
                            font: asset_server.load("fonts/OpenSans-SemiBold.ttf"),
                            font_size: 64.0,
                            color: Color::BLACK,
                        },
                    )],
                    justify: JustifyText::Center,
                    linebreak_behavior: BreakLineOn::NoWrap,
                },
                ..default()
            });

            // folder button
            parent
                .spawn((
                    ButtonBundle {
                        style: BUTTON_STYLE,
                        background_color: NORMAL_BUTTON_COLOR.into(),
                        ..default()
                    },
                    OpenFolderButton {},
                ))
                .with_children(|parent| {
                    parent.spawn(TextBundle {
                        text: Text {
                            sections: vec![TextSection::new(
                                "Open a folder",
                                get_button_text_style(&asset_server),
                            )],
                            justify: JustifyText::Center,
                            ..default()
                        },
                        ..default()
                    });
                });

            // resume previous button
            parent
                .spawn((
                    ButtonBundle {
                        style: BUTTON_STYLE,
                        background_color: NORMAL_BUTTON_COLOR.into(),
                        ..default()
                    },
                    ResumePreviousButton {},
                ))
                .with_children(|parent| {
                    parent.spawn(TextBundle {
                        text: Text {
                            sections: vec![TextSection::new(
                                "Resume a previous game",
                                get_button_text_style(&asset_server),
                            )],
                            justify: JustifyText::Center,
                            ..default()
                        },
                        ..default()
                    });
                });
        })
        .id();

    main_menu_entity
}
