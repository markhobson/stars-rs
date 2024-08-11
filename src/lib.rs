use core::sync::atomic::{AtomicU32, Ordering};
use crate::buffer::{clear_buffer, BUFFER, HEIGHT, WIDTH};
use crate::star::{DEFAULT_STAR, Star};

mod buffer;
mod colour;
mod math;
mod star;

const STAR_COUNT: usize = 100;

static mut STARS: [Star; STAR_COUNT] = [DEFAULT_STAR; STAR_COUNT];

static FRAME: AtomicU32 = AtomicU32::new(0);

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

    clear_buffer(buffer);

    for star in stars {
        star.render(buffer, f);
    }
}

fn initialize(stars: &mut [Star; STAR_COUNT]) {
    for index in 0..STAR_COUNT {
        stars[index] = Star::new()
    }
}
