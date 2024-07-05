use std::collections::HashMap;
use rusqlite::{Connection, params, Result};

pub(crate) fn initialize(conn: &Connection) -> Result<()> {
    conn.execute("CREATE TABLE IF NOT EXISTS images (\
                    id INTEGER PRIMARY KEY AUTOINCREMENT,\
                    image_path STRING,\
                    rating INTEGER DEFAULT 0,\
                    out INTEGER DEFAULT 0)", params![])?;
    
    conn.execute("CREATE TABLE IF NOT EXISTS matches (
                  id INTEGER PRIMARY KEY AUTOINCREMENT,
                  round_number INTEGER NOT NULL DEFAULT 1,
                  participant1_id INTEGER,
                  participant2_id INTEGER,
                  winner_id INTEGER,
                  FOREIGN KEY (participant1_id) REFERENCES participants(id),
                  FOREIGN KEY (participant2_id) REFERENCES participants(id),
                  FOREIGN KEY (winner_id) REFERENCES participants(id)
              )", params![])?;

    Ok(())
}

pub(crate) fn get_latest_round_number(conn: &Connection) -> Result<u64> {
    let query = "SELECT COALESCE(MAX(round_number), 1) FROM matches".to_string();
    conn.query_row(&query, params![], |row| {
        row.get::<usize, i64>(0)
    }).map(|count| count as u64)
}

pub(crate) fn get_total_number_of_rounds(conn: &Connection) -> Result<u64> { 
    let total_images: u64 = conn.query_row(
        "SELECT COUNT(*) FROM images",
        params![],
        |row| row.get(0),
    )?;

    if total_images < 2 {
        return Ok(0);
    }
    Ok((total_images as f64).log2().ceil() as u64)
}

pub(crate) fn get_remaining_participants(conn: &Connection, round_number: u64) -> Result<Vec<u64>> {
    let mut sql_statement = conn.prepare(
        "SELECT id FROM images
         WHERE id NOT IN (
             SELECT participant1_id FROM matches WHERE round_number = ?1
             UNION ALL
             SELECT participant2_id FROM matches WHERE round_number = ?1
         ) 
         AND out != 1"
    )?;

    let participants = sql_statement.query_map(params![round_number], |row| {
        row.get::<usize, i64>(0)
    })?.map(|result| result.unwrap() as u64)
        .collect();

    Ok(participants)
}

pub(crate) fn get_image_path_with_max_rating(conn: &Connection) -> Result<String> {
    conn.query_row(
        "SELECT image_path FROM images ORDER BY rating DESC LIMIT 1",
        params![],
        |row| row.get(0)
    )
}

pub(crate) fn increment_rating(conn: &Connection, image_id: u64) -> Result<()> {
    let mut rating = get_rating(&conn, image_id)?;

    rating += 1;

    conn.execute(
        "UPDATE images SET rating = ?1 WHERE id = ?2",
        params![rating, image_id],
    )?;

    Ok(())
}

pub(crate) fn get_image_path_from_database(conn: &Connection, id: &u64) -> Result<String> {
    let query = "SELECT image_path FROM images WHERE id = ?1".to_string();
    conn.query_row(&query, params![id], |row| {
        row.get(0)
    })
}

pub(crate) fn _get_tournament_finished(conn: &Connection, round_number: u64) -> Result<bool> {
    conn.query_row(
        "SELECT tournament_finished FROM rounds WHERE round_number = ?1",
        params![round_number],
        |row| {
            let finished: i32 = row.get(0)?;
            Ok(finished != 0)
        }
    )
}

pub(crate) fn set_loser_out(conn: &Connection, image_id: u64) -> Result<()> {
    conn.execute(
        "UPDATE images SET out = ?1 WHERE id = ?2",
        params![1, image_id],
    )?;
    
    Ok(())
}

pub(crate) fn calculate_percentiles(conn: &Connection) -> Result<HashMap<String, f64>> {
    // retrieve all images
    let mut stmt = conn.prepare("SELECT image_path FROM images ORDER BY rating DESC")?;
    let images = stmt.query_map(params![], |row| {
        let image_path: String = row.get(0)?;
        Ok(image_path)
    })?.filter_map(Result::ok).collect::<Vec<_>>();

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

fn _get_image_path(conn: &Connection, image_id: u64) -> Result<String> {
    conn.query_row("SELECT image_path FROM images WHERE id = ?1", params![image_id], |row| row.get(0))
}

fn get_rating(conn: &Connection, image_id: u64) -> Result<u32> {
    conn.query_row(
        "SELECT rating FROM images WHERE id = ?1",
        params![image_id],
        |row| row.get(0),
    )
}