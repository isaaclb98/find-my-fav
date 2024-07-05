use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};
use std::io::Result;
use rayon::prelude::{IntoParallelRefIterator, ParallelIterator};

fn get_user_home_directory() -> PathBuf {
    dirs::home_dir().expect("Could not find the home directory")
}

pub fn create_image_directory() -> PathBuf {
    let home = get_user_home_directory();
    home.join("Pictures").join("favourites")
}

pub fn copy_images_to_directory(percentile_map: HashMap<String, f64>, new_directory: &str) -> Result<()> {
    if !Path::new(new_directory).exists() {
        fs::create_dir_all(new_directory)?;
    }

    percentile_map.par_iter().try_for_each(|(image_path, &percentile)| {
        // if in the top 85th percentile
        if percentile >= 85f64 {
            // create a path
            let path = Path::new(&image_path);

            // make sure both file name and extension are valid
            if let Some(file_name) = path.file_name() {
                if let Some(extension) = path.extension() {
                    // create a new file name
                    // append its percentile to one decimal as prefix
                    // and use its original extension
                    let new_file_name = format!("{:05.1}_{}.{}", percentile, file_name.to_string_lossy(), extension.to_string_lossy());

                    let destination_path = Path::new(new_directory).join(new_file_name);

                    // copy the file
                    fs::copy(&image_path, &destination_path)?;
                }
            }
        }
        Ok(())
    })
}