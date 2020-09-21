use gphoto;
use std::path::Path;
use image;
use std::thread::sleep;
use std::time::Duration;

pub struct Camera {
    width: u32,
    height: u32,
    context: gphoto::Context,
    device: gphoto::Camera,
}

impl Camera {
    pub fn new() -> Self {
        let mut context = gphoto::Context::new().expect("cannot create gphoto context, is libgphoto2 installed?");
        let mut device = gphoto::Camera::autodetect(&mut context).expect("cannot
        auto-detect camera, is the camera connected? try re-plugging the
        camera.");

        let abilities = device.abilities();
        let model = abilities.model();

        println!("connected to: {}", model);

        println!("checking [1/2] capture-image");

        let file = device.capture_image(&mut context).expect("could not capture image.");
        let file_name = (*file.basename()).to_string();
        let path = Path::new(&file_name);
        let mut img_file = gphoto::FileMedia::create(path).unwrap();

        device.download(&mut context, &file, &mut img_file).expect("could not download image from camera to local");

        // deleting test image from caputure-image
        std::fs::remove_file(&path).unwrap();

        println!("OK");

        sleep(Duration::from_secs(3));

        println!("checking [2/2] capture-preview");

        let mut media: gphoto::FileMedia;

        loop {
            match device.capture_preview(&mut context) {
                Ok(fm) => {
                    media = fm;
                    break;
                },
                Err(e) => {
                    println!("{}",e.message());
                    panic!();
                }
            }
            // sleep(Duration::from_secs(3));
        }

        let media_bytes = media.get_data();

        std::fs::write("test.jpg", &media_bytes).unwrap();

        let img = image::open("test.jpg").unwrap();
        let img_rgb = img.to_rgb();

        let (width, height) = img_rgb.dimensions();

        std::fs::remove_file("test.jpg").unwrap();

        println!("OK");

        sleep(Duration::from_secs(3));

        println!("checking complete :)");

        println!("width: {}, height:{}", width, height);

        Camera{width, height, context, device}
    }

    pub fn get_size(&self) -> (u32, u32) {
        (self.width, self.height)
    }

    pub fn get_preview(&mut self) -> Vec<u8> {
        let mut media = self.device.capture_preview(&mut self.context).unwrap();
        media.get_data()
    }

    pub fn get_model(&mut self) -> String {
        let abilities = self.device.abilities();
        (*abilities.model()).to_string()
    }
}
