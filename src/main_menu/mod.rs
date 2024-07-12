mod components;
mod styles;
mod systems;

use crate::AppState;
use crate::AppState::MainMenu;
use bevy::prelude::*;
use systems::interactions::*;
use systems::layout::*;

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
