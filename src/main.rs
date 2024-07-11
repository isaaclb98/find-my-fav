use bevy::prelude::*;
mod file_system;
mod database;
mod main_menu;

#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
pub enum AppState {
    #[default]
    MainMenu,
    Tournament,
    Finished,
}

fn main() {}

