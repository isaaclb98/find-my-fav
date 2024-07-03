extern crate sdl2;

use sdl2::image::{self, InitFlag, LoadTexture};
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::{TextureCreator, WindowCanvas};
use sdl2::video::{WindowContext, DisplayMode};
use std::path::Path;
use std::thread;
use std::time::Duration;

pub fn initialize_sdl() -> Result<(WindowCanvas, TextureCreator<WindowContext>, u32, u32), String> {
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;

    let display_mode: DisplayMode = video_subsystem.current_display_mode(0)
        .map_err(|e| e.to_string())?;

    let window_width = display_mode.w as u32;
    let window_height = display_mode.h as u32;

    let window = video_subsystem
        .window("FindMyFav", window_width, window_height)
        .position_centered()
        .resizable()
        .maximized()
        .build()
        .map_err(|e| e.to_string())?;

    let canvas = window.into_canvas().build().map_err(|e| e.to_string())?;
    let texture_creator = canvas.texture_creator();

    // Initialize SDL2_image with support for PNG and JPEG formats
    image::init(InitFlag::PNG | InitFlag::JPG)?;

    Ok((canvas, texture_creator, window_width, window_height))
}

pub fn load_texture<'a>(texture_creator: &'a TextureCreator<WindowContext>, path: &str) -> Result<sdl2::render::Texture<'a>, String> {
    texture_creator.load_texture(Path::new(path))
}

pub fn render_textures(
    canvas: &mut WindowCanvas,
    texture1: &sdl2::render::Texture,
    texture2: &sdl2::render::Texture,
    window_width: u32,
    window_height: u32,
) -> Result<(Rect, Rect), String> {
    canvas.set_draw_color(Color::RGB(255,255,255));
    canvas.clear();

    // get the dimensions of the textures
    let query_texture1 = texture1.query();
    let query_texture2 = texture2.query();
    let (img1_width, img1_height) = (query_texture1.width, query_texture1.height);
    let (img2_width, img2_height) = (query_texture2.width, query_texture2.height);

    // calculate the aspect ratios
    let aspect_ratio1 = img1_width as f32 / img1_height as f32;
    let aspect_ratio2 = img2_width as f32 / img2_height as f32;

    // calculate the new dimensions for the first texture
    let mut scaled_width1 = window_width / 2;
    let mut scaled_height1 = (scaled_width1 as f32 / aspect_ratio1) as u32;
    // fit our image
    if scaled_height1 > window_height {
        scaled_height1 = window_height;
        scaled_width1 = (scaled_height1 as f32 * aspect_ratio1) as u32;
    }

    // calculate the new dimensions for the second texture
    let mut scaled_width2 = window_width / 2;
    let mut scaled_height2 = (scaled_width2 as f32 / aspect_ratio2) as u32;
    if scaled_height2 > window_height {
        scaled_height2 = window_height;
        scaled_width2 = (scaled_height2 as f32 * aspect_ratio2) as u32;
    }

    // center the textures vertically
    let texture1_rect = Rect::new(
        ((window_width / 4) - (scaled_width1 / 2)) as i32,
        ((window_height / 2) - (scaled_height1 / 2)) as i32,
        scaled_width1,
        scaled_height1,
    );
    let texture2_rect = Rect::new(
        ((3 * window_width / 4) - (scaled_width2 / 2)) as i32,
        ((window_height / 2) - (scaled_height2 / 2)) as i32,
        scaled_width2,
        scaled_height2,
    );

    // render the textures
    canvas.copy(texture1, None, Some(texture1_rect))?;
    canvas.copy(texture2, None, Some(texture2_rect))?;
    canvas.present();

    Ok((texture1_rect, texture2_rect))
}

pub fn animate_zoom_out(
    canvas: &mut WindowCanvas,
    texture: &sdl2::render::Texture,
    rect: Rect,
    zoom_factor: f32,
) -> Result<(), String> {
    let steps = 10;
    for i in 0..steps {
        let scale = 1.0 - (i as f32 / steps as f32) * (1.0 - zoom_factor);
        let new_width = (rect.width() as f32 * scale) as u32;
        let new_height = (rect.height() as f32 * scale) as u32;
        let new_x = rect.x() + ((rect.width() - new_width) / 2) as i32;
        let new_y = rect.y() + ((rect.height() - new_height) / 2) as i32;

        let new_rect = Rect::new(new_x, new_y, new_width, new_height);

        canvas.set_draw_color(Color::RGB(255, 255, 255));
        canvas.clear();
        canvas.copy(texture, None, Some(new_rect))?;
        canvas.present();

        thread::sleep(Duration::from_millis(5)); // Adjust the speed of the animation
    }
    Ok(())
}