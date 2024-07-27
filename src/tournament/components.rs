use bevy::prelude::*;
use std::collections::VecDeque;

pub struct ParticipantInfo {
    pub id: u64,
    pub handle: Option<Handle<Image>>,
    pub loaded: bool,
    pub errored: bool,
}

#[derive(Resource, Default)]
pub struct ParticipantsDeque {
    pub participants_deque: VecDeque<ParticipantInfo>,
}

#[derive(Resource, Default, Debug)]
pub struct ParticipantsToLoadDeque {
    pub participants_to_load_deque: VecDeque<u64>,
}

#[derive(Resource, Default, Debug)]
pub struct Indices {
    pub index_1: usize,
    pub index_2: usize,
}

#[derive(Resource, Default, Debug)]
pub struct NumberOfParticipantsForMatch(u32);

#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
pub enum TournamentState {
    #[default]
    Entering,
    Generating,
    Loading,
    Displaying,
    Deciding,
    Resolving,
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
pub struct TransitionToResolvingEvent;

#[derive(Event)]
pub struct TransitionToFinishedEvent;

#[derive(Event)]
pub struct DespawnImagesEvent;

#[derive(Event)]
pub struct ImageClickedEvent {
    pub left_image: bool,
}
