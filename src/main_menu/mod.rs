mod components;
pub mod interactions;
pub mod layout;
mod systems;

use crate::main_menu::interactions::{
    interact_with_folder_button, interact_with_resume_previous_button,
};
use crate::main_menu::layout::{despawn_main_menu, spawn_main_menu};
use crate::main_menu::systems::ResumedState;
use crate::AppState;
use crate::AppState::MainMenu;
use bevy::prelude::*;
use systems::initialize_database_if_image_folder_path;

pub struct MainMenuPlugin;

impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<ResumedState>()
            .add_systems(OnEnter(AppState::MainMenu), spawn_main_menu)
            .add_systems(
                Update,
                (
                    interact_with_folder_button,
                    interact_with_resume_previous_button,
                )
                    .run_if(in_state(MainMenu)),
            )
            .add_systems(OnExit(AppState::MainMenu), despawn_main_menu)
            .add_systems(
                OnEnter(AppState::SpeedSelect),
                initialize_database_if_image_folder_path.run_if(in_state(ResumedState::New)),
            );
    }
}
