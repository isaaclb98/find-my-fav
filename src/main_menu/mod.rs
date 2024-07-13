mod components;
pub mod interactions;
pub mod layout;

use crate::main_menu::interactions::{
    interact_with_folder_button, interact_with_resume_previous_button,
};
use crate::main_menu::layout::{despawn_main_menu, spawn_main_menu};
use crate::AppState;
use crate::AppState::MainMenu;
use bevy::prelude::*;

pub struct MainMenuPlugin;

impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::MainMenu), spawn_main_menu)
            .add_systems(
                Update,
                (
                    interact_with_folder_button,
                    interact_with_resume_previous_button,
                )
                    .run_if(in_state(MainMenu)),
            )
            .add_systems(OnExit(AppState::MainMenu), despawn_main_menu);
    }
}
