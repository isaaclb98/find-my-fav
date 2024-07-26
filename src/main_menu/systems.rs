use crate::database::{get_image_path_with_max_rating, initialize_database};
use crate::resources::ImageFolderPath;
use bevy::prelude::*;
use std::path::Path;

#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
pub enum ResumedState {
    #[default]
    New,
    Resumed,
}

// get highest rated image
// get path from images
// set resource

pub fn initialize_database_if_image_folder_path(image_folder_path: Res<ImageFolderPath>) {
    if let Some(path) = &image_folder_path.image_folder_path {
        initialize_database(path.clone())
            .expect("Something went wrong when initializing the database.");
    }
}

pub fn get_image_folder_path_from_database(mut image_folder_path: &mut ResMut<ImageFolderPath>) {
    let image_folder_path_max_rating =
        get_image_path_with_max_rating().expect("Failed to get image path with max rating.");
    let path = Path::new(&image_folder_path_max_rating);
    let directory_path = path.parent();

    let image_folder_path_option = directory_path.map(|path| path.to_path_buf());
    image_folder_path.image_folder_path = image_folder_path_option;
}
