use crate::speed_select::components::*;
use crate::styles::*;
use crate::AppState;
use bevy::prelude::*;

pub fn interact_with_begin_button(
    mut button_query: Query<
        (&Interaction, &mut BackgroundColor),
        (Changed<Interaction>, With<BeginButton>),
    >,
    mut app_state_next_state: ResMut<NextState<AppState>>,
) {
    if let Ok((interaction, mut background_color)) = button_query.get_single_mut() {
        match *interaction {
            Interaction::Pressed => {
                println!("Begin.");
                app_state_next_state.set(AppState::Tournament);
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

pub fn interact_with_slow_button(
    mut button_query: Query<
        (&Interaction, &mut BackgroundColor),
        (Changed<Interaction>, With<SlowButton>),
    >,
    mut speed_state_next_state: ResMut<NextState<SpeedState>>,
) {
    if let Ok((interaction, mut background_color)) = button_query.get_single_mut() {
        match *interaction {
            Interaction::Pressed => {
                println!("Speed state: Slow");
                speed_state_next_state.set(SpeedState::Slow);
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

pub fn interact_with_normal_button(
    mut button_query: Query<
        (&Interaction, &mut BackgroundColor),
        (Changed<Interaction>, With<NormalButton>),
    >,
    mut speed_state_next_state: ResMut<NextState<SpeedState>>,
) {
    if let Ok((interaction, mut background_color)) = button_query.get_single_mut() {
        match *interaction {
            Interaction::Pressed => {
                println!("Speed state: Normal");
                speed_state_next_state.set(SpeedState::Normal);
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

pub fn interact_with_fast_button(
    mut button_query: Query<
        (&Interaction, &mut BackgroundColor),
        (Changed<Interaction>, With<FastButton>),
    >,
    mut speed_state_next_state: ResMut<NextState<SpeedState>>,
) {
    if let Ok((interaction, mut background_color)) = button_query.get_single_mut() {
        match *interaction {
            Interaction::Pressed => {
                println!("Speed state: Fast");
                speed_state_next_state.set(SpeedState::Fast);
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
