use std::path::{Path, PathBuf};
use std::time::Instant;

use bevy::asset::{AssetServer, Handle, LoadState};
use bevy::hierarchy::{BuildChildren, DespawnRecursiveExt};
use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use image::GenericImageView;
use rand::prelude::SliceRandom;
use rand::thread_rng;

use crate::database::{
    get_image_path_from_database, get_latest_round_number, get_remaining_participants,
    increment_rating, insert_match_into_database, set_loser_out,
};
use crate::styles::NODE_BUNDLE_EMPTY_ROW_STYLE;
use crate::tournament::components::*;
use crate::tournament::components::{
    BothImageComponents, HandlesDeque, LeftImageComponent, ParticipantsDeque, RightImageComponent,
    TournamentState,
};
use crate::AppState;

pub fn get_participants_for_round(
    mut commands: Commands,
    mut participants_deque_resource: ResMut<ParticipantsDeque>,
    mut next_tournament_state: ResMut<NextState<TournamentState>>,
    mut next_app_state: ResMut<NextState<AppState>>,
    both_image_components_query: Query<Entity, With<BothImageComponents>>,
) {
    let start_time = Instant::now();

    let mut participants = get_remaining_participants().unwrap();

    if participants.len() == 1 {
        if let Ok(both_image_components_entity) = both_image_components_query.get_single() {
            commands
                .entity(both_image_components_entity)
                .despawn_recursive();
        }
        next_app_state.set(AppState::Finished);
        return;
    }

    let mut rng = thread_rng();
    participants.shuffle(&mut rng);

    for participant in participants {
        participants_deque_resource.deque.push_back(participant);
    }

    let duration = start_time.elapsed();
    println!("get_participants_for_round took {:?}", duration);
    next_tournament_state.set(TournamentState::Displaying);
}

fn sanitize_filename(path: &Path) -> PathBuf {
    let start_time = Instant::now();

    let sanitized: String = path
        .to_string_lossy()
        .chars()
        .filter(|c| c.is_ascii())
        .collect();

    let duration = start_time.elapsed();
    println!("sanitizing the image paths took {:?}", duration);

    PathBuf::from(sanitized)
}

pub fn load_images(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    images: Res<Assets<Image>>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    mut participants_deque_resource: ResMut<ParticipantsDeque>,
    mut handles_deque_resource: ResMut<HandlesDeque>,
    mut next_tournament_state: ResMut<NextState<TournamentState>>,
) {
    let image_ids: Option<(u64, u64)> = {
        let deque = &participants_deque_resource.deque;
        if let (Some(&image_id_1), Some(&image_id_2)) = (deque.get(0), deque.get(1)) {
            Some((image_id_1, image_id_2))
        } else {
            None
        }
    };

    if let Some((image_id_1, image_id_2)) = image_ids {
        let image_path_1 = get_image_path_from_database(&image_id_1)
            .expect("Failed to get image path from database");
        let image_path_2 = get_image_path_from_database(&image_id_2)
            .expect("Failed to get image path from database");

        let image_path_1 = sanitize_filename(&image_path_1);
        let image_path_2 = sanitize_filename(&image_path_2);

        let texture_handle_1: Handle<Image> = asset_server.load(image_path_1.clone());
        let texture_handle_2: Handle<Image> = asset_server.load(image_path_2.clone());

        handles_deque_resource
            .image_deque
            .push_back(texture_handle_1);
        handles_deque_resource
            .image_deque
            .push_back(texture_handle_2);

        std::thread::sleep(std::time::Duration::from_millis(50));
    }
}

