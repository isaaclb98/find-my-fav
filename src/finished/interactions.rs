use crate::file_system::open_folder;
use crate::finished::components::*;
use crate::main_menu::systems::*;
use crate::resources::ImageFolderPath;
use crate::styles::*;
use crate::AppState;
use bevy::prelude::{BackgroundColor, Changed, Interaction, NextState, Query, ResMut, With};

pub fn interact_with_new_folder_button(
    mut button_query: Query<
        (&Interaction, &mut BackgroundColor),
        (Changed<Interaction>, With<OpenCreatedFolderButton>),
    >,
    mut app_state_next_state: ResMut<NextState<AppState>>,
    mut image_folder_path_resource: ResMut<ImageFolderPath>,
) {
    if let Ok((interaction, mut background_color)) = button_query.get_single_mut() {
        match *interaction {
            Interaction::Pressed => {
                *background_color = BackgroundColor::from(NORMAL_BUTTON_COLOR);
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

pub fn interact_with_start_over_button(
    mut button_query: Query<
        (&Interaction, &mut BackgroundColor),
        (Changed<Interaction>, With<StartOverButton>),
    >,
    mut app_state_next_state: ResMut<NextState<AppState>>,
    mut resumed_state_next_state: ResMut<NextState<ResumedState>>,
) {
    if let Ok((interaction, mut background_color)) = button_query.get_single_mut() {
        match *interaction {
            Interaction::Pressed => {
                println!("Resume a previous tournament.");
                resumed_state_next_state.set(ResumedState::New);
                app_state_next_state.set(AppState::MainMenu);
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
