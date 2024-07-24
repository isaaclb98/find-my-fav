use std::path::PathBuf;

use bevy::asset::LoadState;
use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use rand::prelude::SliceRandom;
use rand::thread_rng;

use crate::database::*;
use crate::resources::UsedMemory;
use crate::styles::NODE_BUNDLE_EMPTY_ROW_STYLE;
use crate::tournament::components::*;
use crate::AppState;

pub fn enter_into_tournament(mut ev_generating: EventWriter<TransitionToGeneratingEvent>) {
    ev_generating.send(TransitionToGeneratingEvent);
}

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

    println!("Participants for round: {:?}", participants);

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
        let info = ParticipantInfo {
            id: participant,
            handle: None,
            loaded: false,
            errored: false,
        };

        participants_deque_resource
            .participants_deque
            .push_back(info);
        participants_to_load_resource
            .participants_to_load_deque
            .push_back(participant);
    }

    ev_loading.send(TransitionToLoadingEvent);
}

// Will run in Loading and Displaying and Deciding
pub fn load_images(
    asset_server: Res<AssetServer>,
    mut participants_to_load_resource: ResMut<ParticipantsToLoadDeque>,
    mut participants_deque_resource: ResMut<ParticipantsDeque>,
    used_memory_res: Res<UsedMemory>,
) {
    // Don't load any more images if memory used is greater than or equal to 90.0% of total memory
    if used_memory_res.0 < 90.0 {
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

            let mut image_handle: Option<Handle<Image>> = None;

            let errored = contains_non_ascii(&image_path);

            if !errored {
                image_handle = Some(asset_server.load(image_path));
            } else {
                println!("{} contains non-ASCII characters and cannot be loaded by bevy. Setting to errored.", image_path.to_string_lossy().to_string());
            }

            for participant in &mut participants_deque_resource.participants_deque {
                if participant.id == image_id {
                    participant.handle = image_handle.clone();

                    if errored {
                        participant.errored = true;
                    }
                }
            }
        }
    } else {
        println!(
            "Current memory usage is {:.2}%. Not loading any more images.",
            used_memory_res.0
        );
    }
}

// run in Loading
pub fn check_if_image_is_okay(
    mut participants_deque_resource: ResMut<ParticipantsDeque>,
    asset_server: Res<AssetServer>,
) {
    for participant in &mut participants_deque_resource.participants_deque {
        if let Some(image_handle_1) = participant.handle.clone() {
            let load_state_1 = asset_server.get_load_state(&image_handle_1);

            match load_state_1 {
                Some(LoadState::Loaded) => {
                    participant.loaded = true;
                }
                Some(LoadState::Failed(_)) => {
                    participant.errored = true;
                }
                _ => {}
            }
        }
    }
}

// run in Loading
pub fn find_first_two_loaded_indices(
    participants_deque_resource: Res<ParticipantsDeque>,
    mut indices: ResMut<Indices>,
    mut ev_displaying: EventWriter<TransitionToDisplayingEvent>,
) {
    let mut loaded_indices = Vec::new();

    for (index, participant) in participants_deque_resource
        .participants_deque
        .iter()
        .enumerate()
    {
        if participant.loaded {
            loaded_indices.push(index);
            if loaded_indices.len() == 2 {
                break;
            }
        }
    }

    if loaded_indices.len() == 2 {
        indices.index_1 = loaded_indices[0];
        indices.index_2 = loaded_indices[1];
        ev_displaying.send(TransitionToDisplayingEvent);
    } else {
        println!("Less than two participants are loaded.");
    }
}

// Will run in Displaying
pub fn display_two_loaded_images(
    mut commands: Commands,
    mut ev_deciding: EventWriter<TransitionToDecidingEvent>,
    images: Res<Assets<Image>>,
    both_image_components_query: Query<Entity, With<BothImageComponents>>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    mut participants_deque_resource: ResMut<ParticipantsDeque>,
    mut indices: ResMut<Indices>,
) {
    // despawn the preexisting images if they exist
    if let Ok(both_image_components_entity) = both_image_components_query.get_single() {
        commands
            .entity(both_image_components_entity)
            .despawn_recursive();
    }

    if let (Some(participant_1), Some(participant_2)) = (
        participants_deque_resource
            .participants_deque
            .get(indices.index_1),
        participants_deque_resource
            .participants_deque
            .get(indices.index_2),
    ) {
        if let (Some(image_1), Some(image_2)) = (
            images.get(&participant_1.handle.clone().unwrap()),
            images.get(&participant_2.handle.clone().unwrap()),
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
                                    image: UiImage::new(participant_1.handle.clone().unwrap()),
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
                                    image: UiImage::new(participant_2.handle.clone().unwrap()),
                                    ..default()
                                },
                                RightImageComponent {},
                            ));
                        });
                });

            ev_deciding.send(TransitionToDecidingEvent);
        }
    }
}

