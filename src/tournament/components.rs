use bevy::prelude::{Component, Resource, States};
use std::collections::VecDeque;

#[derive(Resource, Default, Debug)]
pub struct ParticipantsDeque {
    pub deque: VecDeque<u64>,
}

#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
pub enum TournamentState {
    #[default]
    Generating,
    Displaying,
    Deciding,
}

#[derive(Component)]
pub struct BothImageComponents;

#[derive(Component)]
pub struct LeftImageComponent;

#[derive(Component)]
pub struct RightImageComponent;
