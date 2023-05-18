use crate::circles::particle::*;
use nannou::prelude::*;

const MAX_OBJECTS: i32 = 5;
const FIVE_MINUTES: f32 = 300.0;
const TEN_MINUTES: f32 = 600.0;
const REALLY_LONG: f32 = 10000000000000000000000.0;

pub struct ObjectSystem {
    objects: Vec<Particle>,
    pub origin: Point2,
    pub width: f32,
    pub height: f32,
    pub next_update_time: f32,
}

impl ObjectSystem {
    pub fn new(position: Point2, width: f32, height: f32) -> Self {
        let origin = position;
        let objects = Vec::new();
        // let next_update_time = REALLY_LONG;
        let next_update_time = 1.0;
        ObjectSystem {
            objects,
            origin,
            width,
            height,
            next_update_time,
        }
    }

    pub fn add_object(&mut self) {
        self.objects.push(Particle::new(self.origin));
    }

    pub fn update(&mut self) {
        for i in (0..self.objects.len()).rev() {
            self.objects[i].update();
            if self.objects[i].is_dead() {
                self.objects.remove(i);
            }
        }
    }

    pub fn draw_all(&self, draw: &Draw) {
        for o in self.objects.iter() {
            o.draw(&draw);
        }
    }
}
