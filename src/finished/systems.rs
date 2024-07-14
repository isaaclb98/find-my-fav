use crate::database::*;
use crate::file_system::*;
use bevy::prelude::*;

pub fn generate_favourites_folder() {
    // create the directory to store the images after the tournament has been finished
    let image_directory = create_image_directory().to_string_lossy().to_string();

    let percentile_map =
        calculate_percentiles().expect("Failed to calculate percentiles from database.");

    copy_images_to_directory(percentile_map, &image_directory).expect("Failed to copy images.");
}
