use crate::helpers::*;
use crate::object_system::*;
use nannou::prelude::*;

// TODO: Make something like this but directory 'eye' and have 'iris' and 'pupil'
pub struct Circle {
    position: Point2,
    radius: f32,
    color: Rgba,
    inner_circle: Inner_Circle,
    is_dead: bool,
}

pub struct Inner_Circle {
    outer_circle_origin: Point2,
    outer_circle_radius: f32,
    position: Point2,
    radius: f32,
    color: Rgba,
}

impl Inner_Circle {
    // TODO: Use enums for color
    fn new(position: Point2, outer_circle_origin: Point2, outer_circle_radius: f32) -> Self {
        let color = generate_random_color();
        let radius = 8.0;
        Inner_Circle {
            outer_circle_origin,
            outer_circle_radius,
            position,
            radius,
            color,
        }
    }

    fn update(&mut self, focus_location: Point2) {
        let dx = focus_location.x - self.outer_circle_origin.x;
        let dy = focus_location.y - self.outer_circle_origin.y;
        let distance = (dx * dx + dy * dy).sqrt();

        if distance + self.radius > self.outer_circle_radius {
            let angle = dy.atan2(dx);
            let x =
                self.outer_circle_origin.x + (self.outer_circle_radius - self.radius) * angle.cos();
            let y =
                self.outer_circle_origin.y + (self.outer_circle_radius - self.radius) * angle.sin();
            self.position = pt2(x, y);
        } else {
            self.position = focus_location;
        }
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

impl Drawable for Circle {
    // TODO: Use enums for color
    fn new(position: Point2) -> Self {
        // let color = generate_random_color();
        let color = rgba(255.0, 255.0, 255.0, 1.0);
        let radius = 32.0;
        let inner_circle = Inner_Circle::new(position, position, radius);
        Circle {
            position,
            radius,
            color,
            inner_circle,
            is_dead: false,
        }
    }

    fn update(&mut self, focus_location: Point2) {
        self.is_dead = has_died();
        self.inner_circle.update(focus_location);
    }

    fn draw(&self, draw: &Draw) {
        draw.ellipse()
            .xy(self.position)
            .radius(self.radius)
            .rgba(
                self.color.red,
                self.color.green,
                self.color.blue,
                self.color.alpha,
            )
            .stroke(rgba(0.0, 0.0, 0.0, 1.0))
            .stroke_weight(1.0);

        self.inner_circle.draw(draw);
    }

    fn has_died(&self) -> bool {
        self.is_dead
    }
}
