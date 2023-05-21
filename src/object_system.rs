use crate::circles::circle::*;
use crate::circles::particle::*;
use nannou::prelude::*;
use std::collections::HashMap;
use std::num::Wrapping;

const MAX_OBJECTS: i32 = 5;
const FIVE_MINUTES: f32 = 300.0;
const TEN_MINUTES: f32 = 600.0;
const REALLY_LONG: f32 = 10000000000000000000000.0;

pub trait Drawable {
    fn new(id: u16, position: Point2) -> Self;
    fn update(&mut self);
    fn draw(&self, draw: &Draw);
    fn has_died(&self) -> bool;
}

pub struct ObjectSystem {
    objects: HashMap<u16, Circle>,
    pub origin: Point2,
    pub width: f32,
    pub height: f32,
    pub next_update_time: f32,
    pub next_id: u16,
}

impl ObjectSystem {
    pub fn new(position: Point2, width: f32, height: f32) -> Self {
        let origin = position;
        let objects = HashMap::new();
        let next_id = 0;
        // let next_update_time = REALLY_LONG;
        let next_update_time = 1.0;
        ObjectSystem {
            objects,
            origin,
            width,
            height,
            next_update_time,
            next_id,
        }
    }

    pub fn add_object(&mut self) {
        self.objects
            .insert(self.next_id, Circle::new(self.next_id, self.origin));
        self.next_id = Wrapping(self.next_id + 1).0;
    }

    pub fn update(&mut self) {
        let mut object_ids_to_remove = Vec::new();
        for (key, value) in self.objects.iter_mut() {
            value.update();
            if value.has_died() {
                object_ids_to_remove.push(key.clone());
            }
        }

        for object_id in object_ids_to_remove {
            self.objects.remove(&object_id);
        }
    }

    pub fn draw_all(&self, draw: &Draw) {
        for value in self.objects.values() {
            value.draw(draw);
        }
    }
}
