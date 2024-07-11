use crate::main_menu::MainMenuPlugin;
use crate::systems::spawn_camera;
use bevy::prelude::*;

mod database;
mod file_system;
mod main_menu;
mod systems;

#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
pub enum AppState {
    #[default]
    MainMenu,
    SpeedSelect,
    Tournament,
    Finished,
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .init_state::<AppState>()
        .add_plugins(MainMenuPlugin)
        .add_systems(Startup, spawn_camera)
        .run();
}
