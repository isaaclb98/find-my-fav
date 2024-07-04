use rand::*;
use rand::prelude::SliceRandom;
use rand::thread_rng;
use rusqlite::{Connection, params, Result};
use sdl2::event::Event;
use sdl2::EventPump;
use sdl2::rect::Rect;

use components::file_system;
use components::renderer;

use crate::components::database::DatabaseTable;

mod components;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // const NEW_DIRECTORY: &str = "C:/Users/Isaac/Pictures/favourites";
    let image_directory = file_system::create_image_directory().to_string_lossy().to_string();
    println!("{}", image_directory);

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

        let winner_path = database.get_image_path_with_max_rating()?;
        let winner_texture = renderer::load_texture(&texture_creator, &winner_path)?;

        renderer::render_winner(&mut canvas, &winner_texture, window_width, window_height, "Copying favourites...")?;

        let percentile_map = database.calculate_percentiles()?;
        file_system::copy_images_to_directory(percentile_map, &image_directory)?;

        let status_done = format!("Your favs have been copied to: {}", image_directory);
        renderer::render_winner(&mut canvas,
                                &winner_texture,
                                window_width,
                                window_height,
                                &status_done)?;

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
