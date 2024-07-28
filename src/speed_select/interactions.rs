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

pub fn interact_with_speed_select_buttons(
    mut button_query: Query<
        (&Interaction, &mut BackgroundColor, &SpeedSelectButton),
        (Changed<Interaction>, With<SpeedSelectButton>),
    >,
    mut speed_state_next_state: ResMut<NextState<SpeedState>>,
) {
    for (interaction, mut background_color, speed_select_button) in button_query.iter_mut() {
        match *interaction {
            Interaction::Pressed => match speed_select_button {
                SpeedSelectButton::SlowButton => {
                    println!("Speed state: Slow");
                    *background_color = BackgroundColor::from(HOVERED_BUTTON_COLOR);
                    speed_state_next_state.set(SpeedState::Slow);
                }
                SpeedSelectButton::NormalButton => {
                    println!("Speed state: Normal");
                    *background_color = BackgroundColor::from(HOVERED_BUTTON_COLOR);
                    speed_state_next_state.set(SpeedState::Normal);
                }
                SpeedSelectButton::FastButton => {
                    println!("Speed state: Fast");
                    *background_color = BackgroundColor::from(HOVERED_BUTTON_COLOR);
                    speed_state_next_state.set(SpeedState::Fast);
                }
            },
            Interaction::Hovered => {
                *background_color = BackgroundColor::from(HOVERED_BUTTON_COLOR);
            }
            Interaction::None => {
                *background_color = BackgroundColor::from(NORMAL_BUTTON_COLOR);
            }
        }
    }
}

pub fn colour_the_border_if_selected(
    mut speed_state: Res<State<SpeedState>>,
    mut button_query: Query<(&mut BorderColor, &SpeedSelectButton), (With<SpeedSelectButton>)>,
) {
    if speed_state.is_changed() {
        for (mut border_color, speed_select_button) in button_query.iter_mut() {
            match speed_state.get() {
                SpeedState::Slow => match speed_select_button {
                    SpeedSelectButton::SlowButton => border_color.0 = Color::BLACK,
                    _ => border_color.0 = Color::WHITE,
                },
                SpeedState::Normal => match speed_select_button {
                    SpeedSelectButton::NormalButton => border_color.0 = Color::BLACK,
                    _ => border_color.0 = Color::WHITE,
                },
                SpeedState::Fast => match speed_select_button {
                    SpeedSelectButton::FastButton => border_color.0 = Color::BLACK,
                    _ => border_color.0 = Color::WHITE,
                },
            }
        }
    }
}
