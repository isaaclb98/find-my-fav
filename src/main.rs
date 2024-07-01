
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

fn main() -> Result<(), String> {
    // Open a connection to the SQLite database
    // let conn = match Connection::open("C:/Users/Isaac/RustroverProjects/database.db") {
    //     Ok(conn) => conn,
    //     Err(e) => {
    //         eprintln!("Error opening database: {}", e);
    //         return Err(e.to_string());
    //     }
    // };

    let (mut canvas, texture_creator, window_width, window_height) = renderer::initialize_sdl()?;

    let texture1 = renderer::load_texture(&texture_creator, "C:/Users/Isaac/Pictures/image1")?;
    let texture2 = renderer::load_texture(&texture_creator, "C:/Users/Isaac/Pictures/image2")?;

    let mut event_pump = sdl2::init()?.event_pump()?;

    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } => break 'running,
                Event::MouseButtonDown { x, y, .. } => {
                    let texture1_rect = (window_width / 4 - 50, window_height / 2 - 50, 100, 100);
                    let texture2_rect = (3 * window_width / 4 - 50, window_height / 2 - 50, 100, 100);

                    if x >= texture1_rect.0 as i32 && x <= (texture1_rect.0 + texture1_rect.2) as i32 &&
                        y >= texture1_rect.1 as i32 && y <= (texture1_rect.1 + texture1_rect.3) as i32 {
                        println!("Image 1 clicked");
                    } else if x >= texture2_rect.0 as i32 && x <= (texture2_rect.0 + texture2_rect.2) as i32 &&
                        y >= texture2_rect.1 as i32 && y <= (texture2_rect.1 + texture2_rect.3) as i32 {
                        println!("Image 2 clicked");
                    }
                },
                _ => {}
            }
        }
        renderer::render_textures(&mut canvas, &texture1, &texture2, window_width, window_height)?;
    }

    Ok(())
}
