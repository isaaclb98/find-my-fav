use crate::database::*;
use crate::styles::NODE_BUNDLE_EMPTY_ROW_STYLE;
use crate::tournament::components::*;
use crate::AppState;
use bevy::asset::LoadState;
use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use rand::prelude::SliceRandom;
use rand::thread_rng;
use std::time::Instant;

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

// Will run in Loading and possibly Deciding
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
    mut ev_pop_two_handles: EventWriter<PopTwoHandlesEvent>,
    mut ev_displaying: EventWriter<TransitionToDisplayingEvent>,
    mut participants_deque_resource: ResMut<ParticipantsDeque>,
    asset_server: Res<AssetServer>,
) {
    // println!(
    //     "handles_deque: {:?}",
    //     participants_deque_resource.handles_deque
    // );

    if let (Some(image_handle_1), Some(image_handle_2)) = (
        participants_deque_resource.handles_deque.get(0),
        participants_deque_resource.handles_deque.get(1),
    ) {
        // println!(
        //     "check_if_two_images_are_loaded: \nimage_handle_1: {:?}\nimage_handle_2{:?}",
        //     image_handle_1, image_handle_2
        // );
        if let (Some(LoadState::Loaded), Some(LoadState::Loaded)) = (
            asset_server.get_load_state(image_handle_1),
            asset_server.get_load_state(image_handle_2),
        ) {
            ev_loaded_images.send(TwoImagesLoadedEvent {
                image_handle_1: image_handle_1.clone(),
                image_handle_2: image_handle_2.clone(),
            });
            ev_pop_two_handles.send(PopTwoHandlesEvent);
            ev_displaying.send(TransitionToDisplayingEvent);
        }
    }
}

// Will run in Displaying
/// Display the two images that have been loaded
pub fn display_two_loaded_images(
    mut commands: Commands,
    mut ev_loaded_images: EventReader<TwoImagesLoadedEvent>,
    mut ev_deciding: EventWriter<TransitionToDecidingEvent>,
    images: Res<Assets<Image>>,
    both_image_components_query: Query<Entity, With<BothImageComponents>>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    for ev in ev_loaded_images.read() {
        println!("{:?}\n{:?}", ev.image_handle_1, ev.image_handle_2);

        // despawn the preexisting images if they exist
        if let Ok(both_image_components_entity) = both_image_components_query.get_single() {
            commands
                .entity(both_image_components_entity)
                .despawn_recursive();
        }

        if let (Some(image_1), Some(image_2)) = (
            images.get(&ev.image_handle_1),
            images.get(&ev.image_handle_2),
        ) {
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
                                    image: UiImage::new(ev.image_handle_1.clone()),
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
                                    image: UiImage::new(ev.image_handle_2.clone()),
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
    }
}

// Will run in Deciding
pub fn image_clicked_decision_logic(
    mut ev_image_clicked: EventReader<ImageClickedEvent>,
    mut ev_generating: EventWriter<TransitionToGeneratingEvent>,
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

        let mut round_number = get_latest_round_number().expect("Failed to get round number");

        if ev.left_image {
            set_loser_out(image_id_2).expect("Failed to set loser");
            increment_rating(image_id_1).expect("Failed to increment rating");
            insert_match_into_database(round_number, image_id_1, image_id_2, image_id_1)
                .expect("Failed to insert match");
            println!("Set winner to leftie.");
            ev_loading.send(TransitionToLoadingEvent);
        } else {
            set_loser_out(image_id_1).expect("Failed to set loser");
            increment_rating(image_id_2).expect("Failed to increment rating");
            insert_match_into_database(round_number, image_id_2, image_id_1, image_id_2)
                .expect("Failed to insert match");
            println!("Set winner to rightie.");
            ev_loading.send(TransitionToLoadingEvent);
        }
        // check if 1 or 0 left in queue afterwards
        let participants_left_in_round = participants_deque_resource.participants_deque.len();
        if participants_left_in_round < 2 {
            if participants_left_in_round == 1 {
                if let Some(sole_image) = participants_deque_resource.participants_deque.pop_front()
                {
                    insert_match_into_database(round_number, sole_image, 0.0 as u64, sole_image)
                        .expect("Failed to insert match");
                }
                participants_deque_resource.handles_deque.pop_front();
            }
            println!("None left");

            round_number += 1;
            insert_match_into_database(round_number, 0.0 as u64, 0.0 as u64, 0.0 as u64)
                .expect("Failed to insert match");

            ev_generating.send(TransitionToGeneratingEvent);
            break;
        }
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
        println!("Generated participants for round.");
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

pub fn pop_two_handles_event_listener(
    mut pop_two_handles_event: EventReader<PopTwoHandlesEvent>,
    mut participants_deque_resource: ResMut<ParticipantsDeque>,
) {
    for _ev in pop_two_handles_event.read() {
        participants_deque_resource.handles_deque.pop_front();
        participants_deque_resource.handles_deque.pop_front();
    }
}
