use crate::database::{
    get_image_path_from_database, get_latest_round_number, get_remaining_participants,
    increment_rating, insert_match_into_database, set_loser_out,
};
use crate::styles::NODE_BUNDLE_EMPTY_ROW_STYLE;
use crate::tournament::components::{
    BothImageComponents, LeftImageComponent, ParticipantsDeque, RightImageComponent,
    TournamentState,
};
use crate::tournament::interactions::ImageClickedEvent;
use crate::AppState;
use bevy::asset::{AssetServer, Handle};
use bevy::hierarchy::{BuildChildren, DespawnRecursiveExt};
use bevy::prelude::{
    default, ButtonBundle, Commands, Entity, EventReader, Image, NextState, NodeBundle, Query, Res,
    ResMut, Style, UiImage, Val, Window, With,
};
use bevy::window::PrimaryWindow;
use image::GenericImageView;
use rand::prelude::SliceRandom;
use rand::thread_rng;
use std::path::{Path, PathBuf};
use std::time::Instant;

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

pub fn generate_images_to_click(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    mut participants_deque_resource: ResMut<ParticipantsDeque>,
    mut next_tournament_state: ResMut<NextState<TournamentState>>,
) {
    let start_time = Instant::now();

    let window: &Window = window_query.get_single().unwrap();
    let window_width = window.width();
    let window_height = window.height();

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

        let mut error_occurred = false;
        let image_1 = image::open(&image_path_1).unwrap_or_else(|_| {
            let round_number = get_latest_round_number().expect("Failed to get round number");

            participants_deque_resource
                .deque
                .remove(0)
                .expect("Failed to remove from deque");

            set_loser_out(image_id_1).expect("Failed to set loser");
            insert_match_into_database(round_number, image_id_1, image_id_2, image_id_2)
                .expect("Failed to insert match");

            println!("Handled an error for an image. ID: {}", image_id_1);

            error_occurred = true;

            Default::default()
        });

        if error_occurred {
            return;
        }

        let image_2 = image::open(&image_path_2).unwrap_or_else(|_| {
            let round_number = get_latest_round_number().expect("Failed to get round number");

            participants_deque_resource
                .deque
                .remove(1)
                .expect("Failed to remove from deque");

            set_loser_out(image_id_2).expect("Failed to set loser");
            insert_match_into_database(round_number, image_id_1, image_id_2, image_id_1)
                .expect("Failed to insert match");

            println!("Handled an error for an image. ID: {}", image_id_2);

            error_occurred = true;

            Default::default()
        });

        if error_occurred {
            return;
        }

        let duration = start_time.elapsed();
        println!("getting the images took {:?}", duration);

        // Get the image dimensions
        let (width_1, height_1) = image_1.dimensions();
        println!("Image 1 dimensions: {}x{}", width_1, height_1);
        let (width_2, height_2) = image_2.dimensions();
        println!("Image 2 dimensions: {}x{}", width_2, height_2);

        let image_aspect_ratio_1 = width_1 as f32 / height_1 as f32;
        let image_aspect_ratio_2 = width_2 as f32 / height_2 as f32;

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

        // Load the image as a Bevy asset
        let texture_handle_1: Handle<Image> = asset_server.load(image_path_1);
        let texture_handle_2: Handle<Image> = asset_server.load(image_path_2);

        let start_time = Instant::now();

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
                                image: UiImage::new(texture_handle_1),
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
                                image: UiImage::new(texture_handle_2),
                                ..default()
                            },
                            RightImageComponent {},
                        ));
                    });
            });
    }

    next_tournament_state.set(TournamentState::Deciding);
}

pub fn image_clicked_decision_logic(
    mut commands: Commands,
    mut image_clicked_event: EventReader<ImageClickedEvent>,
    mut participants_deque_resource: ResMut<ParticipantsDeque>,
    mut next_tournament_state: ResMut<NextState<TournamentState>>,
    both_image_components_query: Query<Entity, With<BothImageComponents>>,
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

        if let Ok(both_image_components_entity) = both_image_components_query.get_single() {
            commands
                .entity(both_image_components_entity)
                .despawn_recursive();
        }

        next_tournament_state.set(TournamentState::Displaying);
    }
}
