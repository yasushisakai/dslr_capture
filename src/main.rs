extern crate sdl2;
mod camera;

use sdl2::image::{LoadTexture, InitFlag};
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use camera::Camera;
use std::time::Duration;
use std::thread::sleep;

pub fn run() -> Result<(), String> {

    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;
    let _image_context = sdl2::image::init(InitFlag::JPG)?;

    let mut cam = Camera::new();
    let model = cam.get_model();
    let (width, height) = cam.get_size();
    let window = video_subsystem.window(&model, width, height) //FIXME: image size is hard coded
      .position_centered()
      .build()
      .map_err(|e| e.to_string())?;

    let mut canvas = window.into_canvas().software().build().map_err(|e| e.to_string())?;
    let texture_creator = canvas.texture_creator();

    'mainloop: loop {
        let bytes = cam.get_preview(); // this is bytes thats in jpg format

        let texture = texture_creator.load_texture_bytes(&bytes)?;
        canvas.copy(&texture, None, None)?;
        canvas.present();
        for event in sdl_context.event_pump()?.poll_iter() {
            match event {
                Event::Quit{..} |
                Event::KeyDown {keycode: Option::Some(Keycode::Escape), ..} =>
                    break 'mainloop,
                _ => {}
            }
        }

       sleep(Duration::new(0, 1_000_000_000u32/24)); // nap time

    }

    Ok(())
}

fn main() -> Result<(), String> {
    run()?;
    Ok(())
}
