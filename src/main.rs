mod components;

use std::collections::HashMap;
use rand::*;
use rand::prelude::SliceRandom;
use rand::thread_rng;
use rusqlite::{Connection, params, Result};
use sdl2::event::Event;
use sdl2::EventPump;
use sdl2::rect::Rect;
use components::renderer;
use components::file_system;

struct DatabaseTable<'a> {
    conn: &'a Connection,
    table: String,
    columns: Vec<String>,
}

impl<'a> DatabaseTable<'a> {
    fn initialize(conn: &Connection) -> Result<()> {
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
    fn get_latest_round_number(&self) -> Result<u64> {
        let query = format!("SELECT IFNULL(MAX(round_number), 0) FROM rounds");
        self.conn.query_row(&query, params![], |row| {
            row.get::<usize, i64>(0)
        }).map(|count| count as u64)
    }

    // images and matches
    fn get_remaining_participants(&self, round_number: u64) -> Result<Vec<u64>> {
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

    fn get_image_path_with_max_rating(&self) -> Result<String> {
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

    fn increment_rating(&self, image_id: u64) -> Result<()> {
        let mut rating = self.get_rating(image_id)?;

        rating += 1;

        self.conn.execute(
            "UPDATE images SET rating = ?1 WHERE id = ?2",
            params![rating, image_id],
        )?;

        Ok(())
    }

    fn get_image_path_from_database(&self, id: &u64) -> Result<String> {
        let query = format!("SELECT image_path FROM images WHERE id = ?1");
        self.conn.query_row(&query, params![id], |row| {
            row.get(0)
        })
    }

    fn get_tournament_finished(&self, round_number: u64) -> Result<bool> {
        self.conn.query_row(
            "SELECT tournament_finished FROM rounds WHERE round_number = ?1",
            params![round_number],
            |row| {
                let finished: i32 = row.get(0)?;
                Ok(finished != 0)
            }
        )
    }

    fn calculate_percentiles(&self) -> Result<HashMap<String, f64>> {
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

fn main() -> Result<(), Box<dyn std::error::Error>> {
    const NEW_DIRECTORY: &str = "C:/Users/Isaac/Pictures/favourites";
    // Open a connection to the SQLite database
    let conn = Connection::open("C:/Users/Isaac/RustroverProjects/database.db")?;

    DatabaseTable::initialize(&conn)?;

    // particular to my database. Will try to generalize further in the future.
    let database = DatabaseTable {
        conn: &conn,
        table: String::from("images"),
        columns: vec![
            String::from("id"),
            String::from("image_path"),
            String::from("rating"),
        ]
    };

    let (mut canvas, texture_creator, window_width, window_height) = renderer::initialize_sdl()?;

    let mut rng = thread_rng();
    let mut round_number = database.get_latest_round_number()?;
    let mut participants = database.get_remaining_participants(round_number)?;
    let tournament_finished = database.get_tournament_finished(round_number).unwrap_or(false);

    // run until only one left (winner)
    while participants.len() > 1 && !tournament_finished {
        // increment the round each loop and insert the round into a table for persistent storage
        round_number += 1;
        database.conn.execute(
            "INSERT INTO rounds (round_number) VALUES (?1)",
            params![round_number]
        )?;

        // shuffle our vector
        participants.shuffle(&mut rng);

        // prepare our next round of participants
        let mut next_round: Vec<u64> = Vec::new();

        // chunk the vector into pairs and have them all compete
        for pair in participants.chunks(2) {
            if pair.len() == 2 {
                // get our image paths
                let image_1 = database.get_image_path_from_database(&pair[0])?;
                let image_2 = database.get_image_path_from_database(&pair[1])?;

                // get our image textures
                let texture_1 = renderer::load_texture(&texture_creator, &image_1)?;
                let texture_2 = renderer::load_texture(&texture_creator, &image_2)?;

                // render our textures (only once per round)
                let (texture1_rect, texture2_rect) = renderer::render_textures(&mut canvas, &texture_1, &texture_2, window_width, window_height)?;

                let mut event_pump = sdl2::init()?.event_pump()?;

                let (winner, stupid) = compete_loop(
                    &database,
                    &mut event_pump,
                    pair[0],
                    pair[1],
                    &texture1_rect,
                    &texture2_rect,
                )?;

                if stupid == 1 {
                    renderer::animate_zoom_out(&mut canvas, &texture_1, texture1_rect, 0.96)?;
                } else {
                    renderer::animate_zoom_out(&mut canvas, &texture_2, texture2_rect, 0.96)?;
                }

                next_round.push(winner);

                database.conn.execute(
                    "INSERT INTO matches (round_id, participant1_id, participant2_id, winner_id)
                         VALUES (?1, ?2, ?3, ?4)",
                    params![round_number, pair[0], pair[1], winner])?;

                let match_id = database.conn.last_insert_rowid();

                println!("The winner of round {}, match {} is: {}",
                         round_number,
                         match_id,
                         if winner == pair[0] {image_1} else {image_2});
            } else {
                next_round.push(pair[0]);
            }
        }
        // update participants to the winners of the round
        participants = next_round;
    }

    if let Some(&winner) = participants.get(0) {
        database.conn.execute(
            "UPDATE rounds SET tournament_finished = ?1 WHERE id = ?2",
            params![1, round_number],
        )?;

        let percentile_map = database.calculate_percentiles()?;
        file_system::copy_images_to_directory(percentile_map, NEW_DIRECTORY)?;

        let winner_path = database.get_image_path_with_max_rating()?;
        let winner_texture = renderer::load_texture(&texture_creator, &winner_path)?;
        renderer::render_winner(&mut canvas, &winner_texture, window_width, window_height)?;

        println!("The winner is: {}", winner_path);

        let mut event_pump = sdl2::init()?.event_pump()?;

        loop {
            for event in event_pump.poll_iter() {
                if let Event::Quit { .. } = event {
                    println!("Quitting the application...");
                    std::process::exit(0); // exit application
                }
            }
        }    
    }

    Ok(())
}
fn compete_loop(
    database_table: &DatabaseTable,
    event_pump: &mut EventPump,
    participant1: u64,
    participant2: u64,
    texture1_rect: &Rect,
    texture2_rect: &Rect,
) -> Result<(u64, u8), rusqlite::Error> {
    loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } => {
                    println!("Quitting the application...");
                    std::process::exit(0); // exit application
                },
                Event::MouseButtonDown { x, y, .. } => {
                    if texture1_rect.contains_point((x,y)) {
                        println!("Image 1 clicked");
                        database_table.increment_rating(participant1)?;
                        return Ok((participant1, 1));
                    } else if texture2_rect.contains_point((x,y)) {
                        println!("Image 2 clicked");
                        database_table.increment_rating(participant2)?;
                        return Ok((participant2, 2));
                    }
                },
                _ => {}
            }
        }
    }
}
