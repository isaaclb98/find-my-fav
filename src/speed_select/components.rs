use bevy::prelude::{Component, States};

#[derive(Component)]
pub struct SpeedSelectComponent;

#[derive(Component)]
pub struct BeginButton;

#[derive(Component)]
pub enum SpeedSelectButton {
    SlowButton,
    NormalButton,
    FastButton,
}

#[derive(Component)]
pub struct SlowButton;

#[derive(Component)]
pub struct NormalButton;

#[derive(Component)]
pub struct FastButton;

#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
pub enum SpeedState {
    Slow,
    #[default]
    Normal,
    Fast,
}
