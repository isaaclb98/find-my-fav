use std::path::PathBuf;

use bevy::asset::LoadState;
use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use rand::prelude::SliceRandom;
use rand::thread_rng;

use crate::database::*;
use crate::styles::NODE_BUNDLE_EMPTY_ROW_STYLE;
use crate::tournament::components::*;
use crate::AppState;

// Will run in Generating
/// Get the participants ids for a given round from the database
pub fn get_participants_for_round(
    mut ev_loading: EventWriter<TransitionToLoadingEvent>,
    mut ev_finished: EventWriter<TransitionToFinishedEvent>,
    mut ev_despawn: EventWriter<DespawnImagesEvent>,
    mut participants_deque_resource: ResMut<ParticipantsDeque>,
    mut participants_to_load_resource: ResMut<ParticipantsToLoadDeque>,
) {
    let mut participants = get_remaining_participants().unwrap();

    // Tournament over
    if participants.len() == 1 {
        println!("The tournament is now over.");

        ev_despawn.send(DespawnImagesEvent);
        ev_finished.send(TransitionToFinishedEvent);
        return;
    }

    let mut rng = thread_rng();
    participants.shuffle(&mut rng);

    for participant in participants {
        participants_deque_resource
            .participants_deque
            .push_back(participant);
        participants_to_load_resource
            .participants_to_load_deque
            .push_back(participant);
    }

    ev_loading.send(TransitionToLoadingEvent);
}

// Will run in Loading and Displaying
/// Loads one image at a time
pub fn load_images(
    asset_server: Res<AssetServer>,
    mut participants_to_load_resource: ResMut<ParticipantsToLoadDeque>,
    mut participants_deque_resource: ResMut<ParticipantsDeque>,
) {
    let image_id_option: Option<u64> = {
        if let Some(image_id_1) = participants_to_load_resource
            .participants_to_load_deque
            .pop_front()
        {
            Some(image_id_1)
        } else {
            None
        }
    };

    if let Some(image_id) = image_id_option {
        let image_path = get_image_path_from_database(image_id)
            .expect("Could not load the image path from the database.");

        let image_path = sanitize_filename(image_path);

        let image_handle: Handle<Image> = asset_server.load(image_path);

        participants_deque_resource
            .handles_deque
            .push_back(image_handle);
    } else {
    }
}

// Will run in Loading
/// Will check if two images at the front of the deque are loaded yet. Hardcoded to be two, which sucks.
pub fn check_if_two_images_are_loaded(
    mut ev_loaded_images: EventWriter<TwoImagesLoadedEvent>,
    mut ev_displaying: EventWriter<TransitionToDisplayingEvent>,
    mut image_error_event: EventWriter<ImageErrorEvent>,
    mut participants_deque_resource: ResMut<ParticipantsDeque>,
    asset_server: Res<AssetServer>,
) {
    // println!(
    //     "handles_deque: {:?}",
    //     participants_deque_resource.handles_deque
    // );

    println!(
        "check_if_two_images_are_loaded : {:?}",
        participants_deque_resource
    );

    if let (Some(image_handle_1), Some(image_handle_2)) = (
        participants_deque_resource.handles_deque.get(0),
        participants_deque_resource.handles_deque.get(1),
    ) {
        // println!(
        //     "check_if_two_images_are_loaded: \nimage_handle_1: {:?}\nimage_handle_2{:?}",
        //     image_handle_1, image_handle_2
        // );

        let load_state_1 = asset_server.get_load_state(image_handle_1);
        let load_state_2 = asset_server.get_load_state(image_handle_2);

        match (load_state_1, load_state_2) {
            (Some(LoadState::Loaded), Some(LoadState::Loaded)) => {
                ev_loaded_images.send(TwoImagesLoadedEvent);
                ev_displaying.send(TransitionToDisplayingEvent);
            }
            (Some(LoadState::Failed(_)), _) => {
                image_error_event.send(ImageErrorEvent {
                    left_image_fail: true,
                });
            }
            (_, Some(LoadState::Failed(_))) => {
                image_error_event.send(ImageErrorEvent {
                    left_image_fail: false,
                });
            }
            _ => {}
        }
    }
}

