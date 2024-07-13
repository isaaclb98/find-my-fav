use crate::database::initialize_database;
use crate::resources::ImageFolderPath;
use crate::styles::{get_button_text_style, NODE_BUNDLE_EMPTY_COLUMN_STYLE};
use crate::tournament::components::TournamentState;
use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use image::GenericImageView;
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

pub fn display_tournament_state(tournament_state: Res<State<TournamentState>>) {
    // println!("{:?}", tournament_state);
}

pub fn generate_finished_screen(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    commands
        .spawn(NodeBundle {
            style: NODE_BUNDLE_EMPTY_COLUMN_STYLE,
            ..default()
        })
        .with_children(|parent| {
            parent.spawn(TextBundle {
                text: Text {
                    sections: vec![TextSection::new(
                        "Tournament over. Winner.",
                        get_button_text_style(&asset_server),
                    )],
                    justify: JustifyText::Center,
                    ..default()
                },
                ..default()
            });
        });
}
