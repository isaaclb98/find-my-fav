use std::collections::HashMap;
use std::env;
use std::fs;
use std::path::PathBuf;

use glob::glob;
use rusqlite::{params, Connection, Result};

fn get_database_path() -> Result<PathBuf> {
    let exe_path = env::current_exe().expect("Failed to get the executable path");
    let exe_dir = exe_path
        .parent()
        .expect("Failed to get the executable directory");

    // create the path for the new SQLite database
    let db_path = exe_dir.join("find_my_fav_database.db");
    Ok(db_path)
}

pub(crate) fn initialize_database(image_folder_path: PathBuf) -> Result<()> {
    let db_path = get_database_path().expect("Error getting database path.");

    // check if the database file exists and delete it if it does
    if db_path.exists() {
        fs::remove_file(&db_path).expect("Failed to delete existing database file");
    }

    println!("Initializing database...");

    // create a new SQLite database
    let conn = Connection::open(db_path)?;

    conn.execute(
        "CREATE TABLE IF NOT EXISTS images (\
                    id INTEGER PRIMARY KEY AUTOINCREMENT,\
                    image_path STRING,\
                    rating INTEGER DEFAULT 0,\
                    out INTEGER DEFAULT 0)",
        params![],
    )?;

    conn.execute(
        "CREATE TABLE IF NOT EXISTS matches (
                  id INTEGER PRIMARY KEY AUTOINCREMENT,
                  round_number INTEGER NOT NULL DEFAULT 1,
                  participant1_id INTEGER,
                  participant2_id INTEGER,
                  winner_id INTEGER,
                  FOREIGN KEY (participant1_id) REFERENCES participants(id),
                  FOREIGN KEY (participant2_id) REFERENCES participants(id),
                  FOREIGN KEY (winner_id) REFERENCES participants(id)
              )",
        params![],
    )?;

    let image_patterns = vec!["*.jpg", "*.jpeg", "*.png"];
    let image_folder_path = image_folder_path.to_string_lossy().to_string();

    for pattern in image_patterns {
        let full_pattern = format!("{}/**/{}", image_folder_path, pattern);
        for entry in glob(&full_pattern).expect("Failed to read glob pattern.") {
            match entry {
                Ok(path) => {
                    let image_path = path.to_string_lossy();
                    let image_path = image_path.replace("\\", "/");

                    conn.execute(
                        "INSERT INTO images (image_path) VALUES (?1)",
                        params![image_path],
                    )?;
                }
                Err(e) => println!("{:?}", e),
            }
        }
    }

    println!("Successfully initialized database...");

    Ok(())
}

pub(crate) fn get_latest_round_number() -> Result<u64> {
    let db_path = get_database_path().expect("Error getting database path");
    let conn = Connection::open(db_path).expect("Error opening connection");

    let query = "SELECT COALESCE(MAX(round_number), 1) FROM matches".to_string();
    conn.query_row(&query, params![], |row| row.get::<usize, i64>(0))
        .map(|count| count as u64)
}

pub(crate) fn _get_total_number_of_rounds(conn: &Connection) -> Result<u64> {
    let total_images = get_total_number_of_participants(conn)?;

    if total_images < 2 {
        return Ok(0);
    }
    Ok((total_images as f64).log2().ceil() as u64)
}

pub(crate) fn get_remaining_participants() -> Result<Vec<u64>> {
    let db_path = get_database_path().expect("Error getting database path.");
    let round_number = get_latest_round_number().expect("Failed to get round_number");

    let conn = Connection::open(db_path).expect("Error opening connection");

    let mut sql_statement = conn.prepare(
        "SELECT id FROM images
         WHERE id NOT IN (
             SELECT participant1_id FROM matches WHERE round_number = ?1
             UNION ALL
             SELECT participant2_id FROM matches WHERE round_number = ?1
         ) 
         AND out != 1",
    )?;

    let participants = sql_statement
        .query_map(params![round_number], |row| row.get::<usize, i64>(0))?
        .map(|result| result.unwrap() as u64)
        .collect();

    Ok(participants)
}

pub(crate) fn _get_image_path_with_max_rating(conn: &Connection) -> Result<String> {
    conn.query_row(
        "SELECT image_path FROM images ORDER BY rating DESC LIMIT 1",
        params![],
        |row| row.get(0),
    )
}

