use std::collections::HashMap;
use rusqlite::{Connection, params, Result};

pub struct DatabaseTable<'a> {
    pub(crate) conn: &'a Connection,
    pub table: String,
    pub columns: Vec<String>,
}

impl<'a> DatabaseTable<'a> {
    pub(crate) fn initialize(conn: &Connection) -> rusqlite::Result<()> {
        conn.execute("CREATE TABLE IF NOT EXISTS rounds (
                      id INTEGER PRIMARY KEY AUTOINCREMENT,
                      round_number INTEGER NOT NULL,
                      tournament_finished INTEGER DEFAULT 0
                  )", params![])?;

        conn.execute("CREATE TABLE IF NOT EXISTS matches (
                      id INTEGER PRIMARY KEY AUTOINCREMENT,
                      round_id INTEGER NOT NULL,
                      participant1_id INTEGER,
                      participant2_id INTEGER,
                      winner_id INTEGER,
                      FOREIGN KEY (round_id) REFERENCES rounds(id),
                      FOREIGN KEY (participant1_id) REFERENCES participants(id),
                      FOREIGN KEY (participant2_id) REFERENCES participants(id),
                      FOREIGN KEY (winner_id) REFERENCES participants(id)
                  )", params![])?;

        Ok(())
    }

    // rounds
    pub(crate) fn get_latest_round_number(&self) -> rusqlite::Result<u64> {
        let query = format!("SELECT IFNULL(MAX(round_number), 0) FROM rounds");
        self.conn.query_row(&query, params![], |row| {
            row.get::<usize, i64>(0)
        }).map(|count| count as u64)
    }

    // images and matches
    pub(crate) fn get_remaining_participants(&self, round_number: u64) -> rusqlite::Result<Vec<u64>> {
        let mut sql_statement = self.conn.prepare(
            "SELECT id FROM images
                                 WHERE id NOT IN (SELECT participant1_id FROM matches WHERE round_id = ?1
                                     UNION ALL
                                     SELECT participant2_id FROM matches WHERE round_id = ?1)")?;
        let participants = sql_statement.query_map(params![round_number], |row| {
            row.get::<usize, i64>(0)
        })?.map(|result| result.unwrap() as u64)
            .collect();

        Ok(participants)
    }

    fn get_image_path(&self, image_id: u64) -> Result<String> {
        self.conn.query_row("SELECT image_path FROM images WHERE id = ?1", params![image_id], |row| row.get(0))
    }

    pub(crate) fn get_image_path_with_max_rating(&self) -> Result<String> {
        self.conn.query_row(
            "SELECT image_path FROM images ORDER BY rating DESC LIMIT 1",
            params![],
            |row| row.get(0)
        )
    }

    fn get_rating(&self, image_id: u64) -> Result<u32> {
        self.conn.query_row(
            "SELECT rating FROM images WHERE id = ?1",
            params![image_id],
            |row| row.get(0),
        )
    }

    pub(crate) fn increment_rating(&self, image_id: u64) -> Result<()> {
        let mut rating = self.get_rating(image_id)?;

        rating += 1;

        self.conn.execute(
            "UPDATE images SET rating = ?1 WHERE id = ?2",
            params![rating, image_id],
        )?;

        Ok(())
    }

    pub(crate) fn get_image_path_from_database(&self, id: &u64) -> Result<String> {
        let query = format!("SELECT image_path FROM images WHERE id = ?1");
        self.conn.query_row(&query, params![id], |row| {
            row.get(0)
        })
    }

    pub(crate) fn get_tournament_finished(&self, round_number: u64) -> Result<bool> {
        self.conn.query_row(
            "SELECT tournament_finished FROM rounds WHERE round_number = ?1",
            params![round_number],
            |row| {
                let finished: i32 = row.get(0)?;
                Ok(finished != 0)
            }
        )
    }

    pub(crate) fn calculate_percentiles(&self) -> Result<HashMap<String, f64>> {
        // retrieve all images
        let mut stmt = self.conn.prepare("SELECT image_path FROM images ORDER BY rating DESC")?;
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

        for (imagepath, percentile) in &percentiles {
            println!("{}: {}", percentile, imagepath);
        }
        Ok(percentiles)
    }

}