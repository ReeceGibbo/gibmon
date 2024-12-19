use std::path::Path;

pub fn generate_image_rgb565(width: usize, height: usize, pixel_color: u16) -> Vec<u8> {
    // Create a buffer for the entire image in RGB565 format
    let total_pixels = width * height;
    let mut image_buffer = vec![0u8; total_pixels * 2];

    // Fill the buffer with white pixels
    for i in 0..total_pixels {
        image_buffer[i * 2] = (pixel_color & 0xFF) as u8; // Little Endian - Low byte first
        image_buffer[i * 2 + 1] = ((pixel_color >> 8) & 0xFF) as u8; // High byte
    }

    image_buffer
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

    // Convert the image to RGB565 format
    let mut rgb565_data = Vec::with_capacity((target_width * target_height * 2) as usize);
    for pixel in resized_img.pixels() {
        let r = (pixel[0] as u16) >> 3; // 5 bits
        let g = (pixel[1] as u16) >> 2; // 6 bits
        let b = (pixel[2] as u16) >> 3; // 5 bits
        let rgb565 = (r << 11) | (g << 5) | b;
        rgb565_data.push((rgb565 & 0xFF) as u8); // Low byte
        rgb565_data.push((rgb565 >> 8) as u8); // High byte
    }

    Ok(rgb565_data)
}
