use crate::main_menu::MainMenuPlugin;
use crate::resources::ImageFolderPath;
use crate::speed_select::SpeedSelectPlugin;
use crate::systems::*;
use crate::AppState::{SpeedSelect, Tournament};
use bevy::prelude::*;

mod database;
mod file_system;
mod main_menu;
mod resources;
mod speed_select;
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
        .add_plugins(SpeedSelectPlugin)
        .add_systems(Startup, spawn_camera)
        .add_systems(
            OnEnter(SpeedSelect),
            initialize_database_if_image_folder_path,
        )
        .add_systems(OnEnter(Tournament), get_two_participants)
        .add_event::<ImageLoadedEvent>()
        .add_systems(Update, check_image_loaded)
        .run();
}
