mod components;

use sdl2::event::Event;
use rusqlite::{params, Connection, Result};
use crate::components::renderer;
use rand::*;
use rand::prelude::SliceRandom;
use rand::thread_rng;

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

}

fn main() -> Result<()> {
    // Open a connection to the SQLite database
    let conn = match Connection::open("C:/Users/Isaac/RustroverProjects/database.db") {
        Ok(conn) => conn,
        Err(e) => {
            eprintln!("Error opening database: {}", e);
            return Err(e);
        }
    };

    match DatabaseTable::initialize(&conn) {
        Ok(()) => (),
        Err(e) => {
            eprintln!("Error initializing database: {}", e);
            return Err(e);
        }
    }

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

    let latest_round = match rounds_table.get_latest_round_number() {
        Ok(count) => count,
        Err(e) => {
            eprintln!("Failed to get the count: {}", e);
            return Err(e);
        }
    };

    run_tournament(&images_table)?;
    // println!("{}", latest_round);
    // println!("{}", images_table.voting_in_progress().unwrap().to_string());
    // println!("{}", images_table.get_current_round().unwrap().to_string());

    // pseudocode
    // check if not in progress (fresh table)
    // get count
    // if count is odd, we select the median value and send it to the next round automatically
    // for the list of all entries where out = false
    //      loop, pitting n and count-n-1 against each other
    //          specifically, check the round of each. Make sure both are same as current round
    //          *should we store this (current round) persistently in the database?
    //          set the 'out' bool to yes for the loser of each vote
    //      stop after we encounter the id where id = (id of median value) - 1
    //






    //
    // let (mut canvas, texture_creator, window_width, window_height) = renderer::initialize_sdl()?;
    //
    // let texture1 = renderer::load_texture(&texture_creator, "C:/Users/Isaac/Pictures/image1")?;
    // let texture2 = renderer::load_texture(&texture_creator, "C:/Users/Isaac/Pictures/image2")?;
    //
    // let mut event_pump = sdl2::init()?.event_pump()?;
    //
    // 'running: loop {
    //     for event in event_pump.poll_iter() {
    //         match event {
    //             Event::Quit { .. } => break 'running,
    //             Event::MouseButtonDown { x, y, .. } => {
    //                 let texture1_rect = (window_width / 4 - 50, window_height / 2 - 50, 100, 100);
    //                 let texture2_rect = (3 * window_width / 4 - 50, window_height / 2 - 50, 100, 100);
    //
    //                 if x >= texture1_rect.0 as i32 && x <= (texture1_rect.0 + texture1_rect.2) as i32 &&
    //                     y >= texture1_rect.1 as i32 && y <= (texture1_rect.1 + texture1_rect.3) as i32 {
    //                     println!("Image 1 clicked");
    //                 } else if x >= texture2_rect.0 as i32 && x <= (texture2_rect.0 + texture2_rect.2) as i32 &&
    //                     y >= texture2_rect.1 as i32 && y <= (texture2_rect.1 + texture2_rect.3) as i32 {
    //                     println!("Image 2 clicked");
    //                 }
    //             },
    //             _ => {}
    //         }
    //     }
    //     renderer::render_textures(&mut canvas, &texture1, &texture2, window_width, window_height)?;
    // }

    Ok(())
}

fn compete(database_table: &DatabaseTable, participant1: u64, participant2: u64) -> Result<u64> {
    if random() {
        database_table.increment_rating(participant1)?;
        Ok(participant1)
    } else {
        database_table.increment_rating(participant2)?;
        Ok(participant2)
    }
}

fn run_tournament(database_table: &DatabaseTable) -> Result<()> {
    let mut rng = thread_rng();
    let mut round_number = database_table.get_latest_round_number()?;
    let mut participants = database_table.get_remaining_participants(round_number)?;

    // run until only one left (winner)
    while participants.len() > 1 {
        // increment the round each loop and insert the round into a table for persistent storage
        round_number += 1;
        database_table.conn.execute("INSERT INTO rounds (round_number) VALUES (?1)",
                                    params![round_number])?;

        let round_id = database_table.conn.last_insert_rowid();

        // shuffle our vector
        participants.shuffle(&mut rng);

        // prepare our next round of participants
        let mut next_round: Vec<u64> = Vec::new();

        // chunk the vector into pairs and have them all compete
        for pair in participants.chunks(2) {
            if pair.len() == 2 {
                let winner = compete(database_table, pair[0], pair[1])?;
                next_round.push(winner);
                database_table.conn.execute(
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
        println!("The winner is: {}", database_table.get_image_path(winner)?);
    }

    Ok(())
}