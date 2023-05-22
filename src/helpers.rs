use nannou::prelude::*;
use rand::prelude::*;

pub fn generate_random_color() -> Rgba {
    let mut rng = rand::thread_rng();
    let red = rng.gen::<f32>();
    let green = rng.gen::<f32>();
    let blue = rng.gen::<f32>();

    rgba(red, green, blue, 1.0)
}

pub fn has_died() -> bool {
    let mut rng = rand::thread_rng();
    let max = 10000000;
    let random_num: u32 = rng.gen_range(1..=max);
    random_num == max
}
