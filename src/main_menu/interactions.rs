use crate::database::database_exists;
use crate::file_system::open_folder;
use crate::main_menu::components::{OpenFolderButton, ResumePreviousButton};
use crate::main_menu::systems::{get_image_folder_path_from_database, ResumedState};
use crate::resources::ImageFolderPath;
use crate::styles::{HOVERED_BUTTON_COLOR, NORMAL_BUTTON_COLOR};
use crate::AppState;
use bevy::prelude::{BackgroundColor, Changed, Interaction, NextState, Query, ResMut, With};

pub fn interact_with_folder_button(
    // Interaction is provided by Bevy for buttons
    mut button_query: Query<
        (&Interaction, &mut BackgroundColor),
        (Changed<Interaction>, With<OpenFolderButton>),
    >,
    mut app_state_next_state: ResMut<NextState<AppState>>,
    mut image_folder_path_resource: ResMut<ImageFolderPath>,
) {
    if let Ok((interaction, mut background_color)) = button_query.get_single_mut() {
        match *interaction {
            Interaction::Pressed => {
                *background_color = BackgroundColor::from(NORMAL_BUTTON_COLOR);

                match open_folder() {
                    Some(folder_path) => {
                        println!("{}", folder_path.to_string_lossy().to_string());

                        image_folder_path_resource.image_folder_path = Some(folder_path);
                        app_state_next_state.set(AppState::SpeedSelect);
                    }
                    None => {
                        println!("Failed to open a folder. Try again?")
                    }
                }
            }
            Interaction::Hovered => {
                *background_color = BackgroundColor::from(HOVERED_BUTTON_COLOR);
            }
            Interaction::None => {
                *background_color = BackgroundColor::from(NORMAL_BUTTON_COLOR);
            }
        }
    }
}

pub fn interact_with_resume_previous_button(
    mut button_query: Query<
        (&Interaction, &mut BackgroundColor),
        (Changed<Interaction>, With<ResumePreviousButton>),
    >,
    mut app_state_next_state: ResMut<NextState<AppState>>,
    mut resumed_state_next_state: ResMut<NextState<ResumedState>>,
    mut image_folder_path: ResMut<ImageFolderPath>,
) {
    if let Ok((interaction, mut background_color)) = button_query.get_single_mut() {
        match *interaction {
            Interaction::Pressed => {
                if database_exists() {
                    println!("Resume a previous tournament.");
                    get_image_folder_path_from_database(&mut image_folder_path);
                    resumed_state_next_state.set(ResumedState::Resumed);
                    app_state_next_state.set(AppState::SpeedSelect);
                } else {
                    println!("There is no previous tournament.");
                }
            }
            Interaction::Hovered => {
                *background_color = BackgroundColor::from(HOVERED_BUTTON_COLOR);
            }
            Interaction::None => {
                *background_color = BackgroundColor::from(NORMAL_BUTTON_COLOR);
            }
        }
    }
}