// Will run in Deciding
/// Logic to handle when the user clicks an image
pub fn image_clicked_decision_logic(
    mut ev_image_clicked: EventReader<ImageClickedEvent>,
    mut ev_resolving: EventWriter<TransitionToResolvingEvent>,
    mut participants_deque_resource: ResMut<ParticipantsDeque>,

    mut indices: ResMut<Indices>,
) {
    for ev in ev_image_clicked.read() {
        if let (participant_1, participant_2) = (
            participants_deque_resource
                .participants_deque
                .get(indices.index_1),
            participants_deque_resource
                .participants_deque
                .get(indices.index_2),
        ) {
            let round_number = get_latest_round_number().expect("Failed to get round number");

            if let (image_id_1, image_id_2) = (participant_1.unwrap().id, participant_2.unwrap().id)
            {
                if ev.left_image {
                    set_loser_out(image_id_2).expect("Failed to set loser");
                    increment_rating(image_id_1).expect("Failed to increment rating");
                    insert_match_into_database(round_number, image_id_1, image_id_2, image_id_1)
                        .expect("Failed to insert match");
                    println!("Set winner to left.");
                } else {
                    set_loser_out(image_id_1).expect("Failed to set loser");
                    increment_rating(image_id_2).expect("Failed to increment rating");
                    insert_match_into_database(round_number, image_id_2, image_id_1, image_id_2)
                        .expect("Failed to insert match");
                    println!("Set winner to right.");
                }

                ev_resolving.send(TransitionToResolvingEvent);
            }
        }
    }
}

// run in Resolving
pub fn resolve_deque(
    mut ev_generating: EventWriter<TransitionToGeneratingEvent>,
    mut ev_loading: EventWriter<TransitionToLoadingEvent>,
    mut participants_deque_resource: ResMut<ParticipantsDeque>,
    mut indices: ResMut<Indices>,
) {
    participants_deque_resource
        .participants_deque
        .remove(indices.index_2);
    participants_deque_resource
        .participants_deque
        .remove(indices.index_1);

    let mut errored_ids = Vec::new();

    // collect IDs of participants where errored is true
    for participant in &participants_deque_resource.participants_deque {
        if participant.errored {
            errored_ids.push(participant.id);
        }
    }

    for id in errored_ids {
        println!("Setting participant with id {} to out in the database", id);
        set_loser_out(id).expect("Failed to set loser out");
    }

    // remove all errored
    participants_deque_resource
        .participants_deque
        .retain(|participant| !participant.errored);

    let participants_left_in_round = participants_deque_resource.participants_deque.len();

    if participants_left_in_round < 2 {
        let mut round_number = get_latest_round_number().expect("Failed to get round number");

        if participants_left_in_round == 1 {
            if let Some(participant) = participants_deque_resource.participants_deque.pop_front() {
                let sole_image = participant.id;
                insert_match_into_database(round_number, sole_image, 0.0 as u64, sole_image)
                    .expect("Failed to insert match");
            }
        }

        round_number += 1;
        insert_match_into_database(round_number, 0.0 as u64, 0.0 as u64, 0.0 as u64)
            .expect("Failed to insert match");

        ev_generating.send(TransitionToGeneratingEvent);
    } else {
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
        println!("Transitioning to generating...");
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

pub fn transition_to_resolving_event_listener(
    mut ev_resolving: EventReader<TransitionToResolvingEvent>,
    mut next_tournament_state: ResMut<NextState<TournamentState>>,
) {
    for _ev in ev_resolving.read() {
        println!("Transitioning to resolving...");
        next_tournament_state.set(TournamentState::Resolving);
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

fn contains_non_ascii(path: &PathBuf) -> bool {
    path.to_string_lossy().chars().any(|c| !c.is_ascii())
}
