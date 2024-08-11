use crate::math::{min, rnd};
use crate::{HEIGHT, WIDTH};
use crate::colour::Colour;

pub const DEFAULT_STAR: Star = Star { x0: 0, y0: 0, dx: 0, size: 0, pixel: 0 };

pub struct Star {
    x0: u32,
    y0: u32,
    dx: u8,
    size: u8,
    pixel: u32,
}

impl Star {
    pub fn new() -> Self {
        const MAX_Z: u8 = 2;
        let z = rnd(1 + MAX_Z as u32) as u8;
        let brightness = 0x80 + (0x7F * z / MAX_Z);

        Self {
            x0: rnd(WIDTH as u32),
            y0: rnd(HEIGHT as u32),
            dx: 1 + z,
            size: 2 + z,
            pixel: Colour::from(brightness, brightness, brightness).pixel(),
        }
    }

    pub fn render(&self, buffer: &mut [u32; WIDTH * HEIGHT], f: u32) {
        let x0 = (self.x0 + f * self.dx as u32) % WIDTH as u32;

        for v in 0..self.size as u32 {
            let y = (self.y0 + v) as usize;
            if y >= HEIGHT {
                break;
            }

            let d = if v < self.size as u32 / 2 { 1 } else { 0 };
            let c = (self.size - d) / 2;

            let u0 = (c as i32 - v as i32).abs() as u32;
            let u1 = self.size as u32 - u0;

            Self::render_raster(buffer, (x0 + u0) as usize, (x0 + u1) as usize, y, self.pixel);
        }
    }

    fn render_raster(buffer: &mut [u32; WIDTH * HEIGHT], x0: usize, x1: usize, y: usize, pixel: u32) {
        for x in x0..min(x1, WIDTH) {
            buffer[y * WIDTH + x] = pixel;
        }
    }
}
