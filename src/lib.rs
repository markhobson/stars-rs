use core::sync::atomic::{AtomicU32, Ordering};

const WIDTH: usize = 600;
const HEIGHT: usize = 600;
const STARS: usize = 3;

static FRAME: AtomicU32 = AtomicU32::new(0);
const STAR_X0: [u32; STARS] = [100, 500, 200];
const STAR_Y0: [u32; STARS] = [100, 300, 500];
const STAR_DX: [u32; STARS] = [1, 3, 2];

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

    for star in 0..STARS {
        let x: usize = (STAR_X0[star] + f * STAR_DX[star]) as usize % WIDTH;
        let y = STAR_Y0[star] as usize;
        render_star(buffer, x, y);
    }
}

fn clear_frame(buffer: &mut [u32; WIDTH * HEIGHT]) {
    for pixel in buffer.iter_mut() {
        *pixel = 0xFF_00_00_00;
    }
}

fn render_star(buffer: &mut [u32; WIDTH * HEIGHT], x0: usize, y0: usize) {
    let colour = 0xFF_FF_FF_FF;
    let size = 3;

    for y in y0..min(y0 + size, HEIGHT) {
        for x in x0..min(x0 + size, WIDTH) {
            buffer[y * WIDTH + x] = colour;
        }
    }
}

// avoid std to minimise binary
fn min(v1: usize, v2: usize) -> usize {
    return if v1 < v2 { v1 } else { v2 };
}
