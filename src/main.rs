use crate::main_menu::MainMenuPlugin;
use crate::resources::ImageFolderPath;
use crate::speed_select::SpeedSelectPlugin;
use crate::systems::*;
use crate::AppState::{SpeedSelect, Tournament};
use bevy::prelude::*;

mod components;
mod database;
mod file_system;
mod main_menu;
mod resources;
mod speed_select;
mod styles;
mod systems;
mod tournament;

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
        .init_state::<TournamentState>()
        .init_resource::<ImageFolderPath>()
        .init_resource::<ParticipantsDeque>()
        .add_plugins(MainMenuPlugin)
        .add_plugins(SpeedSelectPlugin)
        .add_systems(Startup, spawn_camera)
        .add_systems(
            OnEnter(SpeedSelect),
            initialize_database_if_image_folder_path,
        )
        .add_systems(
            OnEnter(Tournament),
            get_participants_for_round.run_if(in_state(TournamentState::Generating)),
        )
        .add_systems(
            Update,
            generate_images_to_click.run_if(in_state(TournamentState::Displaying)),
        )
        .run();
}