pub fn display_images(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut handles_deque_resource: ResMut<HandlesDeque>,
    images: Res<Assets<Image>>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    mut event_writer: EventWriter<ImagesLoadedEvent>,
    mut next_tournament_state: ResMut<NextState<TournamentState>>,
) {
    let window: &Window = window_query.get_single().unwrap();
    let window_width = window.width();
    let window_height = window.height();

    let mut images_loaded = false;

    while !images_loaded {
        if let (Some(texture_handle_1), Some(texture_handle_2)) = (
            handles_deque_resource.image_deque.get(0),
            handles_deque_resource.image_deque.get(1),
        ) {
            if let (Some(LoadState::Loaded), Some(LoadState::Loaded)) = (
                asset_server.get_load_state(texture_handle_1),
                asset_server.get_load_state(texture_handle_2),
            ) {
                println!("IMAGES LOADED.");
                images_loaded = true;

                if let (Some(image_1), Some(image_2)) =
                    (images.get(texture_handle_1), images.get(texture_handle_2))
                {
                    let size_1 = image_1.size();
                    let size_2 = image_2.size();

                    println!("Image 1 size: {:?}", size_1);
                    println!("Image 2 size: {:?}", size_2);

                    let width_1 = size_1.x as f32;
                    let height_1 = size_1.y as f32;
                    let width_2 = size_2.x as f32;
                    let height_2 = size_2.y as f32;

                    let image_aspect_ratio_1 = width_1 / height_1;
                    let image_aspect_ratio_2 = width_2 / height_2;

                    // Calculate the target width to be half of the window's width
                    let target_width = window_width / 2.0;

                    // Calculate the target height to maintain the aspect ratio
                    let target_height_1 = target_width / image_aspect_ratio_1;
                    let target_height_2 = target_width / image_aspect_ratio_2;

                    // If the target height is greater than the window height, adjust the target dimensions
                    let (final_width_1, final_height_1) = if target_height_1 > window_height {
                        let adjusted_height = window_height;
                        let adjusted_width = adjusted_height * image_aspect_ratio_1;
                        (adjusted_width, adjusted_height)
                    } else {
                        (target_width, target_height_1)
                    };
                    let (final_width_2, final_height_2) = if target_height_2 > window_height {
                        let adjusted_height = window_height;
                        let adjusted_width = adjusted_height * image_aspect_ratio_2;
                        (adjusted_width, adjusted_height)
                    } else {
                        (target_width, target_height_2)
                    };

                    commands
                        .spawn((
                            NodeBundle {
                                style: NODE_BUNDLE_EMPTY_ROW_STYLE,
                                ..default()
                            },
                            BothImageComponents,
                        ))
                        .with_children(|parent| {
                            parent
                                .spawn(NodeBundle {
                                    style: NODE_BUNDLE_EMPTY_ROW_STYLE,
                                    ..default()
                                })
                                .with_children(|parent| {
                                    // image 1
                                    parent.spawn((
                                        ButtonBundle {
                                            style: Style {
                                                width: Val::Px(final_width_1),
                                                height: Val::Px(final_height_1),
                                                ..Default::default()
                                            },
                                            image: UiImage::new(texture_handle_1.clone()),
                                            ..default()
                                        },
                                        LeftImageComponent {},
                                    ));
                                });

                            parent
                                .spawn(NodeBundle {
                                    style: NODE_BUNDLE_EMPTY_ROW_STYLE,
                                    ..default()
                                })
                                .with_children(|parent| {
                                    // image 2
                                    parent.spawn((
                                        ButtonBundle {
                                            style: Style {
                                                width: Val::Px(final_width_2),
                                                height: Val::Px(final_height_2),
                                                ..Default::default()
                                            },
                                            image: UiImage::new(texture_handle_2.clone()),
                                            ..default()
                                        },
                                        RightImageComponent {},
                                    ));
                                });
                        });

                    event_writer.send(ImagesLoadedEvent);
                    return;
                }
            } else {
                println!("waiting...");
                std::thread::sleep(std::time::Duration::from_millis(50));
                return;
            }
        } else {
            println!("Handles not found in deque.");
            return;
        }
    }

    // commands
    //     .spawn((
    //         NodeBundle {
    //             style: NODE_BUNDLE_EMPTY_ROW_STYLE,
    //             ..default()
    //         },
    //         BothImageComponents,
    //     ))
    //     .with_children(|parent| {
    //         parent
    //             .spawn(NodeBundle {
    //                 style: NODE_BUNDLE_EMPTY_ROW_STYLE,
    //                 ..default()
    //             })
    //             .with_children(|parent| {
    //                 // image 1
    //                 parent.spawn((
    //                     ButtonBundle {
    //                         style: Style {
    //                             width: Val::Px(final_width_1),
    //                             height: Val::Px(final_height_1),
    //                             ..Default::default()
    //                         },
    //                         image: UiImage::new(texture_handle_1.clone()),
    //                         ..default()
    //                     },
    //                     LeftImageComponent {},
    //                 ));
    //             });
    //
    //         parent
    //             .spawn(NodeBundle {
    //                 style: NODE_BUNDLE_EMPTY_ROW_STYLE,
    //                 ..default()
    //             })
    //             .with_children(|parent| {
    //                 // image 2
    //                 parent.spawn((
    //                     ButtonBundle {
    //                         style: Style {
    //                             width: Val::Px(final_width_2),
    //                             height: Val::Px(final_height_2),
    //                             ..Default::default()
    //                         },
    //                         image: UiImage::new(texture_handle_2.clone()),
    //                         ..default()
    //                     },
    //                     RightImageComponent {},
    //                 ));
    //             });
    //     });
}

