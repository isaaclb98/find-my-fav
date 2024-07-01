
// the idea is to create a program where the user is shown two different images side by side.
// They have to pick which of the two they like.
// This will be updated in a database.
// images with a similar likeability will be contested.
// the winner will move up a tier.
// this continues until there is a winner.

// to begin with, lets start with ten images only.

mod components;

use sdl2::event::Event;
use rusqlite::{params, Connection, Result};
use crate::components::renderer;

struct DatabaseTable<'a> {
    conn: &'a Connection,
    table: String,
    columns: Vec<String>,
}

impl<'a> DatabaseTable<'a> {
    // see if the voting is in progress or if it is a fresh table
    fn voting_in_progress(&self) -> Result<bool, rusqlite::Error> {
        // prepare the SQL query dynamically using the table and column names
        let query = format!("SELECT COUNT(*) FROM {} WHERE {} != 0", &self.table, &self.columns[3]);

        // execute the query. No parameters, and we are getting the result of one row.
        let count: i64 = self.conn.query_row(&query, params![], |row| row.get(0))?;

        Ok(count != 0)
    }

    // get the current round. It is the minimum round from the 'round' column in our database.
    // a round will move on once all entries in our database have moved up a round.
    // initially from round 0 to round 1, for instance.
    fn get_current_round(&self) -> Result<u16, rusqlite::Error> {
        let query = format!("SELECT MIN({}) FROM {}", &self.columns[3], &self.table);

        self.conn.query_row(&query, params![], |row| {
            row.get::<usize, i64>(0)
        }).map(|count| count as u16)
    }

    // get initial count
    fn get_initial_count(&self) -> Result<u64, rusqlite::Error> {
        let query = format!("SELECT COUNT(*) FROM {}", &self.table);

        self.conn.query_row(&query, params![], |row| {
            row.get::<usize, i64>(0)
        }).map(|count| count as u64)
    }
}

fn main() -> Result<(), String> {
    // Open a connection to the SQLite database
    let conn = match Connection::open("C:/Users/Isaac/RustroverProjects/database.db") {
        Ok(conn) => conn,
        Err(e) => {
            eprintln!("Error opening database: {}", e);
            return Err(e.to_string());
        }
    };

    // particular to my database. Will try to generalize further in the future.
    let database_table = DatabaseTable {
        conn: &conn,
        table: String::from("images"),
        columns: vec![
            String::from("id"),
            String::from("image_path"),
            String::from("rating"),
            String::from("round"),
            String::from("out"),
        ]
    };

    let initial_count = match database_table.get_initial_count() {
        Ok(count) => count,
        Err(e) => {
            eprintln!("Failed to get the count: {}", e);
            return Err(e.to_string());
        }
    };

    println!("{}", initial_count);
    println!("{}", database_table.voting_in_progress().unwrap().to_string());
    println!("{}", database_table.get_current_round().unwrap().to_string());

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