use crate::colour::Colour;

pub const WIDTH: usize = 600;
pub const HEIGHT: usize = 600;

#[no_mangle]
pub static mut BUFFER: [u32; WIDTH * HEIGHT] = [0; WIDTH * HEIGHT];

pub fn clear_frame(buffer: &mut [u32; WIDTH * HEIGHT]) {
    let background = Colour::from(0, 0, 0).pixel();

    for pixel in buffer.iter_mut() {
        *pixel = background;
    }
}
