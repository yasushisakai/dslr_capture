use gphoto;

pub struct Camera {
    context: gphoto::Context,
    device: gphoto::Camera,
}

impl Camera {
    pub fn new() -> Self {
        let mut context = gphoto::Context::new().unwrap();
        let mut device = gphoto::Camera::autodetect(&mut context).unwrap();

        let abilities = device.abilities();
        let model = abilities.model();

        println!("connected to: {}", model);

        Camera{context, device}
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
