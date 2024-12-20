use crate::device::Device;
use crate::r#virtual::layer::Layer;
use std::sync::{Arc, Mutex};

pub struct Display {
    width: u32,
    height: u32,
    buffer: Vec<u8>, // RGBA Buffer: width * height * 4
    device_ref: Arc<Mutex<Device>>,
    layers: Vec<(u32, Arc<Mutex<dyn Layer + Send + Sync>>)>,
}

impl Display {
    pub fn new(device: Arc<Mutex<Device>>) -> Self {
        let (width, height) = {
            let dev = device.lock().unwrap();
            (dev.get_width(), dev.get_height())
        };
        let buffer = vec![0; (width as u32 * height as u32 * 4) as usize];

        Display {
            width: width as u32,
            height: height as u32,
            buffer,
            device_ref: device,
            layers: Vec::new(),
        }
    }

    pub fn add_layer(&mut self, order: u32, layer: Arc<Mutex<dyn Layer + Send + Sync>>) {
        self.layers.push((order, layer));
    }

    fn draw_pixel(&mut self, x: u32, y: u32, rgba: [u8; 4]) {
        if x >= self.width || y >= self.height {
            return;
        }
        let idx = ((y * self.width + x) * 4) as usize;
        let dst = &mut self.buffer[idx..idx + 4];
        let src_a = rgba[3] as f32 / 255.0;
        let inv_a = 1.0 - src_a;
        dst[0] = (rgba[0] as f32 * src_a + dst[0] as f32 * inv_a) as u8;
        dst[1] = (rgba[1] as f32 * src_a + dst[1] as f32 * inv_a) as u8;
        dst[2] = (rgba[2] as f32 * src_a + dst[2] as f32 * inv_a) as u8;
        // If you care about alpha channel in the framebuffer:
        dst[3] = 255; // final image opaque or keep alpha if needed
    }

    fn as_rgb565_subregion(&self, x: u32, y: u32, w: u32, h: u32) -> Vec<u8> {
        let mut output = Vec::with_capacity((w * h * 2) as usize);

        for row in y..(y + h) {
            let start = ((row * self.width + x) * 4) as usize;
            let end = start + (w * 4) as usize;
            let row_data = &self.buffer[start..end];

            for pixel in row_data.chunks_exact(4) {
                let r = pixel[0] as u16;
                let g = pixel[1] as u16;
                let b = pixel[2] as u16;
                // Convert to RGB565
                let rgb565 = ((r & 0xF8) << 8) | ((g & 0xFC) << 3) | (b >> 3);
                output.push((rgb565 & 0xFF) as u8);
                output.push((rgb565 >> 8) as u8);
            }
        }

        output
    }

    pub fn redraw_full(&mut self) {
        // Clear entire display
        self.buffer.fill(0);

        self.layers.sort_by_key(|(order, _)| *order);
        let layers = self.layers.clone();

        for (order, layer) in layers {
            let l = layer.lock().unwrap();
            let layer_data = l.get_image_data();
            let (lx, ly, lw, lh) = l.bounding_box();

            // Blend each pixel of the layer onto the display buffer
            for row in 0..lh {
                for col in 0..lw {
                    let idx = ((row * lw + col) * 4) as usize;
                    let pixel = [
                        layer_data[idx],
                        layer_data[idx + 1],
                        layer_data[idx + 2],
                        layer_data[idx + 3],
                    ];

                    self.draw_pixel(lx + col, ly + row, pixel);
                }
            }
        }

        let full_data = self.as_rgb565_subregion(0, 0, self.width, self.height);
        self.device_ref
            .lock()
            .unwrap()
            .display_picture(full_data, 0, 0, self.width as u16, self.height as u16)
            .expect("Failed to update full display");
    }
}

fn rects_intersect(x1: u32, y1: u32, w1: u32, h1: u32, x2: u32, y2: u32, w2: u32, h2: u32) -> bool {
    let (r1x2, r1y2) = (x1 + w1, y1 + h1);
    let (r2x2, r2y2) = (x2 + w2, y2 + h2);
    !(r2x2 <= x1 || x2 >= r1x2 || r2y2 <= y1 || y2 >= r1y2)
}