pub(crate) fn increment_rating(image_id: u64) -> Result<()> {
    let db_path = get_database_path().expect("Error getting database path.");
    let conn = Connection::open(db_path).expect("Error opening connection");

    let mut rating: i32 = conn
        .query_row(
            "SELECT rating FROM images WHERE id = ?1",
            params![image_id],
            |row| row.get(0),
        )
        .expect("Failed to get rating");

    rating += 1;

    conn.execute(
        "UPDATE images SET rating = ?1 WHERE id = ?2",
        params![rating, image_id],
    )?;

    Ok(())
}

pub(crate) fn get_image_path_from_database(id: &u64) -> Result<PathBuf> {
    let db_path = get_database_path().expect("Error getting database path.");
    let conn = Connection::open(db_path).expect("Error opening connection");

    let query = "SELECT image_path FROM images WHERE id = ?1".to_string();
    let path: String = conn.query_row(&query, params![id], |row| row.get(0))?;

    Ok(PathBuf::from(path))
}

pub(crate) fn _get_tournament_finished(conn: &Connection, round_number: u64) -> Result<bool> {
    conn.query_row(
        "SELECT tournament_finished FROM rounds WHERE round_number = ?1",
        params![round_number],
        |row| {
            let finished: i32 = row.get(0)?;
            Ok(finished != 0)
        },
    )
}

pub(crate) fn set_loser_out(image_id: u64) -> Result<()> {
    let db_path = get_database_path().expect("Error getting database path.");
    let conn = Connection::open(db_path).expect("Error opening connection");

    conn.execute(
        "UPDATE images SET out = ?1 WHERE id = ?2",
        params![1, image_id],
    )?;

    Ok(())
}

pub(crate) fn calculate_percentiles() -> Result<HashMap<String, f64>> {
    let db_path = get_database_path().expect("Error getting database path.");
    let conn = Connection::open(db_path).expect("Error opening connection");

    // retrieve all images
    let mut stmt = conn.prepare("SELECT image_path FROM images ORDER BY rating DESC")?;
    let images = stmt
        .query_map(params![], |row| {
            let image_path: String = row.get(0)?;
            Ok(image_path)
        })?
        .filter_map(Result::ok)
        .collect::<Vec<_>>();

    // calculate the total number of images
    let total_images = images.len() as f64;

    // (image_path, percentile) map
    let mut percentiles = HashMap::new();

    for (index, image_path) in images.iter().enumerate() {
        let percentile = (1.0 - (index as f64 / total_images)) * 100.0;
        percentiles.insert(image_path.clone(), percentile);
    }

    Ok(percentiles)
}

fn get_total_number_of_participants(conn: &Connection) -> Result<u64> {
    let total_images: u64 =
        conn.query_row("SELECT COUNT(*) FROM images", params![], |row| row.get(0))?;

    Ok(total_images)
}

pub(crate) fn get_number_of_matches(conn: &Connection, round_number: u64) -> Result<u64> {
    let total_images = get_total_number_of_participants(conn)?;

    if round_number < 1 || total_images < 2 {
        return Ok(0);
    }

    let mut remaining = total_images;
    for _ in 1..round_number {
        remaining = (remaining + 1) / 2; // ceil division by 2
    }
    Ok((remaining + 1) / 2)
}

pub(crate) fn get_total_number_of_matches_until_now(
    conn: &Connection,
    round_number: u64,
) -> Result<u64> {
    if round_number == 1 {
        return get_number_of_matches(conn, round_number);
    }

    let matches_for_current_round = get_number_of_matches(conn, round_number)?;
    let matches_for_past_rounds = get_total_number_of_matches_until_now(conn, round_number - 1)?;

    Ok(matches_for_current_round + matches_for_past_rounds)
}

pub(crate) fn insert_match_into_database(
    round_number: u64,
    participant1: u64,
    participant2: u64,
    winner: u64,
) -> Result<()> {
    let db_path = get_database_path().expect("Error getting database path.");
    let conn = Connection::open(db_path).expect("Error opening connection");

    conn.execute(
        "INSERT INTO matches (round_number, participant1_id, participant2_id, winner_id)
                         VALUES (?1, ?2, ?3, ?4)",
        params![round_number, participant1, participant2, winner],
    )?;

    Ok(())
}
