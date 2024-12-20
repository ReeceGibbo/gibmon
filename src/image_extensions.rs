use ab_glyph::{FontRef, PxScale};
use image::imageops::{FilterType, resize};
use image::{self, ImageBuffer, Rgb, RgbImage, Rgba};
use imageproc::drawing::{draw_filled_rect_mut, draw_text_mut, text_size};
use imageproc::rect::Rect;
use reqwest;
use std::error::Error;
use std::path::Path;

fn convert_to_rgb565(
    img: &ImageBuffer<Rgb<u8>, Vec<u8>>,
    target_width: u32,
    target_height: u32,
) -> Vec<u8> {
    let mut rgb565_data = Vec::with_capacity((target_width * target_height * 2) as usize);
    for pixel in img.pixels() {
        let r = (pixel[0] as u16) >> 3; // 5 bits
        let g = (pixel[1] as u16) >> 2; // 6 bits
        let b = (pixel[2] as u16) >> 3; // 5 bits

        let rgb565 = (r << 11) | (g << 5) | b;
        rgb565_data.push((rgb565 & 0xFF) as u8); // Low byte
        rgb565_data.push((rgb565 >> 8) as u8); // High byte
    }
    rgb565_data
}

pub fn load_png_to_rgb565<P: AsRef<Path>>(
    path: P,
    target_width: u32,
    target_height: u32,
) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    // Load the PNG image
    let img = image::open(path)?.to_rgb8();

    // Resize the image
    let resized_img =
        image::imageops::resize(&img, target_width, target_height, image::imageops::Lanczos3);

    Ok(convert_to_rgb565(&resized_img, target_width, target_height))
}

pub async fn load_image_from_url_to_rgb565(
    url: &str,
    target_width: u32,
    target_height: u32,
) -> Result<Vec<u8>, Box<dyn Error>> {
    // Fetch the image bytes from the URL using a blocking request
    let resp = reqwest::get(url).await?.error_for_status()?;
    let bytes = resp.bytes().await?;

    // Decode the image from memory (this automatically handles PNG, JPEG, etc.)
    let img = image::load_from_memory(&bytes)?.to_rgb8();

    // Resize the image
    let resized_img = resize(&img, target_width, target_height, FilterType::Lanczos3);

    Ok(convert_to_rgb565(&resized_img, target_width, target_height))
}

pub fn create_playing_bar_to_rgb565(
    target_width: u32,
    target_height: u32,
    progress: u8,
) -> Result<Vec<u8>, Box<dyn Error>> {
    // Validate brightness level
    if progress > 100 || progress < 1 {
        return Err(Box::from(
            "Progress level must be between 0 and 100".to_string(),
        ));
    }

    let mut img: RgbImage = image::ImageBuffer::new(target_width, target_height);

    // Fill background with u8 literal suffixes
    for pixel in img.pixels_mut() {
        *pixel = Rgb([16, 16, 16]);
    }

    // Set the progress
    let progress_width = (target_width as f64 * (progress as f64 / 100f64)) as u32;

    draw_filled_rect_mut(
        &mut img,
        Rect::at(0, 0).of_size(progress_width, 50),
        Rgb([255, 255, 255]),
    );

    Ok(convert_to_rgb565(&img, target_width, target_height))
}

pub fn create_text_image(
    text: &str,
    target_width: u32,
    target_height: u32,
) -> Result<Vec<u8>, Box<dyn Error>> {
    // Create a white background image
    let mut img = RgbImage::new(target_width, target_height);

    // Fill background with u8 literal suffixes
    for pixel in img.pixels_mut() {
        *pixel = Rgb([255, 255, 255]);
    }

    // Load the font data
    let font_data = include_bytes!("../assets/fonts/ARIAL.TTF");
    let font = FontRef::try_from_slice(font_data)?;

    let height = 12.0;
    let scale = PxScale {
        x: height * 1.2,
        y: height,
    };

    // Draw the text in black
    draw_text_mut(&mut img, Rgb([0, 0, 0]), 0, 0, scale, &font, text);

    let (w, h) = text_size(scale, &font, text);
    println!("Text size: {}x{}", w, h);

    Ok(convert_to_rgb565(&img, target_width, target_height))
}

pub async fn load_image_from_url_to_rgba(
    url: &str,
    target_width: u32,
    target_height: u32,
) -> Result<Vec<u8>, Box<dyn Error>> {
    // Fetch the image bytes from the URL using a blocking request
    let resp = reqwest::get(url).await?.error_for_status()?;
    let bytes = resp.bytes().await?;

    // Decode the image from memory (this automatically handles PNG, JPEG, etc.)
    let img = image::load_from_memory(&bytes)?.to_rgba8();

    // Resize the image
    let resized_img = resize(&img, target_width, target_height, FilterType::Lanczos3);

    Ok(resized_img.to_vec())
}
