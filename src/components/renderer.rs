extern crate sdl2;

use sdl2::image::{self, InitFlag, LoadTexture};
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::{TextureCreator, WindowCanvas};
use sdl2::video::{WindowContext, DisplayMode};
use std::path::Path;

pub fn initialize_sdl() -> Result<(WindowCanvas, TextureCreator<WindowContext>, u32, u32), String> {
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;

    let display_mode: DisplayMode = video_subsystem.current_display_mode(0)
        .map_err(|e| e.to_string())?;

    let window_width = display_mode.w as u32;
    let window_height = display_mode.h as u32;

    let window = video_subsystem
        .window("Penis", window_width, window_height)
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

// no fuckin clue what this is... look it up later
pub fn load_texture<'a>(texture_creator: &'a TextureCreator<WindowContext>, path: &str) -> Result<sdl2::render::Texture<'a>, String> {
    texture_creator.load_texture(Path::new(path))
}

pub fn render_textures(
    canvas: &mut WindowCanvas,
    texture1: &sdl2::render::Texture,
    texture2: &sdl2::render::Texture,
    window_width: u32,
    window_height: u32,
) -> Result<(), String> {
    canvas.set_draw_color(Color::RGB(255,255,255));
    canvas.clear();

    // change texture rectangle sizes later
    // need actual dimensions of the image
    let texture1_rect = Rect::new((window_width / 4 - 500) as i32, (window_height / 2 - 500) as i32, 1000, 1000);
    let texture2_rect = Rect::new((3 * window_width / 4 - 500) as i32, (window_height / 2 - 500) as i32, 1000, 1000);

    canvas.copy(texture1, None, Some(texture1_rect))?;
    canvas.copy(texture2, None, Some(texture2_rect))?;
    canvas.present();
    Ok(())
}