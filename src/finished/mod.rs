use bevy::prelude::*;

use crate::finished::components::{FavouritesFolderResource, FolderGeneratedState};
use crate::finished::interactions::{
    interact_with_new_folder_button, interact_with_start_over_button,
};
use crate::finished::layout::{despawn_finished_screen, spawn_finished_screen};
use crate::finished::systems::generate_favourites_folder;
use crate::AppState;

mod components;
mod interactions;
mod layout;
mod systems;

pub struct FinishedPlugin;

impl Plugin for FinishedPlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<FolderGeneratedState>()
            .init_resource::<FavouritesFolderResource>()
            .add_systems(
                OnEnter(AppState::Finished),
                (spawn_finished_screen, generate_favourites_folder),
            )
            .add_systems(
                Update,
                interact_with_start_over_button.run_if(in_state(AppState::Finished)),
            )
            .add_systems(
                Update,
                (interact_with_new_folder_button,)
                    .run_if(in_state(AppState::Finished))
                    .run_if(in_state(FolderGeneratedState::Generated)),
            )
            .add_systems(OnExit(AppState::Finished), despawn_finished_screen);
    }
}
