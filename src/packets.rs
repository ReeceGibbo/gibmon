use crate::device::Orientation;

enum PacketIds {
    Hello,
    ScreenWhite,
    ScreenBlack,
    ScreenOff,
    ScreenOn,
    Brightness,
    Orientation,
    DisplayImage,
}

impl PacketIds {
    fn get_id(&self) -> u8 {
        match self {
            PacketIds::Hello => 0xFF,
            PacketIds::ScreenWhite => 0x66,
            PacketIds::ScreenBlack => 0x67,
            PacketIds::ScreenOff => 0x6C,
            PacketIds::ScreenOn => 0x6D,
            PacketIds::Brightness => 0x6E,
            PacketIds::Orientation => 0x79,
            PacketIds::DisplayImage => 0xC5,
        }
    }
}

pub fn create_hello_packet() -> [u8; 6] {
    [0x00, 0x00, 0x00, 0x00, 0x00, PacketIds::Hello.get_id()]
}

pub fn create_screen_white_packet() -> [u8; 6] {
    [
        0x00,
        0x00,
        0x00,
        0x00,
        0x00,
        PacketIds::ScreenWhite.get_id(),
    ]
}

pub fn create_screen_black_packet() -> [u8; 6] {
    [
        0x00,
        0x00,
        0x00,
        0x00,
        0x00,
        PacketIds::ScreenBlack.get_id(),
    ]
}

pub fn create_screen_on_packet() -> [u8; 6] {
    [0x00, 0x00, 0x00, 0x00, 0x00, PacketIds::ScreenOn.get_id()]
}

pub fn create_screen_off_packet() -> [u8; 6] {
    [0x00, 0x00, 0x00, 0x00, 0x00, PacketIds::ScreenOff.get_id()]
}

pub fn create_screen_brightness_packet(level: u8) -> Result<[u8; 6], String> {
    // Validate brightness level
    if level > 100 || level < 1 {
        return Err("Brightness level must be between 0 and 100".to_string());
    }

    // Map brightness level (0-100) to display range (0-255)
    let level_absolute = 255 - ((level as f32 / 100.0) * 255.0).round() as u8;

    // Create the brightness packet
    let mut packet = [0u8; 6];
    packet[0] = (level_absolute >> 2);
    packet[1] = ((level_absolute & 0x03) << 6);
    packet[5] = PacketIds::Brightness.get_id();

    Ok(packet)
}

pub fn create_orientation_packet(orientation: Orientation, width: u16, height: u16) -> [u8; 16] {
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
    packet[5] = PacketIds::Orientation.get_id();
    packet[6] = orientation.get_id() + 100;
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

pub fn create_display_image_packet(x: u16, y: u16, width: u16, height: u16) -> [u8; 6] {
    // Create a 6-byte buffer
    let mut packet = [0u8; 6];

    let (ex, ey) = (x + width - 1, y + height - 1);

    packet[0] = (x >> 2) as u8;
    packet[1] = (((x & 3) << 6) + (y >> 4)) as u8;
    packet[2] = (((y & 15) << 4) + (ex >> 6)) as u8;
    packet[3] = (((ex & 63) << 2) + (ey >> 8)) as u8;
    packet[4] = (ey & 255) as u8;
    packet[5] = PacketIds::DisplayImage.get_id();

    packet
}
