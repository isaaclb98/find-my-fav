use bevy::prelude::*;
use std::collections::VecDeque;

#[derive(Resource, Default, Debug)]
pub struct ParticipantsDeque {
    pub participants_deque: VecDeque<u64>,
    pub handles_deque: VecDeque<Handle<Image>>,
}

#[derive(Resource, Default, Debug)]
pub struct ParticipantsToLoadDeque {
    pub participants_to_load_deque: VecDeque<u64>,
}

#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
pub enum TournamentState {
    #[default]
    Generating,
    Displaying,
    Loading,
    Deciding,
}

#[derive(Component)]
pub struct BothImageComponents;

#[derive(Component)]
pub struct LeftImageComponent;

#[derive(Component)]
pub struct RightImageComponent;

#[derive(Event)]
pub struct TransitionToGeneratingEvent;

#[derive(Event)]
pub struct TransitionToLoadingEvent;

#[derive(Event)]
pub struct TransitionToDisplayingEvent;

#[derive(Event)]
pub struct TransitionToDecidingEvent;

#[derive(Event)]
pub struct TransitionToFinishedEvent;

#[derive(Event)]
pub struct DespawnImagesEvent;

#[derive(Event)]
pub struct TwoImagesLoadedEvent;

#[derive(Event)]
pub struct PopTwoHandlesEvent;

#[derive(Event)]
pub struct ImageClickedEvent {
    pub left_image: bool,
}

#[derive(Event)]
pub struct ImageErrorEvent {
    pub left_image_fail: bool,
}

#[derive(Event)]
pub struct NewRoundNeeded;
