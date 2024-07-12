use crate::main_menu::MainMenuPlugin;
use crate::resources::ImageFolderPath;
use crate::systems::*;
use bevy::prelude::*;

mod database;
mod file_system;
mod main_menu;
mod resources;
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
        .init_resource::<ImageFolderPath>()
        .add_plugins(MainMenuPlugin)
        .add_systems(Startup, spawn_camera)
        .add_systems(Update, get_image_path_resource)
        .run();
}
