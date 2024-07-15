use crate::tournament::components::{
    HandlesDeque, ImagesLoadedEvent, ParticipantsDeque, PopHandlesDequeEvent, TournamentState,
};
use crate::tournament::interactions::{
    interact_with_left_image_button, interact_with_right_image_button,
};
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
        app.add_event::<ImageClickedEvent>()
            .add_event::<ImagesLoadedEvent>()
            .add_event::<PopHandlesDequeEvent>()
            .init_state::<TournamentState>()
            .init_resource::<ParticipantsDeque>()
            .init_resource::<HandlesDeque>()
            .add_systems(
                Update,
                get_participants_for_round
                    .run_if(in_state(AppState::Tournament))
                    .run_if(in_state(TournamentState::Generating)),
            )
            .add_systems(
                Update,
                (load_images, display_images, images_loaded_event_logic)
                    .run_if(in_state(AppState::Tournament))
                    .run_if(in_state(TournamentState::Displaying)),
            )
            .add_systems(
                Update,
                (
                    interact_with_left_image_button,
                    interact_with_right_image_button,
                    image_clicked_decision_logic,
                    pop_handles_deque_event_logic,
                )
                    .run_if(in_state(AppState::Tournament))
                    .run_if(in_state(TournamentState::Deciding)),
            );
    }
}
