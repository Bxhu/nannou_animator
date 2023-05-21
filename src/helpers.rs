use nannou::prelude::*;
use rand::prelude::*;

pub fn generate_random_color() -> Rgba {
    let mut rng = rand::thread_rng();
    let red = rng.gen::<f32>();
    let green = rng.gen::<f32>();
    let blue = rng.gen::<f32>();

    rgba(red, green, blue, 1.0)
}

pub fn still_alive() -> bool {
    let mut rng = rand::thread_rng();
    let random_num: u8 = rng.gen_range(0..=99);
    random_num < 99
}
