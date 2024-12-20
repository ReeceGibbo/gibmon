use crate::r#virtual::layer::Layer;

pub struct Image {
    x: u32,
    y: u32,
    width: u32,
    height: u32,
    data: Vec<u8>, // RGBA data of the image
}

impl Image {
    pub fn new(x: u32, y: u32, width: u32, height: u32, data: Vec<u8>) -> Self {
        Image {
            x,
            y,
            width,
            height,
            data,
        }
    }
}

impl Layer for Image {
    fn bounding_box(&self) -> (u32, u32, u32, u32) {
        (self.x, self.y, self.width, self.height)
    }

    fn get_image_data(&self) -> &Vec<u8> {
        &self.data
    }
}
