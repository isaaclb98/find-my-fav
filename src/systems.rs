use crate::database::initialize_database;
use crate::resources::ImageFolderPath;
use bevy::prelude::*;
use bevy::window::PrimaryWindow;

pub fn spawn_camera(mut commands: Commands, window_query: Query<&Window, With<PrimaryWindow>>) {
    let window: &Window = window_query.get_single().unwrap();

    commands.spawn(Camera2dBundle {
        transform: Transform::from_xyz(window.width() / 2.0, window.height() / 2.0, 0.0),
        ..default()
    });
}

pub fn initialize_database_if_image_folder_path(image_folder_path: Res<ImageFolderPath>) {
    if let Some(path) = &image_folder_path.image_folder_path {
        initialize_database(path.clone())
            .expect("Something went wrong when initializing the database.");
    }
}

pub fn check_if_tournament_in_progress() {}
