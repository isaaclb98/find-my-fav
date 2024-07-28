use std::path::PathBuf;

use bevy::asset::LoadState;
use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use rand::prelude::SliceRandom;
use rand::thread_rng;

use crate::database::*;
use crate::resources::UsedMemory;
use crate::speed_select::components::*;
use crate::styles::{NODE_BUNDLE_EMPTY_COLUMN_STYLE, NODE_BUNDLE_EMPTY_ROW_STYLE};
use crate::tournament::components::*;
use crate::AppState;

/// This function gets the participants' ids for a given round from the database.
pub fn get_participants_for_round(
    mut ev_loading: EventWriter<TransitionToLoadingEvent>,
    mut ev_finished: EventWriter<TransitionToFinishedEvent>,
    mut ev_despawn: EventWriter<DespawnImagesEvent>,
    mut participants_deque_resource: ResMut<ParticipantsDeque>,
    mut participants_to_load_resource: ResMut<ParticipantsToLoadDeque>,
    speed_state: Res<State<SpeedState>>,
    mut number_of_participants_for_match: ResMut<NumberOfParticipantsForMatch>,
) {
    let mut participants = get_remaining_participants().unwrap();

    let num_participants = participants.len();
    calculate_number_of_images_for_match(
        num_participants,
        &speed_state,
        &mut number_of_participants_for_match,
    );

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

/// This function loads images one-at-a-time using Bevy's asset loader.
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

            // Handle errors where the image path is incompatible with Bevy.
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

/// This function checks to see if an image was loaded by Bevy yet. If it has failed to load (corrupt image file, etc.), it will be marked as 'errored' and will be cleaned up in the 'Resolving' state.
pub fn check_if_image_has_loaded(
    mut participants_deque_resource: ResMut<ParticipantsDeque>,
    asset_server: Res<AssetServer>,
) {
    for participant in &mut participants_deque_resource.participants_deque {
        if let Some(image_handle) = participant.handle.clone() {
            let load_state = asset_server.get_load_state(&image_handle);

            match load_state {
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

/// This function finds the indices in the participants deque which have been successfully loaded.
pub fn find_first_two_loaded_indices(
    participants_deque_resource: Res<ParticipantsDeque>,
    mut indices: ResMut<ParticipantsDequeIndices>,
    mut ev_displaying: EventWriter<TransitionToDisplayingEvent>,
    number_of_participants_for_match: Res<NumberOfParticipantsForMatch>,
) {
    let mut loaded_indices = Vec::new();
    let num_images = number_of_participants_for_match.0;

    for (index, participant) in participants_deque_resource
        .participants_deque
        .iter()
        .enumerate()
    {
        if participant.loaded {
            loaded_indices.push(index);
            if loaded_indices.len() == num_images {
                break;
            }
        }
    }

    if loaded_indices.len() == num_images {
        for i in 0..num_images {
            indices.indices.push(loaded_indices[i]);
        }
        ev_displaying.send(TransitionToDisplayingEvent);
    } else {
        println!("Less than two participants are loaded.");
    }
}

/// This function displays images for the user to choose between.
pub fn display_two_loaded_images(
    mut commands: Commands,
    mut ev_deciding: EventWriter<TransitionToDecidingEvent>,
    images: Res<Assets<Image>>,
    both_image_components_query: Query<Entity, With<BothImageComponents>>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    mut participants_deque_resource: ResMut<ParticipantsDeque>,
    mut indices: ResMut<ParticipantsDequeIndices>,
    number_of_participants_for_match: Res<NumberOfParticipantsForMatch>,
) {
    // Despawn the preexisting images if they exist
    if let Ok(both_image_components_entity) = both_image_components_query.get_single() {
        commands
            .entity(both_image_components_entity)
            .despawn_recursive();
    }

    let window: &Window = window_query.get_single().unwrap();
    let window_width = window.width();
    let window_height = window.height();
    let num_images = number_of_participants_for_match.0;
    let num_rows = if num_images >= 4 { 2 } else { 1 };
    let images_per_row = (num_images as f32 / num_rows as f32).ceil() as usize;
    let target_width = window_width / images_per_row as f32;

    commands
        .spawn((
            NodeBundle {
                style: NODE_BUNDLE_EMPTY_COLUMN_STYLE,
                ..default()
            },
            BothImageComponents,
        ))
        .with_children(|parent| {
            for row in 0..num_rows {
                parent
                    .spawn(NodeBundle {
                        style: NODE_BUNDLE_EMPTY_ROW_STYLE,
                        ..default()
                    })
                    .with_children(|parent| {
                        for i in 0..images_per_row {
                            let idx = row * images_per_row + i;
                            if idx < num_images {
                                if let Some(participant) = participants_deque_resource
                                    .participants_deque
                                    .get(indices.indices[idx])
                                {
                                    if let Some(image) =
                                        images.get(&participant.handle.clone().unwrap())
                                    {
                                        let size = image.size();
                                        let width = size.x as f32;
                                        let height = size.y as f32;
                                        let image_aspect_ratio = width / height;

                                        let target_height = target_width / image_aspect_ratio;
                                        let (final_width, final_height) = if target_height
                                            > window_height / num_rows as f32
                                        {
                                            let adjusted_height = window_height / num_rows as f32;
                                            let adjusted_width =
                                                adjusted_height * image_aspect_ratio;
                                            (adjusted_width, adjusted_height)
                                        } else {
                                            (target_width, target_height)
                                        };

                                        parent
                                            .spawn(NodeBundle {
                                                style: NODE_BUNDLE_EMPTY_ROW_STYLE,
                                                ..default()
                                            })
                                            .with_children(|parent| {
                                                // Image
                                                parent.spawn((
                                                    ButtonBundle {
                                                        style: Style {
                                                            width: Val::Px(final_width),
                                                            height: Val::Px(final_height),
                                                            ..Default::default()
                                                        },
                                                        image: UiImage::new(
                                                            participant.handle.clone().unwrap(),
                                                        ),
                                                        ..default()
                                                    },
                                                    ImageComponent {
                                                        index: indices.indices[idx],
                                                        id: participant.id,
                                                    },
                                                ));
                                            });
                                    }
                                }
                            }
                        }
                    });
            }
        });

    ev_deciding.send(TransitionToDecidingEvent);
}

/// This function is the logic that occurs when the user clicks an image.
pub fn image_clicked_decision_logic(
    mut ev_image_clicked: EventReader<ImageClickedEvent>,
    mut ev_resolving: EventWriter<TransitionToResolvingEvent>,
    mut participants_deque_resource: ResMut<ParticipantsDeque>,
    mut indices: ResMut<ParticipantsDequeIndices>,
    number_of_participants_for_match: Res<NumberOfParticipantsForMatch>,
) {
    for ev in ev_image_clicked.read() {
        let id = ev.id;

        let round_number = get_latest_round_number().expect("Failed to get round number");

        for _ in 0..number_of_participants_for_match.0 {
            let loser_id = participants_deque_resource
                .participants_deque
                .pop_front()
                .unwrap()
                .id;
            if loser_id != id {
                set_loser_out(loser_id).expect("Failed to set loser");
                increment_rating(id).expect("Failed to increment rating");
                insert_match_into_database(round_number, id, loser_id, id)
                    .expect("Failed to insert match");
            }
        }

        ev_resolving.send(TransitionToResolvingEvent);
    }
}

/// This function works to resolve the state of the tournament. It removes errored participants, and checks if a new round is needed (less than two participants left in round).
pub fn resolve_deque(
    mut ev_generating: EventWriter<TransitionToGeneratingEvent>,
    mut ev_loading: EventWriter<TransitionToLoadingEvent>,
    mut participants_deque_resource: ResMut<ParticipantsDeque>,
    mut indices: ResMut<ParticipantsDequeIndices>,
) {
    indices.indices.clear();

    let mut errored_ids = Vec::new();

    // Collect IDs of participants where errored is true
    for participant in &participants_deque_resource.participants_deque {
        if participant.errored {
            errored_ids.push(participant.id);
        }
    }

    for id in errored_ids {
        println!("Setting participant with id {} to out in the database", id);
        set_loser_out(id).expect("Failed to set loser out");
    }

    // Remove all errored participants
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

pub fn enter_into_tournament(mut ev_generating: EventWriter<TransitionToGeneratingEvent>) {
    ev_generating.send(TransitionToGeneratingEvent);
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

// Bevy unfortunately has problems loading file paths with non-ASCII characters
fn contains_non_ascii(path: &PathBuf) -> bool {
    path.to_string_lossy().chars().any(|c| !c.is_ascii())
}

fn calculate_number_of_images_for_match(
    num_participants: usize,
    speed_state: &Res<State<SpeedState>>,
    mut number_of_participants_for_match: &mut ResMut<NumberOfParticipantsForMatch>,
) {
    match speed_state.get() {
        SpeedState::Fast => match num_participants {
            0..=100 => number_of_participants_for_match.0 = 2,
            100..=250 => number_of_participants_for_match.0 = 3,
            251..=500 => number_of_participants_for_match.0 = 4,
            501..=1000 => number_of_participants_for_match.0 = 6,
            1001..=2000 => number_of_participants_for_match.0 = 8,
            2001..=4000 => number_of_participants_for_match.0 = 10,
            4001..=6000 => number_of_participants_for_match.0 = 12,
            _ => number_of_participants_for_match.0 = 16,
        },
        _ => number_of_participants_for_match.0 = 2,
    }
}
