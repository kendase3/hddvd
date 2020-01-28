extern crate rand;
extern crate sdl2;

use sdl2::event::Event;
use sdl2::image::{InitFlag, LoadTexture};
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::{Point, Rect};
use sdl2::render::Texture;
use std::path::Path;
use std::time::Duration;

pub fn bounce(mut texture: Texture) -> Texture {
    texture.set_color_mod(
        rand::random::<u8>(),
        rand::random::<u8>(),
        rand::random::<u8>(),
    );
    texture
}

pub fn main() -> Result<(), String> {
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;
    let image_context = sdl2::image::init(InitFlag::PNG | InitFlag::JPG)?;

    let window = video_subsystem
        .window(
            "Please Re-insert 'Kangaroo Jack' to resume movie",
            1024,
            768,
        )
        .position_centered()
        .opengl()
        .build()
        .map_err(|e| e.to_string())?;

    let mut canvas = window.into_canvas().build().map_err(|e| e.to_string())?;

    canvas.set_draw_color(Color::RGB(0, 0, 0));
    canvas.clear();
    let texture_creator = canvas.texture_creator();
    let mut texture = texture_creator.load_texture(Path::new("logo.png"))?;
    texture = bounce(texture);
    let mut hdrect = Rect::new(10, 10, 780, 580);
    canvas.copy(&texture, None, hdrect)?;
    canvas.present();

    let mut event_pump = sdl_context.event_pump()?;
    let mut x_dir = "right";
    let mut y_dir = "down";

    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                }
                | Event::KeyDown {
                    keycode: Some(Keycode::Q),
                    ..
                } => break 'running,
                _ => {}
            }
        }
        ::std::thread::sleep(Duration::from_millis(10));
        // main logic loop, fires at interval ^
        match x_dir {
            "right" => {
                if hdrect.x + hdrect.width() as i32 > 1024 {
                    texture = bounce(texture);
                    x_dir = "left";
                } else {
                    hdrect.x += 1;
                }
            }
            "left" => {
                if hdrect.x <= 0 {
                    texture = bounce(texture);
                    x_dir = "right";
                } else {
                    hdrect.x -= 1;
                }
            }
            _ => {}
        }
        match y_dir {
            "down" => {
                if hdrect.y + hdrect.height() as i32 > 768 {
                    texture = bounce(texture);
                    y_dir = "up";
                } else {
                    hdrect.y += 1;
                }
            }
            "up" => {
                if hdrect.y <= 0 {
                    y_dir = "down";
                    texture = bounce(texture);
                } else {
                    hdrect.y -= 1;
                }
            }
            _ => {}
        }
        canvas.clear();
        canvas.copy(&texture, None, hdrect)?;
        canvas.present();
    }

    Ok(())
}
