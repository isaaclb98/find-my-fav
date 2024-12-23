use crate::speed_select::interactions::*;
use crate::speed_select::layout::*;
use crate::AppState;
use bevy::app::{App, Plugin, Update};
use bevy::prelude::*;
use components::SpeedState;

pub mod components;
pub mod interactions;
pub mod layout;

pub struct SpeedSelectPlugin;

impl Plugin for SpeedSelectPlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<SpeedState>()
            .add_systems(OnEnter(AppState::SpeedSelect), spawn_speed_select)
            .add_systems(
                Update,
                (
                    interact_with_begin_button,
                    interact_with_speed_select_buttons,
                    colour_the_border_if_selected,
                )
                    .run_if(in_state(AppState::SpeedSelect)),
            )
            .add_systems(OnExit(AppState::SpeedSelect), despawn_speed_select);
    }
}
