use bevy::prelude::*;
use std::path::PathBuf;

#[derive(Resource)]
pub struct ImageFolderPath {
    pub image_folder_path: Option<PathBuf>,
}

impl Default for ImageFolderPath {
    fn default() -> Self {
        ImageFolderPath {
            image_folder_path: None,
        }
    }
}

#[derive(Resource)]
pub struct UsedMemory(pub f64);

impl Default for UsedMemory {
    fn default() -> Self {
        UsedMemory(0.0)
    }
}
