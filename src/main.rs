use rand::prelude::SliceRandom;
use rand::thread_rng;

use rusqlite::{Connection, params, Result};
use sdl2::event::Event;
use sdl2::EventPump;
use sdl2::rect::Rect;

use components::file_system;
use components::renderer;
use components::database;

mod components;

fn main() -> Result<(), Box<dyn std::error::Error>> {

    // create the directory to store the images after the tournament has been finished
    let image_directory = file_system::create_image_directory().to_string_lossy().to_string();

    // open a connection to the SQLite database
    // maybe in the future: create an sqlite database if the user does not already have one
    // this has many implications. For instance, we may have to integrate our image_to_db program.
    // that way, the user need only drag or type in a folder on their computer from which to get images
    // and the database will be created automatically.
    let conn = Connection::open("C:/Users/Isaac/RustroverProjects/database.db")?;

    // initialize database tables that are needed for tournament logic
    database::initialize(&conn)?;
    
    let (mut canvas, texture_creator, window_width, window_height) = renderer::initialize_sdl()?;
    let mut round_number = database::get_latest_round_number(&conn)?;
    let total_number_of_rounds = database::get_total_number_of_rounds(&conn)?;
    
    // get participants based on round number
    let mut participants = database::get_remaining_participants(&conn, round_number)?;
    
    // run until only one left (winner)
    while participants.len() > 1 {
        let mut rng = thread_rng();

        // shuffle our vector
        participants.shuffle(&mut rng);

        // chunk the vector into pairs and have them all compete
        for pair in participants.chunks(2) {
            if pair.len() == 2 {
                // get our image paths
                let image_1 = database::get_image_path_from_database(&conn, &pair[0])?;
                let image_2 = database::get_image_path_from_database(&conn, &pair[1])?;

                // try to catch when an image fails to render
                let texture_1 = match renderer::load_texture(&texture_creator, &image_1) {
                    Ok(texture) => texture,
                    Err(e) => {
                        eprintln!("Error with one of your images: {}", e);
                        database::set_loser_out(&conn, pair[0])?;
                        continue;
                    }
                };
                let texture_2 = match renderer::load_texture(&texture_creator, &image_2) {
                    Ok(texture) => texture,
                    Err(e) => {
                        eprintln!("Error with one of your images: {}", e);
                        database::set_loser_out(&conn, pair[1])?;
                        continue;
                    }
                };

                // render our textures (only once per round)
                let (texture1_rect, texture2_rect) = renderer::render_textures(&mut canvas, &texture_1, &texture_2, window_width, window_height)?;

                let mut event_pump = sdl2::init()?.event_pump()?;

                let winner = compete_loop(
                    &mut event_pump,
                    pair[0],
                    pair[1],
                    &texture1_rect,
                    &texture2_rect,
                )?;

                let (winner_texture, loser_id) = if winner == pair[0] {
                    (&texture_1, pair[1])
                } else {
                    (&texture_2, pair[0])
                };

                renderer::animate_zoom_out_and_in(&mut canvas, winner_texture, if winner == pair[0] { texture1_rect } else { texture2_rect }, 0.96)?;
                database::set_loser_out(&conn, loser_id)?;
                database::increment_rating(&conn, winner)?;

                conn.execute(
                    "INSERT INTO matches (round_number, participant1_id, participant2_id, winner_id)
                         VALUES (?1, ?2, ?3, ?4)",
                    params![round_number, pair[0], pair[1], winner])?;

                let match_id = conn.last_insert_rowid();

                println!("The winner of round {} (of {}), match {} is: {}",
                         round_number,
                         total_number_of_rounds,
                         match_id - (round_number - 1) as i64,
                         if winner == pair[0] {image_1} else {image_2});
            }
        }

        // increment the round each loop and insert the round into a table for persistent storage
        round_number += 1;
        // persist the change when match increases. create a fake round.
        conn.execute(
            "INSERT INTO matches (round_number, participant1_id, participant2_id, winner_id)
                         VALUES (?1, ?2, ?3, ?4)",
            params![round_number, 0, 0, 0])?;

        // update participants after every loop
        participants = database::get_remaining_participants(&conn, round_number)?;
    }

    let winner_path = database::get_image_path_with_max_rating(&conn)?;
    let winner_texture = renderer::load_texture(&texture_creator, &winner_path)?;

    renderer::render_winner(&mut canvas, &winner_texture, window_width, window_height, "Copying favourites...")?;

    let percentile_map = database::calculate_percentiles(&conn)?;
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

fn compete_loop(
    event_pump: &mut EventPump,
    participant1: u64,
    participant2: u64,
    texture1_rect: &Rect,
    texture2_rect: &Rect,
) -> Result<u64, rusqlite::Error> {
    loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } => {
                    println!("Quitting the application...");
                    std::process::exit(0); // exit application
                },
                Event::MouseButtonDown { x, y, .. } => {
                    if texture1_rect.contains_point((x, y)) {
                        return Ok(participant1);
                    } else if texture2_rect.contains_point((x, y)) {
                        return Ok(participant2);
                    }
                },
                _ => {}
            }
        }
    }
}

