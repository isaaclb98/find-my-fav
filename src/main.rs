use crate::finished::FinishedPlugin;
use crate::main_menu::MainMenuPlugin;
use crate::resources::ImageFolderPath;
use crate::speed_select::SpeedSelectPlugin;
use crate::systems::*;
use crate::tournament::TournamentPlugin;
use bevy::prelude::*;
use bevy::window::{WindowMode, WindowTheme};
mod database;
mod file_system;
mod finished;
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
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "FindMyFav".into(),
                window_theme: Some(WindowTheme::Dark),
                mode: WindowMode::Windowed,
                ..default()
            }),
            ..default()
        }))
        .add_plugins(MainMenuPlugin)
        .add_plugins(SpeedSelectPlugin)
        .add_plugins(TournamentPlugin)
        .add_plugins(FinishedPlugin)
        .init_state::<AppState>()
        .init_resource::<ImageFolderPath>()
        .add_systems(Startup, spawn_camera)
        .add_systems(Update, show_app_state)
        .run();
}
