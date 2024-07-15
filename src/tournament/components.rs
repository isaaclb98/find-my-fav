use bevy::prelude::*;
use std::collections::VecDeque;

#[derive(Resource, Default, Debug)]
pub struct ParticipantsDeque {
    pub deque: VecDeque<u64>,
}

#[derive(Resource, Default, Debug)]
pub struct HandlesDeque {
    pub image_deque: VecDeque<Handle<Image>>,
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

#[derive(Event)]
pub struct ImageClickedEvent {
    pub(crate) left_image: bool,
}

#[derive(Event)]
pub struct ImagesLoadedEvent;

#[derive(Event)]
pub struct PopHandlesDequeEvent;
