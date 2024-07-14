use crate::styles::{get_button_text_style, NODE_BUNDLE_EMPTY_COLUMN_STYLE};
use crate::tournament::components::TournamentState;
use crate::AppState;
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

pub fn show_app_state(app_state_res: Res<State<AppState>>) {
    let state = app_state_res.get();
    println!("{:?}", state);
}
