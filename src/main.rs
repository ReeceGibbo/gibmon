mod device;
mod gibmon_config;
mod image_extensions;
mod packets;
mod spotify;
mod r#virtual;

use crate::device::{Device, Orientation};
use crate::gibmon_config::load_config;
use crate::image_extensions::{
    create_playing_bar_to_rgb565, create_text_image, load_image_from_url_to_rgb565,
    load_image_from_url_to_rgba,
};
use crate::spotify::{fetch_currently_playing, fetch_spotify_token};
use crate::r#virtual::display::Display;
use crate::r#virtual::image::Image;
use image::{GenericImageView, Pixel};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

#[tokio::main]
async fn main() {
    //
    // let image = Image::new(&display);
    // image.change_image("path_to_new_image");
    //
    // let text = Text::new(&display);
    // text.update_text("New text value");

    let device = Arc::new(Mutex::new(
        Device::new().expect("Could not connect to COM4 port"),
    ));

    // Perform some basic device setup
    {
        let mut dev = device.lock().unwrap();
        dev.set_brightness(80).expect("Could not set brightness");

        dev.set_orientation(Orientation::ReverseLandscape)
            .expect("Could not set orientation");
    }

    let display = Arc::new(Mutex::new(Display::new(device.clone())));

    let image_data = load_image_from_url_to_rgba(
        "https://i.scdn.co/image/ab67616d0000b273e9c3c16b480e1c5a84d7b188",
        256,
        256,
    )
    .await
    .expect("Could not load image");
    let basic_image = Image::new(112, 32, 256, 256, image_data);

    let transparent_image_data = load_image_from_url_to_rgba(
        "https://www.transparenttextures.com/patterns/brushed-alum-dark.png",
        480,
        320,
    )
    .await
    .expect("Could not load image");
    let transparent_image = Image::new(0, 0, 480, 320, transparent_image_data);

    // Add image to display as layer
    {
        let mut d = display.lock().unwrap();
        d.add_layer(1, Arc::new(Mutex::new(basic_image)));
        d.add_layer(0, Arc::new(Mutex::new(transparent_image)));
        d.redraw_full();
    }

    // device
    //     .set_background_picture("C:\\Users\\susif\\Pictures\\Wallpapers\\templeofdoom.png")
    //     .expect("Could not set background image");
    //
    // let image_data = load_image_from_url_to_rgb565(
    //     "https://i.scdn.co/image/ab67616d0000b273e9c3c16b480e1c5a84d7b188",
    //     256,
    //     256,
    // )
    // .await
    // .expect("Could not load image");
    //
    // device
    //     .display_picture(image_data, 112, 32, 256, 256)
    //     .expect("Could not display image");
    //
    // let text_image_data =
    //     create_text_image("Shut up man", 128, 128).expect("Could not create text image");
    //
    // device
    //     .display_picture(text_image_data, 64, 64, 128, 128)
    //     .expect("Could not display image");
    //
    // for x in 1..11 {
    //     let playing_bar_image_data = create_playing_bar_to_rgb565(400, 10, x * 10)
    //         .expect("Could not create playing bar image");
    //
    //     device
    //         .display_picture(playing_bar_image_data, 40, 280, 400, 10)
    //         .expect("Could not display image");
    //
    //     tokio::time::sleep(Duration::from_secs(1)).await;
    // }

    // device.screen_black().expect("Could not set black color");
    // device.screen_white().expect("Could not set white color");
    // device.screen_off().expect("Could not set screen off");

    // API Integration
    // Soundcloud
    // Spotify
    // GitHub

    // let mut config = load_config("config.yaml");
    //
    // let token = match fetch_spotify_token(config.get_spotify_config()).await {
    //     Ok(token) => {
    //         println!("Token fetched successfully: {:?}", token);
    //         token
    //     }
    //     Err(e) => {
    //         eprintln!("Failed to fetch Spotify token: {}", e);
    //         return;
    //     }
    // };
    //
    // match fetch_currently_playing(&token).await {
    //     Ok(Some(data)) => {
    //         if let Some(track) = data.item {
    //             println!("Now Playing: {}", track.name);
    //             println!(
    //                 "Artists: {}",
    //                 track
    //                     .artists
    //                     .iter()
    //                     .map(|a| a.name.as_str())
    //                     .collect::<Vec<_>>()
    //                     .join(", ")
    //             );
    //             println!("Album: {}", track.album.name);
    //             println!("Link: {}", track.external_urls.spotify);
    //
    //             if let Some(image) = track.album.images.first() {
    //                 println!("Album Cover: {}", image.url);
    //             }
    //         } else {
    //             println!("No track is currently playing.");
    //         }
    //     }
    //     Ok(None) => println!("No track is currently playing."),
    //     Err(err) => eprintln!("Failed to fetch currently playing track: {}", err),
    // }
}
