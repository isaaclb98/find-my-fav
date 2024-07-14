use chrono::Local;
use rayon::prelude::{IntoParallelRefIterator, ParallelIterator};
use rfd::FileDialog;
use std::collections::HashMap;
use std::fs;
use std::io::Result;
use std::path::{Path, PathBuf};
use std::time::{SystemTime, UNIX_EPOCH};

fn get_user_home_directory() -> PathBuf {
    dirs::home_dir().expect("Could not find the home directory")
}

pub fn create_image_directory(original_folder_name: &str) -> PathBuf {
    // get users home directory. platform-agnostic.
    let home = get_user_home_directory();

    // get date
    let current_date = Local::now().format("%Y%m%d").to_string();

    // get number of seconds from unix epoch.
    let start = SystemTime::now();
    let since_epoch = start.duration_since(UNIX_EPOCH).expect("Time error");
    let seconds = since_epoch.as_secs();

    // retrieve the last eight digits.
    let seconds = format!("{}", seconds);
    let last_four_digits = &seconds[seconds.len() - 4..];

    home.join("Pictures")
        .join("Favourites".to_string())
        .join(format!(
            "{}-{}-{}",
            original_folder_name, current_date, last_four_digits
        ))
}

pub fn copy_images_to_directory(
    percentile_map: HashMap<String, f64>,
    new_directory: &str,
) -> Result<()> {
    if !Path::new(new_directory).exists() {
        fs::create_dir_all(new_directory)?;
    }

    percentile_map
        .par_iter()
        .try_for_each(|(image_path, &percentile)| {
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
                        let new_file_name =
                            format!("{:05.1}_{}", percentile, file_name.to_string_lossy());

                        let destination_path = Path::new(new_directory).join(new_file_name);

                        // copy the file
                        fs::copy(image_path, destination_path)?;
                    }
                }
            }
            Ok(())
        })
}

pub fn open_folder() -> Option<PathBuf> {
    FileDialog::new().pick_folder()
}
