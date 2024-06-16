use core::sync::atomic::{AtomicU32, Ordering};

const WIDTH: usize = 600;
const HEIGHT: usize = 600;
const STAR_COUNT: usize = 3;

struct Star {
    x0: u32,
    y0: u32,
    dx: u32,
}

const STARS: [Star; STAR_COUNT] = [
    Star { x0: 100, y0: 100, dx: 1},
    Star { x0: 500, y0: 300, dx: 3},
    Star { x0: 200, y0: 500, dx: 2},
];

static FRAME: AtomicU32 = AtomicU32::new(0);

#[no_mangle]
static mut BUFFER: [u32; WIDTH * HEIGHT] = [0; WIDTH * HEIGHT];

#[no_mangle]
pub unsafe extern fn go() {
    // This is called from JavaScript, and should *only* be
    // called from JavaScript. If you maintain that condition,
    // then we know that the &mut we're about to produce is
    // unique, and therefore safe.
    render_frame_safe(&mut BUFFER)
}

// We split this out so that we can escape 'unsafe' as quickly
// as possible.
fn render_frame_safe(buffer: &mut [u32; WIDTH * HEIGHT]) {
    let f = FRAME.fetch_add(1, Ordering::Relaxed);

    clear_frame(buffer);

    for star in STARS {
        let x: usize = (star.x0 + f * star.dx) as usize % WIDTH;
        let y = star.y0 as usize;
        render_star(buffer, x, y);
    }
}

fn clear_frame(buffer: &mut [u32; WIDTH * HEIGHT]) {
    let colour = rgb(0, 0, 0);

    for pixel in buffer.iter_mut() {
        *pixel = colour;
    }
}

fn render_star(buffer: &mut [u32; WIDTH * HEIGHT], x0: usize, y0: usize) {
    let colour = rgb(255, 255, 255);
    let size = 3;

    for y in y0..min(y0 + size, HEIGHT) {
        for x in x0..min(x0 + size, WIDTH) {
            buffer[y * WIDTH + x] = colour;
        }
    }
}

fn rgb(r: u8, g: u8, b: u8) -> u32 {
    return rgba(r, g, b, 0xFF)
}

fn rgba(r: u8, g: u8, b: u8, a: u8) -> u32 {
    return ((a as u32) << 24) | ((b as u32) << 16) | ((g as u32) << 8) | (r as u32);
}

// avoid std to minimise binary
fn min(v1: usize, v2: usize) -> usize {
    return if v1 < v2 { v1 } else { v2 };
}
