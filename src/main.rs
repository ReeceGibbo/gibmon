use std::time::Duration;

fn main() {
    // Specify the serial port name and baud rate
    let port_name = "COM4"; // Update this to your serial port
    let baud_rate = 9600;

    // Create the packet to send
    let packet = hello_packet();
    let brightness_packet = screen_brightness_packet(50);
    let screen_on_packet = screen_on_packet();
    let orientation_packet = orientation_packet(3, 480, 320);
    let pre_image_packet = pre_image_packet(0, 0, 480, 320);

    // Open the serial port
    match serialport::new(port_name, baud_rate)
        .timeout(Duration::from_secs(1))
        .open()
    {
        Ok(mut port) => {
            println!("Serial port opened successfully.");

            // Send the hello packet
            match port.write_all(&packet) {
                Ok(_) => println!("Packet sent successfully: {:?}", packet),
                Err(e) => eprintln!("Failed to send packet: {:?}", e),
            }

            // Send the screen brightness packet
            match port.write_all(&brightness_packet) {
                Ok(_) => println!("Packet sent successfully: {:?}", brightness_packet),
                Err(e) => eprintln!("Failed to send packet: {:?}", e),
            }

            // Send the screen on packet
            match port.write_all(&screen_on_packet) {
                Ok(_) => println!("Packet sent successfully: {:?}", screen_on_packet),
                Err(e) => eprintln!("Failed to send packet: {:?}", e),
            }

            // Send the orientation packet
            match port.write_all(&orientation_packet) {
                Ok(_) => println!("Packet sent successfully: {:?}", orientation_packet),
                Err(e) => eprintln!("Failed to send packet: {:?}", e),
            }

            // Send the pre-image packet
            match port.write_all(&pre_image_packet) {
                Ok(_) => println!("Packet sent successfully: {:?}", pre_image_packet),
                Err(e) => eprintln!("Failed to send packet: {:?}", e),
            }

            // Send image
            let image_data = generate_image_rgb565(480, 320, 0xFFFF);

            // Replace this with your command to send the image data
            let total_size = image_data.len();
            let rows_per_chunk = 2;
            let chunk_size = 480 * 2 * rows_per_chunk;
            let mut start = 0;

            while start < total_size {
                let end = (start + chunk_size).min(total_size); // Avoid overflow
                let chunk = &image_data[start..end];
                println!("Sending chunk: {:?}", chunk); // Replace this with actual sending logic
                match port.write_all(&chunk) {
                    Ok(_) => println!("Packet sent successfully: {:?}", chunk),
                    Err(e) => eprintln!("Failed to send packet: {:?}", e),
                }
                start = end;
            }

            println!("Finished sending image data.");

            // Send the pre-image packet
            match port.write_all(&pre_image_packet) {
                Ok(_) => println!("Packet sent successfully: {:?}", pre_image_packet),
                Err(e) => eprintln!("Failed to send packet: {:?}", e),
            }

            // Send image
            let image_data = generate_image_rgb565(480, 320, 0x0000);

            // Replace this with your command to send the image data
            let total_size = image_data.len();
            let mut start = 0;

            while start < total_size {
                let end = (start + chunk_size).min(total_size); // Avoid overflow
                let chunk = &image_data[start..end];
                println!("Sending chunk: {:?}", chunk); // Replace this with actual sending logic
                match port.write_all(&chunk) {
                    Ok(_) => println!("Packet sent successfully: {:?}", chunk),
                    Err(e) => eprintln!("Failed to send packet: {:?}", e),
                }
                start = end;
            }

            println!("Finished sending image data.");


        }
        Err(e) => eprintln!("Failed to open serial port: {:?}", e),
    }
}

fn hello_packet() -> [u8; 6] {
    [0x00, 0x00, 0x00, 0x00, 0x00, 0xFF]
}

fn screen_brightness_packet(level: u8) -> [u8; 6] {
    assert!(level <= 100, "Brightness level must be between 0 and 100");

    // Map brightness level (0-100) to display range (0-255)
    let level_absolute = 255 - ((level as f32 / 100.0) * 255.0).round() as u8;

    // Create the brightness packet
    let mut packet = [0u8; 6];
    packet[0] = (level_absolute >> 2);
    packet[1] = ((level_absolute & 0x03) << 6);
    packet[5] = 0x6E;

    packet
}

fn screen_on_packet() -> [u8; 6] {
    [0x00, 0x00, 0x00, 0x00, 0x00, 0x6D]
}

fn orientation_packet(orientation: u8, width: u16, height: u16) -> [u8; 16] {
    // Validate the orientation value
    assert!(orientation <= 3, "Orientation must be between 0 and 3");

    // Initialize coordinates (x, y, ex, ey)
    let x: u16 = 0;
    let y: u16 = 0;
    let ex: u16 = 0;
    let ey: u16 = 0;

    // Create the packet
    let mut packet = [0u8; 16];
    packet[0] = (x >> 2) as u8;
    packet[1] = (((x & 3) << 6) + (y >> 4)) as u8;
    packet[2] = (((y & 15) << 4) + (ex >> 6)) as u8;
    packet[3] = (((ex & 63) << 2) + (ey >> 8)) as u8;
    packet[4] = (ey & 255) as u8;
    packet[5] = 0x79; // Command for SET_ORIENTATION
    packet[6] = orientation + 100;
    packet[7] = (width >> 8) as u8;
    packet[8] = (width & 255) as u8;
    packet[9] = (height >> 8) as u8;
    packet[10] = (height & 255) as u8;

    // The rest of the packet is padding (0x00)
    for i in 11..16 {
        packet[i] = 0x00;
    }

    packet
}

fn pre_image_packet(x: u16, y: u16, width: u16, height: u16) -> [u8; 6] {
    // Create a 6-byte buffer
    let mut byte_buffer = [0u8; 6];

    let (ex, ey) = (x + width - 1, y + height - 1);

    // Populate the buffer with the encoded values
    byte_buffer[0] = (x >> 2) as u8;
    byte_buffer[1] = (((x & 3) << 6) + (y >> 4)) as u8;
    byte_buffer[2] = (((y & 15) << 4) + (ex >> 6)) as u8;
    byte_buffer[3] = (((ex & 63) << 2) + (ey >> 8)) as u8;
    byte_buffer[4] = (ey & 255) as u8;
    byte_buffer[5] = 0xC5;

    // Return the generated packet
    byte_buffer
}

fn generate_image_rgb565(width: usize, height: usize, pixel_color: u16) -> Vec<u8> {
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
