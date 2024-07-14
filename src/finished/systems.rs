use crate::database::*;
use crate::file_system::*;
use crate::resources::ImageFolderPath;
use bevy::prelude::*;
use std::path::PathBuf;

pub fn generate_favourites_folder(image_folder_path_resource: Res<ImageFolderPath>) {
    if let Some(original_folder_name) = get_original_folder_name(&image_folder_path_resource) {
        let image_directory = create_image_directory(&original_folder_name)
            .to_string_lossy()
            .to_string();

        let percentile_map =
            calculate_percentiles().expect("Failed to calculate percentiles from database.");

        copy_images_to_directory(percentile_map, &image_directory).expect("Failed to copy images.");
    }
}

pub fn get_original_folder_name(
    image_folder_path_resource: &Res<ImageFolderPath>,
) -> Option<String> {
    if let Some(path) = &image_folder_path_resource.image_folder_path {
        let folder_path = PathBuf::from(path.file_name()?);
        return Some(folder_path.to_string_lossy().to_string());
    }
    None
}
