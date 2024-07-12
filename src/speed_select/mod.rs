use crate::speed_select::components::*;
use crate::speed_select::systems::interactions::*;
use crate::speed_select::systems::layout::{
    build_speed_select, despawn_speed_select, spawn_speed_select,
};
use crate::AppState;
use bevy::app::{App, Plugin, Update};
use bevy::prelude::*;

mod components;
mod styles;
mod systems;

pub struct SpeedSelectPlugin;

impl Plugin for SpeedSelectPlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<SpeedState>()
            .add_systems(OnEnter(AppState::SpeedSelect), spawn_speed_select)
            .add_systems(
                Update,
                (
                    interact_with_begin_button,
                    interact_with_slow_button,
                    interact_with_normal_button,
                    interact_with_fast_button,
                )
                    .run_if(in_state(AppState::SpeedSelect)),
            )
            .add_systems(OnExit(AppState::SpeedSelect), despawn_speed_select);
    }
}
