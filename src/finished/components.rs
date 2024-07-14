use bevy::prelude::*;

#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
pub enum FolderGeneratedState {
    #[default]
    NotGenerated,
    Generated,
}

#[derive(Resource, Default)]
pub struct FavouritesFolderResource {
    pub favourites_folder_path: Option<String>,
}

#[derive(Component)]
pub struct FinishedScreenComponent;

#[derive(Component)]
pub struct OpenCreatedFolderButton;

#[derive(Component)]
pub struct StartOverButton;