// Will run in Displaying
/// Display the two images that have been loaded
pub fn display_two_loaded_images(
    mut commands: Commands,
    mut ev_loaded_images: EventReader<TwoImagesLoadedEvent>,
    mut ev_deciding: EventWriter<TransitionToDecidingEvent>,
    mut ev_loading: EventWriter<TransitionToLoadingEvent>,
    images: Res<Assets<Image>>,
    both_image_components_query: Query<Entity, With<BothImageComponents>>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    mut participants_deque_resource: ResMut<ParticipantsDeque>,
) {
    for ev in ev_loaded_images.read() {
        // despawn the preexisting images if they exist
        if let Ok(both_image_components_entity) = both_image_components_query.get_single() {
            commands
                .entity(both_image_components_entity)
                .despawn_recursive();
        }

        println!(
            "display_two_loaded_images : {:?}",
            participants_deque_resource
        );

        if let (Some(image_handle_1), Some(image_handle_2)) = (
            participants_deque_resource.handles_deque.pop_front(),
            participants_deque_resource.handles_deque.pop_front(),
        ) {
            if let (Some(image_1), Some(image_2)) =
                (images.get(&image_handle_1), images.get(&image_handle_2))
            {
                let size_1 = image_1.size();
                let size_2 = image_2.size();

                let width_1 = size_1.x as f32;
                let height_1 = size_1.y as f32;
                let width_2 = size_2.x as f32;
                let height_2 = size_2.y as f32;

                let image_aspect_ratio_1 = width_1 / height_1;
                let image_aspect_ratio_2 = width_2 / height_2;

                let window: &Window = window_query.get_single().unwrap();
                let window_width = window.width();
                let window_height = window.height();

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
                                        image: UiImage::new(image_handle_1.clone()),
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
                                        image: UiImage::new(image_handle_2.clone()),
                                        ..default()
                                    },
                                    RightImageComponent {},
                                ));
                            });
                    });

                ev_deciding.send(TransitionToDecidingEvent);
                break;
            }
            println!("Error in Display: display_two_loaded_images.");
            ev_loading.send(TransitionToLoadingEvent);
        }
    }
}

// Will run in Deciding
/// Logic to handle when the user clicks an image
pub fn image_clicked_decision_logic(
    mut ev_image_clicked: EventReader<ImageClickedEvent>,
    mut ev_new_round_needed: EventWriter<NewRoundNeeded>,
    mut ev_loading: EventWriter<TransitionToLoadingEvent>,
    mut participants_deque_resource: ResMut<ParticipantsDeque>,
) {
    for ev in ev_image_clicked.read() {
        let image_id_1 = participants_deque_resource
            .participants_deque
            .pop_front()
            .expect("Failed to pop");
        let image_id_2 = participants_deque_resource
            .participants_deque
            .pop_front()
            .expect("Failed to pop");

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

        // check if 1 or 0 left in queue afterwards
        let participants_left_in_round = participants_deque_resource.participants_deque.len();
        if participants_left_in_round < 2 {
            ev_new_round_needed.send(NewRoundNeeded);
            break;
        }

        ev_loading.send(TransitionToLoadingEvent);
    }
}

pub fn _display_current_tournament_state(tournament_state: Res<State<TournamentState>>) {
    print!("{:?} ", tournament_state);
}

pub fn transition_to_generating_event_listener(
    mut ev_generating: EventReader<TransitionToGeneratingEvent>,
    mut next_tournament_state: ResMut<NextState<TournamentState>>,
) {
    for _ev in ev_generating.read() {
        println!("Generating new round...");
        next_tournament_state.set(TournamentState::Generating);
    }
}

pub fn transition_to_loading_event_listener(
    mut ev_loading: EventReader<TransitionToLoadingEvent>,
    mut next_tournament_state: ResMut<NextState<TournamentState>>,
) {
    for _ev in ev_loading.read() {
        println!("Transitioning to loading...");
        next_tournament_state.set(TournamentState::Loading);
    }
}

