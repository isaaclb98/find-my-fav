use rand::*;
use rand::prelude::SliceRandom;
use rand::thread_rng;
use rusqlite::{Connection, params, Result};
use sdl2::event::Event;
use sdl2::EventPump;

use crate::components::renderer;

mod components;

struct DatabaseTable<'a> {
    conn: &'a Connection,
    table: String,
    columns: Vec<String>,
}

impl<'a> DatabaseTable<'a> {
    fn initialize(conn: &Connection) -> Result<()> {
        conn.execute("CREATE TABLE IF NOT EXISTS rounds (
                      id INTEGER PRIMARY KEY AUTOINCREMENT,
                      round_number INTEGER NOT NULL
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

    // input: &DatabaseTable, &u64
    // output: Result<&str>
    fn get_image_path_from_database(&self, id: &u64) -> Result<String> {
        let query = format!("SELECT image_path FROM images WHERE id = ?1");
        self.conn.query_row(&query, params![id], |row| {
            row.get(0)
        })
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Open a connection to the SQLite database
    let conn = Connection::open("C:/Users/Isaac/RustroverProjects/database.db")?;

    DatabaseTable::initialize(&conn)?;

    // particular to my database. Will try to generalize further in the future.
    let images_table = DatabaseTable {
        conn: &conn,
        table: String::from("images"),
        columns: vec![
            String::from("id"),
            String::from("image_path"),
            String::from("rating"),
        ]
    };

    let rounds_table = DatabaseTable {
        conn: &conn,
        table: String::from("rounds"),
        columns: vec![
            String::from("id"),
            String::from("round_number")
        ]
    };

    let matches_table = DatabaseTable {
        conn: &conn,
        table: String::from("matches"),
        columns: vec![
            String::from("id"),
            String::from("round_id"),
            String::from("participant1_id"),
            String::from("participant2_id"),
            String::from("winner_id"),
        ]
    };

    let (mut canvas, texture_creator, window_width, window_height) = renderer::initialize_sdl()?;
    let aspect_ratio = window_width / window_height;

    let mut rng = thread_rng();
    let mut round_number = images_table.get_latest_round_number()?;
    let mut participants = images_table.get_remaining_participants(round_number)?;

    // run until only one left (winner)
    while participants.len() > 1 {
        // increment the round each loop and insert the round into a table for persistent storage
        round_number += 1;
        images_table.conn.execute("INSERT INTO rounds (round_number) VALUES (?1)",
                                    params![round_number])?;

        let round_id = images_table.conn.last_insert_rowid();

        // shuffle our vector
        participants.shuffle(&mut rng);

        // prepare our next round of participants
        let mut next_round: Vec<u64> = Vec::new();

        // chunk the vector into pairs and have them all compete
        for pair in participants.chunks(2) {
            if pair.len() == 2 {
                // get our image paths
                let image_1 = images_table.get_image_path_from_database(&pair[0])?;
                let image_2 = images_table.get_image_path_from_database(&pair[1])?;

                // get our image textures
                let texture_1 = renderer::load_texture(&texture_creator, &image_1)?;
                let texture_2 = renderer::load_texture(&texture_creator, &image_2)?;

                // render our textures (only once per round)
                renderer::render_textures(&mut canvas, &texture_1, &texture_2, window_width, window_height)?;

                let mut event_pump = sdl2::init()?.event_pump()?;

                let winner = compete_loop(
                    &images_table,
                    &mut event_pump,
                    pair[0],
                    pair[1],
                    window_width,
                    window_height,
                )?;

                next_round.push(winner);

                images_table.conn.execute(
                    "INSERT INTO matches (round_id, participant1_id, participant2_id, winner_id)
                         VALUES (?1, ?2, ?3, ?4)",
                    params![round_id, pair[0], pair[1], winner])?;
            } else {
                next_round.push(pair[0]);
            }
        }

        // update participants to the winners of the round
        participants = next_round;
    }

    if let Some(&winner) = participants.get(0) {
        println!("The winner is: {}", images_table.get_image_path(winner)?);
    }

    Ok(())
}
fn compete_loop(
    database_table: &DatabaseTable,
    event_pump: &mut EventPump,
    participant1: u64,
    participant2: u64,
    window_width: u32,
    window_height: u32,
) -> Result<u64, rusqlite::Error> {
    loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } => {
                    println!("Quitting the application...");
                    std::process::exit(0); // exit application
                },
                Event::MouseButtonDown { x, y, .. } => {
                    // areas in which the user clicks
                    // supposed to cover the images
                    let texture1_rect = (
                        window_width / 4 - 500,
                        window_height / 2 - 500,
                        1000,
                        1000
                    );
                    let texture2_rect = (
                        3 * window_width / 4 - 500,
                        window_height / 2 - 500,
                        1000,
                        1000
                    );

                    if x >= texture1_rect.0 as i32 && x <= (texture1_rect.0 + texture1_rect.2) as i32 &&
                        y >= texture1_rect.1 as i32 && y <= (texture1_rect.1 + texture1_rect.3) as i32 {
                        println!("Image 1 clicked");
                        database_table.increment_rating(participant1)?;
                        return Ok(participant1);
                    } else if x >= texture2_rect.0 as i32 && x <= (texture2_rect.0 + texture2_rect.2) as i32 &&
                        y >= texture2_rect.1 as i32 && y <= (texture2_rect.1 + texture2_rect.3) as i32 {
                        println!("Image 2 clicked");
                        database_table.increment_rating(participant2)?;
                        return Ok(participant2);
                    }
                },
                _ => {}
            }
        }
    }
}
