use crate::circles::particle::*;
use nannou::prelude::*;

const MAX_OBJECTS: i32 = 5;

pub struct ObjectSystem {
    objects: Vec<Particle>,
    pub origin: Point2,
}

impl ObjectSystem {
    pub fn new(position: Point2) -> Self {
        let origin = position;
        let objects = Vec::new();
        ObjectSystem { origin, objects }
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

    pub fn draw(&self, draw: &Draw) {
        for o in self.objects.iter() {
            o.display(&draw);
        }
    }
}
