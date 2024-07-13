use crate::database::initialize_database;
use crate::resources::ImageFolderPath;
use bevy::prelude::{Res, States};

#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
pub enum ResumedState {
    #[default]
    New,
    Resumed,
}

pub fn initialize_database_if_image_folder_path(image_folder_path: Res<ImageFolderPath>) {
    if let Some(path) = &image_folder_path.image_folder_path {
        initialize_database(path.clone())
            .expect("Something went wrong when initializing the database.");
    }
}
