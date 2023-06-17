use crate::helpers::*;
use crate::object_system::*;
use nannou::prelude::*;

// TODO: Make something like this but directory 'eye' and have 'iris' and 'pupil'
pub struct Circle {
    position: Point2,
    radius: f32,
    color: Rgba,
    is_dead: bool,
}

impl Drawable for Circle {
    fn new(position: Point2) -> Self {
        let color = generate_random_color();
        let radius = 24.0;
        Circle {
            position,
            radius,
            color,
            is_dead: false,
        }
    }

    fn update(&mut self) {
        self.is_dead = has_died();
    }

    fn draw(&self, draw: &Draw) {
        draw.ellipse().xy(self.position).radius(self.radius).rgba(
            self.color.red,
            self.color.green,
            self.color.blue,
            self.color.alpha,
        );
    }

    fn has_died(&self) -> bool {
        self.is_dead
    }
}
