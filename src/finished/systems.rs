use crate::database::*;
use crate::file_system::*;
use crate::finished::components::{FavouritesFolderResource, FolderGeneratedState};
use crate::resources::ImageFolderPath;
use bevy::prelude::*;
use std::path::Path;
use std::path::PathBuf;
use std::process::{Command, Stdio};

pub fn generate_favourites_folder(
    image_folder_path_resource: Res<ImageFolderPath>,
    mut folder_generated_next_state: ResMut<NextState<FolderGeneratedState>>,
    mut favourites_folder_resource: ResMut<FavouritesFolderResource>,
) {
    if let Some(original_folder_name) = get_original_folder_name(&image_folder_path_resource) {
        let image_directory = create_image_directory(&original_folder_name)
            .to_string_lossy()
            .to_string();

        let percentile_map =
            calculate_percentiles().expect("Failed to calculate percentiles from database.");

        copy_images_to_directory(percentile_map, &image_directory).expect("Failed to copy images.");

        favourites_folder_resource.favourites_folder_path = Some(image_directory);
        folder_generated_next_state.set(FolderGeneratedState::Generated);
    }
}

pub fn open_new_folder(favourites_folder_resource: &Res<FavouritesFolderResource>) {
    if let Some(favourites_folder) = &favourites_folder_resource.favourites_folder_path {
        let path: &Path = Path::new(favourites_folder);

        if cfg!(target_os = "windows") {
            if let Err(e) = Command::new("explorer")
                .arg(path)
                .stdout(Stdio::null())
                .stderr(Stdio::null())
                .spawn()
            {
                eprintln!("Failed to open path in explorer: {}", e);
            }
        } else if cfg!(target_os = "macos") {
            if let Err(e) = Command::new("open")
                .arg(path)
                .stdout(Stdio::null())
                .stderr(Stdio::null())
                .spawn()
            {
                eprintln!("Failed to open path in finder: {}", e);
            }
        } else if cfg!(target_os = "linux") {
            if let Err(e) = Command::new("xdg-open")
                .arg(path)
                .stdout(Stdio::null())
                .stderr(Stdio::null())
                .spawn()
            {
                eprintln!("Failed to open path in file manager: {}", e);
            }
        } else {
            eprintln!("Unsupported operating system");
        }
    }
}

fn get_original_folder_name(image_folder_path_resource: &Res<ImageFolderPath>) -> Option<String> {
    if let Some(path) = &image_folder_path_resource.image_folder_path {
        let folder_path = PathBuf::from(path.file_name()?);
        return Some(folder_path.to_string_lossy().to_string());
    }
    None
}
