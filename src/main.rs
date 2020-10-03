extern crate sdl2;
mod camera;

use sdl2::image::{LoadTexture, InitFlag};
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use camera::Camera;
use std::time::Duration;
use std::thread::sleep;
use std::thread::spawn;
use std::sync::{Arc, Mutex};

pub fn run() -> Result<(), String> {

    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;
    let _image_context = sdl2::image::init(InitFlag::JPG|InitFlag::PNG)?;

    // let mut cam = Camera::new();
    // let model = cam.get_model();
    // let (width, height) = cam.get_size();

    let test_img = image::open("connecting.png").unwrap();

    let window = video_subsystem.window("test", 1024, 680)
      .position_centered()
      .build()
      .map_err(|e| e.to_string())?;

    let mut canvas = window.into_canvas().software().build().map_err(|e| e.to_string())?;
    let texture_creator = canvas.texture_creator();

    let bytes: Arc<Mutex<Vec<u8>>> = Arc::new(Mutex::new(test_img.to_bytes()));

    //different thread that asks for the buffer

    let b = Arc::clone(&bytes);

    spawn( move|| {
        let mut cam = Camera::new();
        loop {
            {
                let mut bytes = b.lock().unwrap();
                *bytes = cam.get_preview();
            }
            sleep(Duration::new(0,1_000_000_000u32/24));
        }
    });

    'mainloop: loop {
        // let bytes = cam.get_preview(); // this is bytes thats in jpg format
        {
            let new_bytes = Arc::clone(&bytes);
            let texture = match texture_creator.load_texture_bytes(&new_bytes.lock().unwrap()){
                Ok(t) => t,
                Err(_s) => {
                    texture_creator.load_texture("connecting.png").unwrap()
                }
            };
            canvas.copy(&texture, None, None)?;
            canvas.present();
        }
        for event in sdl_context.event_pump()?.poll_iter() {
            match event {
                Event::Quit{..} |
                Event::KeyDown {keycode: Option::Some(Keycode::Escape), ..} =>
                    break 'mainloop,
                _ => {}
            }
        }

       sleep(Duration::new(0, 1_000_000_000u32/30)); // nap time
    }

    Ok(())
}

fn main() -> Result<(), String> {
    run()?;
    Ok(())
}
