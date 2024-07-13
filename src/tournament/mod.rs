use crate::systems::display_tournament_state;
use crate::tournament::components::{ParticipantsDeque, TournamentState};
use crate::tournament::interactions::{
    interact_with_left_image_button, interact_with_right_image_button, ImageClickedEvent,
};
use crate::tournament::systems::{
    generate_images_to_click, get_participants_for_round, image_clicked_decision_logic,
};
use crate::AppState;
use bevy::prelude::*;

pub mod components;
pub mod interactions;
pub mod systems;

pub struct TournamentPlugin;

impl Plugin for TournamentPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<ImageClickedEvent>()
            .init_state::<TournamentState>()
            .init_resource::<ParticipantsDeque>()
            .add_systems(
                Update,
                get_participants_for_round
                    .run_if(in_state(AppState::Tournament))
                    .run_if(in_state(TournamentState::Generating)),
            )
            .add_systems(
                Update,
                generate_images_to_click
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
                display_tournament_state.run_if(in_state(AppState::Tournament)),
            );
    }
}
