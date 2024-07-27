use crate::tournament::components::*;
use crate::tournament::interactions::*;
use crate::tournament::systems::*;
use crate::AppState;

use bevy::prelude::*;
use components::ImageClickedEvent;

pub mod components;
pub mod interactions;
pub mod systems;

pub struct TournamentPlugin;

impl Plugin for TournamentPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<TransitionToGeneratingEvent>()
            .add_event::<TransitionToLoadingEvent>()
            .add_event::<TransitionToDisplayingEvent>()
            .add_event::<TransitionToDecidingEvent>()
            .add_event::<TransitionToFinishedEvent>()
            .add_event::<DespawnImagesEvent>()
            .add_event::<ImageClickedEvent>()
            .add_event::<TransitionToResolvingEvent>()
            .init_state::<TournamentState>()
            .init_resource::<ParticipantsDeque>()
            .init_resource::<ParticipantsToLoadDeque>()
            .init_resource::<ParticipantsDequeIndices>()
            .init_resource::<NumberOfParticipantsForMatch>()
            .add_systems(
                Update,
                enter_into_tournament.run_if(in_state(TournamentState::Entering)),
            )
            .add_systems(
                OnEnter(TournamentState::Generating),
                get_participants_for_round.run_if(in_state(AppState::Tournament)),
            )
            .add_systems(
                Update,
                (
                    check_if_image_has_loaded,
                    find_first_two_loaded_indices,
                    load_images,
                )
                    .run_if(in_state(AppState::Tournament))
                    .run_if(in_state(TournamentState::Loading)),
            )
            .add_systems(
                Update,
                (display_two_loaded_images, load_images)
                    .run_if(in_state(AppState::Tournament))
                    .run_if(in_state(TournamentState::Displaying)),
            )
            .add_systems(
                Update,
                (
                    interact_with_left_image_button,
                    interact_with_right_image_button,
                    image_clicked_decision_logic,
                )
                    .run_if(in_state(AppState::Tournament))
                    .run_if(in_state(TournamentState::Deciding)),
            )
            .add_systems(OnEnter(TournamentState::Resolving), resolve_deque)
            .add_systems(
                Update,
                (
                    transition_to_generating_event_listener,
                    transition_to_loading_event_listener,
                    transition_to_displaying_event_listener,
                    transition_to_deciding_event_listener,
                    transition_to_resolving_event_listener,
                    transition_to_finished_event_listener,
                    despawn_images_event_listener,
                )
                    .run_if(in_state(AppState::Tournament)),
            );
    }
}
