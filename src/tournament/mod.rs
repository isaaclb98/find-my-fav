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
            .add_event::<TwoImagesLoadedEvent>()
            .add_event::<PopTwoHandlesEvent>()
            .add_event::<ImageClickedEvent>()
            .add_event::<ImageErrorEvent>()
            .add_event::<NewRoundNeeded>()
            .init_state::<TournamentState>()
            .init_resource::<ParticipantsDeque>()
            .init_resource::<ParticipantsToLoadDeque>()
            .add_systems(
                Update,
                get_participants_for_round
                    .run_if(in_state(AppState::Tournament))
                    .run_if(in_state(TournamentState::Generating)),
            )
            .add_systems(
                Update,
                (check_if_two_images_are_loaded, load_images)
                    .after(image_error_event_listener)
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
            .add_systems(
                Update,
                (
                    transition_to_generating_event_listener,
                    transition_to_loading_event_listener,
                    transition_to_displaying_event_listener,
                    transition_to_deciding_event_listener,
                    transition_to_finished_event_listener,
                    despawn_images_event_listener,
                    image_error_event_listener,
                    new_round_needed_event_listener,
                )
                    .run_if(in_state(AppState::Tournament)),
            );
    }
}
