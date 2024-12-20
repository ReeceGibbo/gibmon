use crate::r#virtual::display::Display;

pub trait Layer {
    fn bounding_box(&self) -> (u32, u32, u32, u32);
    fn get_image_data(&self) -> &Vec<u8>;
}
