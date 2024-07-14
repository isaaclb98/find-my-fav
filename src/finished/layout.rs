use crate::database::{get_image_path_from_database, get_remaining_participants};
use crate::finished::components::*;
use crate::styles::*;
use crate::tournament::components::ParticipantsDeque;
use crate::AppState;
use bevy::asset::AssetServer;
use bevy::prelude::*;
use bevy::text::BreakLineOn;
use bevy::window::PrimaryWindow;
use image::GenericImageView;

pub fn spawn_finished_screen(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    mut participants_deque_resource: ResMut<ParticipantsDeque>,
    mut next_app_state: ResMut<NextState<AppState>>,
) {
    let window: &Window = window_query.get_single().unwrap();
    let window_width = window.width();
    let window_height = window.height();

    let mut participants = get_remaining_participants().unwrap();

    let image_id_1 = participants.get(0).expect("Couldn't get image_id");
    let image_path_1 =
        get_image_path_from_database(&image_id_1).expect("Failed to get image path from database");
    let image_1 = image::open(&image_path_1).unwrap();
    let (width_1, height_1) = image_1.dimensions();
    let image_aspect_ratio_1 = width_1 as f32 / height_1 as f32;
    let target_height = window_height / 2.0;
    let target_width = target_height * image_aspect_ratio_1;
    let (final_width_1, final_height_1) = (target_width, target_height);

    let texture_handle_1: Handle<Image> = asset_server.load(image_path_1);

    commands
        .spawn((
            NodeBundle {
                style: NODE_BUNDLE_GAPS_COLUMN_STYLE,
                background_color: Color::WHITE.into(),
                ..default()
            },
            FinishedScreenComponent,
        ))
        .with_children(|parent| {
            // winner text
            parent.spawn(TextBundle {
                text: Text {
                    sections: vec![TextSection::new(
                        "Winner!",
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

            // image
            parent.spawn(ImageBundle {
                style: Style {
                    width: Val::Px(final_width_1),
                    height: Val::Px(final_height_1),
                    ..default()
                },
                image: UiImage::new(texture_handle_1),
                ..default()
            });

            // horizontal flexbox
            parent
                .spawn(NodeBundle {
                    style: NODE_BUNDLE_GAPS_ROW_STYLE,
                    background_color: Color::WHITE.into(),
                    ..default()
                })
                .with_children(|parent| {
                    // open new folder button
                    parent
                        .spawn((
                            ButtonBundle {
                                style: BUTTON_STYLE,
                                background_color: NORMAL_BUTTON_COLOR.into(),
                                ..default()
                            },
                            OpenCreatedFolderButton {},
                        ))
                        .with_children(|parent| {
                            parent.spawn(TextBundle {
                                text: Text {
                                    sections: vec![TextSection::new(
                                        "Open new folder",
                                        get_button_text_style(&asset_server),
                                    )],
                                    justify: JustifyText::Center,
                                    ..default()
                                },
                                ..default()
                            });
                        });

                    // restart button
                    parent
                        .spawn((
                            ButtonBundle {
                                style: BUTTON_STYLE,
                                background_color: NORMAL_BUTTON_COLOR.into(),
                                ..default()
                            },
                            StartOverButton {},
                        ))
                        .with_children(|parent| {
                            parent.spawn(TextBundle {
                                text: Text {
                                    sections: vec![TextSection::new(
                                        "Start a new game",
                                        get_button_text_style(&asset_server),
                                    )],
                                    justify: JustifyText::Center,
                                    ..default()
                                },
                                ..default()
                            });
                        });
                });
        });
}

pub fn despawn_finished_screen(
    mut commands: Commands,
    finished_screen_query: Query<Entity, With<FinishedScreenComponent>>,
) {
    if let Ok(finished_screen_entity) = finished_screen_query.get_single() {
        commands.entity(finished_screen_entity).despawn_recursive();
    } else {
        println!("FinishedScreenComponent entity not found.");
    }
}
