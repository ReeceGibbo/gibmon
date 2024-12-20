use crate::image_extensions::load_png_to_rgb565;
use crate::packets::{
    create_display_image_packet, create_hello_packet, create_orientation_packet,
    create_screen_black_packet, create_screen_brightness_packet, create_screen_off_packet,
    create_screen_on_packet, create_screen_white_packet,
};
use serialport::SerialPort;
use std::fmt;
use std::time::Duration;

#[derive(Debug)]
pub enum DeviceError {
    IoError(std::io::Error),
    PacketError(String),
    ImageError(String),
}

impl fmt::Display for DeviceError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DeviceError::IoError(e) => write!(f, "I/O error: {}", e),
            DeviceError::PacketError(msg) => write!(f, "Packet error: {}", msg),
            DeviceError::ImageError(msg) => write!(f, "Image error: {}", msg),
        }
    }
}

impl std::error::Error for DeviceError {}

pub enum Orientation {
    Portrait,
    Landscape,
    ReversePortrait,
    ReverseLandscape,
}

impl Orientation {
    pub fn get_id(&self) -> u8 {
        match self {
            Orientation::Portrait => 0,
            Orientation::ReversePortrait => 1,
            Orientation::Landscape => 2,
            Orientation::ReverseLandscape => 3,
        }
    }
}

pub struct Device {
    width: u16,
    height: u16,
    device_width: u16,
    device_height: u16,
    serial_connection: Box<dyn SerialPort>,
}

impl Device {
    pub fn new() -> Result<Self, DeviceError> {
        let width = 320;
        let height = 480;
        let port_name = "COM4";
        let baud_rate = 115200;

        let open_serial = serialport::new(port_name, baud_rate)
            .timeout(Duration::from_secs(3))
            .open()
            .map_err(|err| DeviceError::PacketError(format!("Packet error: {}", err)))?;

        let mut device = Self {
            width,
            height,
            device_width: width,
            device_height: height,
            serial_connection: open_serial,
        };

        device.init()?;

        Ok(device)
    }

    fn init(&mut self) -> Result<(), DeviceError> {
        let hello_packet = create_hello_packet();
        self.serial_connection
            .write_all(&hello_packet)
            .map_err(DeviceError::IoError)?;

        self.set_brightness(10)?;
        self.screen_on()?;
        self.set_brightness(10)?;
        self.set_orientation(Orientation::Portrait)
    }

    pub fn screen_black(&mut self) -> Result<(), DeviceError> {
        self.set_orientation(Orientation::Portrait)?;

        let screen_black_packet = create_screen_black_packet();
        self.serial_connection
            .write_all(&screen_black_packet)
            .map_err(DeviceError::IoError)
    }

    pub fn screen_white(&mut self) -> Result<(), DeviceError> {
        self.set_orientation(Orientation::Portrait)?;

        let screen_white_packet = create_screen_white_packet();
        self.serial_connection
            .write_all(&screen_white_packet)
            .map_err(DeviceError::IoError)
    }

    pub fn screen_on(&mut self) -> Result<(), DeviceError> {
        let on_packet = create_screen_on_packet();

        self.serial_connection
            .write_all(&on_packet)
            .map_err(DeviceError::IoError)
    }

    pub fn screen_off(&mut self) -> Result<(), DeviceError> {
        let off_packet = create_screen_off_packet();

        self.serial_connection
            .write_all(&off_packet)
            .map_err(DeviceError::IoError)
    }

    pub fn set_brightness(&mut self, brightness: u8) -> Result<(), DeviceError> {
        let brightness_packet = create_screen_brightness_packet(brightness)
            .map_err(|err| DeviceError::PacketError(format!("Packet error: {}", err)))?;

        self.serial_connection
            .write_all(&brightness_packet)
            .map_err(DeviceError::IoError)
    }

    pub fn set_orientation(&mut self, orientation: Orientation) -> Result<(), DeviceError> {
        match orientation {
            Orientation::Portrait | Orientation::ReversePortrait => {
                self.width = self.device_width;
                self.height = self.device_height;
            }
            Orientation::Landscape | Orientation::ReverseLandscape => {
                self.width = self.device_height;
                self.height = self.device_width;
            }
        }

        let orientation_packet = create_orientation_packet(orientation, self.width, self.height);

        self.serial_connection
            .write_all(&orientation_packet)
            .map_err(DeviceError::IoError)
    }

    pub fn set_background_picture(&mut self, path: &str) -> Result<(), DeviceError> {
        let display_packet = create_display_image_packet(0, 0, self.width, self.height);

        self.serial_connection
            .write_all(&display_packet)
            .map_err(DeviceError::IoError)?;

        let image_data = load_png_to_rgb565(path, self.width as u32, self.height as u32)
            .map_err(|err| DeviceError::ImageError(format!("Image error: {}", err)))?;

        // Replace this with your command to send the image data
        let total_size = image_data.len();
        let rows_per_chunk = 2;
        let chunk_size = 480 * 2 * rows_per_chunk;
        let mut start = 0;

        while start < total_size {
            let end = (start + chunk_size).min(total_size);
            let chunk = &image_data[start..end];

            self.serial_connection
                .write_all(&chunk)
                .map_err(DeviceError::IoError)?;

            start = end;
        }
        Ok(())
    }

    pub fn display_picture(
        &mut self,
        image_data: Vec<u8>,
        x: u16,
        y: u16,
        width: u16,
        height: u16,
    ) -> Result<(), DeviceError> {
        let display_packet = create_display_image_packet(x, y, width, height);

        self.serial_connection
            .write_all(&display_packet)
            .map_err(DeviceError::IoError)?;

        let total_size = image_data.len();
        let rows_per_chunk = 2;
        let chunk_size = 480 * 2 * rows_per_chunk;
        let mut start = 0;

        while start < total_size {
            let end = (start + chunk_size).min(total_size);
            let chunk = &image_data[start..end];

            self.serial_connection
                .write_all(&chunk)
                .map_err(DeviceError::IoError)?;

            start = end;
        }
        Ok(())
    }

    pub fn get_width(&self) -> u16 {
        self.width
    }

    pub fn get_height(&self) -> u16 {
        self.height
    }
}
