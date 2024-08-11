extern {
    fn js_random() -> f32;
}

fn random() -> f32 {
    unsafe {
        js_random()
    }
}

pub fn rnd(max: u32) -> u32 {
    (random() * max as f32) as u32
}

// avoid std to minimise binary
pub fn min(v1: usize, v2: usize) -> usize {
    if v1 < v2 { v1 } else { v2 }
}
