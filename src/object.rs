use nannou::prelude::*;

pub trait Animating {
    fn draw(&self, draw: &Draw);
}
