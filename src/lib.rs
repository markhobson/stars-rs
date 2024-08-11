use core::sync::atomic::{AtomicU32, Ordering};
use crate::colour::Colour;

mod colour;

const WIDTH: usize = 600;
const HEIGHT: usize = 600;
const STAR_COUNT: usize = 100;

extern {
    fn js_random() -> f32;
}

fn random() -> f32 {
    unsafe {
        js_random()
    }
}

fn rnd(max: u32) -> u32 {
    (random() * max as f32) as u32
}

struct Star {
    x0: u32,
    y0: u32,
    dx: u8,
    size: u8,
    pixel: u32,
}

impl Star {
    fn new() -> Self {
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

    fn render(&self, buffer: &mut [u32; WIDTH * HEIGHT], f: u32) {
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

const DEFAULT_STAR: Star = Star { x0: 0, y0: 0, dx: 0, size: 0, pixel: 0 };

static mut STARS: [Star; STAR_COUNT] = [DEFAULT_STAR; STAR_COUNT];

static FRAME: AtomicU32 = AtomicU32::new(0);

#[no_mangle]
static mut BUFFER: [u32; WIDTH * HEIGHT] = [0; WIDTH * HEIGHT];

#[no_mangle]
pub unsafe extern fn go() {
    // This is called from JavaScript, and should *only* be
    // called from JavaScript. If you maintain that condition,
    // then we know that the &mut we're about to produce is
    // unique, and therefore safe.
    render_frame_safe(&mut BUFFER, &mut STARS)
}

// We split this out so that we can escape 'unsafe' as quickly
// as possible.
fn render_frame_safe(buffer: &mut [u32; WIDTH * HEIGHT], stars: &mut [Star; STAR_COUNT]) {
    let f = FRAME.fetch_add(1, Ordering::Relaxed);

    if f == 0 {
        initialize(stars);
    }

    clear_frame(buffer);

    for star in stars {
        star.render(buffer, f);
    }
}

fn initialize(stars: &mut [Star; STAR_COUNT]) {
    for index in 0..STAR_COUNT {
        stars[index] = Star::new()
    }
}

fn clear_frame(buffer: &mut [u32; WIDTH * HEIGHT]) {
    let colour = Colour::from(0, 0, 0);

    for pixel in buffer.iter_mut() {
        *pixel = colour.pixel();
    }
}

// avoid std to minimise binary
fn min(v1: usize, v2: usize) -> usize {
    if v1 < v2 { v1 } else { v2 }
}
