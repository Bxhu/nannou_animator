use crate::helpers::*;
use crate::object_system::*;
use nannou::prelude::*;

// TODO: Make something like this but directory 'eye' and have 'iris' and 'pupil'
pub struct Circle {
    id: u16,
    position: Point2,
    radius: f32,
    color: Rgba,
    is_alive: bool,
}

impl Drawable for Circle {
    fn new(id: u16, position: Point2) -> Self {
        let color = generate_random_color();
        let radius = 12.0;
        Circle {
            id,
            position,
            radius,
            color,
            is_alive: true,
        }
    }

    fn update(&mut self) {
        self.is_alive = still_alive();
    }

    fn draw(&self, draw: &Draw) {
        draw.ellipse().xy(self.position).radius(self.radius).rgba(
            self.color.red,
            self.color.green,
            self.color.blue,
            self.color.alpha,
        );
    }
}