pub fn transition_to_displaying_event_listener(
    mut ev_displaying: EventReader<TransitionToDisplayingEvent>,
    mut next_tournament_state: ResMut<NextState<TournamentState>>,
) {
    for _ev in ev_displaying.read() {
        println!("Transitioning to displaying...");
        next_tournament_state.set(TournamentState::Displaying);
    }
}

pub fn transition_to_deciding_event_listener(
    mut ev_deciding: EventReader<TransitionToDecidingEvent>,
    mut next_tournament_state: ResMut<NextState<TournamentState>>,
) {
    for _ev in ev_deciding.read() {
        println!("Transitioning to deciding...");
        next_tournament_state.set(TournamentState::Deciding);
    }
}

pub fn transition_to_finished_event_listener(
    mut ev_finished: EventReader<TransitionToFinishedEvent>,
    mut next_app_state: ResMut<NextState<AppState>>,
) {
    for _ev in ev_finished.read() {
        next_app_state.set(AppState::Finished);
    }
}

pub fn despawn_images_event_listener(
    mut commands: Commands,
    mut ev_despawn: EventReader<DespawnImagesEvent>,
    both_image_components_query: Query<Entity, With<BothImageComponents>>,
) {
    for _ev in ev_despawn.read() {
        if let Ok(both_image_components_entity) = both_image_components_query.get_single() {
            commands
                .entity(both_image_components_entity)
                .despawn_recursive();
        }
    }
}

fn sanitize_filename(path: PathBuf) -> PathBuf {
    let sanitized: String = path
        .to_string_lossy()
        .chars()
        .filter(|c| c.is_ascii())
        .collect();

    PathBuf::from(sanitized)
}

pub fn image_error_event_listener(
    mut image_error_event: EventReader<ImageErrorEvent>,
    mut ev_new_round_needed: EventWriter<NewRoundNeeded>,

    mut participants_deque_resource: ResMut<ParticipantsDeque>,
) {
    for ev in image_error_event.read() {
        let round_number = get_latest_round_number().expect("Failed to get round number");

        println!(
            "participants_deque len before error: {:?}",
            participants_deque_resource.participants_deque.len()
        );
        println!(
            "handles_deque len before error: {:?}",
            participants_deque_resource.handles_deque.len()
        );

        let image_id_1 = participants_deque_resource
            .participants_deque
            .pop_front()
            .expect("Failed to pop");
        let image_id_2 = participants_deque_resource
            .participants_deque
            .pop_front()
            .expect("Failed to pop");

        participants_deque_resource.handles_deque.pop_front();
        participants_deque_resource.handles_deque.pop_front();

        if ev.left_image_fail == true {
            set_loser_out(image_id_1).expect("Failed to set loser");
            insert_match_into_database(round_number, image_id_1, image_id_2, image_id_2)
                .expect("Failed to insert match");
        } else if ev.left_image_fail == false {
            set_loser_out(image_id_2).expect("Failed to set loser");
            insert_match_into_database(round_number, image_id_1, image_id_2, image_id_1)
                .expect("Failed to insert match");
        }
        println!("Caught an error.");

        // handle for when 0 or 1
        if participants_deque_resource.participants_deque.len() < 2 {
            ev_new_round_needed.send(NewRoundNeeded);
        }
    }
}

pub fn new_round_needed_event_listener(
    mut ev_new_round_needed: EventReader<NewRoundNeeded>,
    mut ev_generating: EventWriter<TransitionToGeneratingEvent>,
    mut participants_deque_resource: ResMut<ParticipantsDeque>,
) {
    for _ev in ev_new_round_needed.read() {
        let participants_left_in_round = participants_deque_resource.participants_deque.len();
        let mut round_number = get_latest_round_number().expect("Failed to get round number");

        if participants_left_in_round == 1 {
            if let Some(sole_image) = participants_deque_resource.participants_deque.pop_front() {
                insert_match_into_database(round_number, sole_image, 0.0 as u64, sole_image)
                    .expect("Failed to insert match");
            }
            participants_deque_resource.handles_deque.pop_front();
        }

        round_number += 1;
        insert_match_into_database(round_number, 0.0 as u64, 0.0 as u64, 0.0 as u64)
            .expect("Failed to insert match");

        ev_generating.send(TransitionToGeneratingEvent);
    }
}
