use nannou::prelude::*;

pub struct Circle {
    position: Point2,
    radius: f32,
    color: Rgba,
}

impl Circle {
    pub fn new(position: Point2, radius: f32) -> Self {
        Circle {
            position,
            radius,
            color,
        }
    }

    pub fn update(&self) {}

    pub fn draw(&self, draw: &Draw) {
        draw.ellipse()
            .xy(self.position)
            .radius(self.radius)
            .rgba(self.color)
    }
}
