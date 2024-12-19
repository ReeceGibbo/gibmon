mod device;
mod image_extensions;
mod packets;

use crate::device::{Device, Orientation};
use image::{GenericImageView, Pixel};

fn main() {
    let mut device = Device::new().expect("Could not connect to COM4 port");

    device.set_brightness(50).expect("Could not set brightness");

    device
        .set_background_picture("C:\\Users\\susif\\Pictures\\Wallpapers\\templeofdoom.png")
        .expect("Could not set background image");

    device
        .set_orientation(Orientation::ReverseLandscape)
        .expect("Could not set orientation");

    device
        .set_background_picture("C:\\Users\\susif\\Pictures\\Wallpapers\\templeofdoom.png")
        .expect("Could not set background image");

    // device.screen_black().expect("Could not set black color");
    device.screen_white().expect("Could not set white color");
    device.screen_off().expect("Could not set screen off");


    // API Integration
    // Soundcloud
    // Spotify
    // Github
}
