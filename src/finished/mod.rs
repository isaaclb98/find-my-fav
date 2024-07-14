mod components;
mod interactions;
mod layout;
mod systems;

use crate::finished::interactions::{
    interact_with_new_folder_button, interact_with_start_over_button,
};
use crate::finished::layout::{despawn_finished_screen, spawn_finished_screen};
use crate::finished::systems::generate_favourites_folder;
use crate::main_menu::layout::spawn_main_menu;
use crate::AppState;
use bevy::prelude::*;

pub struct FinishedPlugin;

impl Plugin for FinishedPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            OnEnter(AppState::Finished),
            (spawn_finished_screen, generate_favourites_folder),
        )
        .add_systems(
            Update,
            (
                interact_with_new_folder_button,
                interact_with_start_over_button,
            )
                .run_if(in_state(AppState::Finished)),
        )
        .add_systems(OnExit(AppState::Finished), despawn_finished_screen);
    }
}