pub fn images_loaded_event_logic(
    mut event_reader: EventReader<ImagesLoadedEvent>,
    mut next_tournament_state: ResMut<NextState<TournamentState>>,
) {
    for ev in event_reader.read() {
        next_tournament_state.set(TournamentState::Deciding);
    }
}

pub fn image_clicked_decision_logic(
    mut image_clicked_event: EventReader<ImageClickedEvent>,
    mut next_tournament_state: ResMut<NextState<TournamentState>>,
    mut participants_deque_resource: ResMut<ParticipantsDeque>,
    mut pop_handles_deque_event_writer: EventWriter<PopHandlesDequeEvent>,
) {
    if let None = participants_deque_resource.deque.get(1) {
        let mut round_number = get_latest_round_number().expect("Failed to get round number");

        if let Some(sole_image) = participants_deque_resource.deque.pop_front() {
            insert_match_into_database(round_number, sole_image, 0.0 as u64, sole_image)
                .expect("Failed to insert match");
        }

        println!("None left");

        round_number += 1;
        insert_match_into_database(round_number, 0.0 as u64, 0.0 as u64, 0.0 as u64)
            .expect("Failed to insert match");

        next_tournament_state.set(TournamentState::Generating);

        return;
    }

    for ev in image_clicked_event.read() {
        println!("Before popping: {:?}", participants_deque_resource.deque);

        let image_id_1 = participants_deque_resource
            .deque
            .pop_front()
            .expect("Failed to pop");
        let image_id_2 = participants_deque_resource
            .deque
            .pop_front()
            .expect("Failed to pop");

        println!("Popped IDs: {}, {}", image_id_1, image_id_2);
        println!("After popping: {:?}", participants_deque_resource.deque);

        let round_number = get_latest_round_number().expect("Failed to get round number");

        if ev.left_image {
            set_loser_out(image_id_2).expect("Failed to set loser");
            increment_rating(image_id_1).expect("Failed to increment rating");
            insert_match_into_database(round_number, image_id_1, image_id_2, image_id_1)
                .expect("Failed to insert match");
            println!("Set winner to leftie.");
        } else {
            set_loser_out(image_id_1).expect("Failed to set loser");
            increment_rating(image_id_2).expect("Failed to increment rating");
            insert_match_into_database(round_number, image_id_2, image_id_1, image_id_2)
                .expect("Failed to insert match");
            println!("Set winner to rightie.");
        }

        pop_handles_deque_event_writer.send(PopHandlesDequeEvent);
    }
}

pub fn pop_handles_deque_event_logic(
    mut event_reader: EventReader<PopHandlesDequeEvent>,
    mut handles_deque_resource: ResMut<HandlesDeque>,
    both_image_components_query: Query<Entity, With<BothImageComponents>>,
    mut commands: Commands,
    mut next_tournament_state: ResMut<NextState<TournamentState>>,
) {
    for ev in event_reader.read() {
        println!(
            "Before popping handles deque: {:?}",
            handles_deque_resource.image_deque
        );

        handles_deque_resource.image_deque.remove(0);
        handles_deque_resource.image_deque.remove(1);

        println!(
            "After popping handles deque: {:?}",
            handles_deque_resource.image_deque
        );

        if let Ok(both_image_components_entity) = both_image_components_query.get_single() {
            commands
                .entity(both_image_components_entity)
                .despawn_recursive();
        }

        next_tournament_state.set(TournamentState::Displaying);
    }
}
