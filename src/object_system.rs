use crate::circles::circle::*;
// use crate::circles::particle::*;
use nannou::prelude::*;
use rand::prelude::*;
use std::collections::HashMap;
use std::num::Wrapping;

const MAX_OBJECTS: usize = 100000000000;
const FIVE_MINUTES: f32 = 300.0;
const TEN_MINUTES: f32 = 600.0;
const REALLY_LONG: f32 = 10000000000000000000000.0;

pub trait Drawable {
    fn new(position: Point2) -> Self;
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

    // Adds an object
    // Note: think about if I need id?
    pub fn add_object(&mut self) {
        self.objects.insert(
            self.next_id,
            Circle::new(self.generate_initial_object_position()),
        );
        self.next_id = Wrapping(self.next_id + 1).0;
    }

    fn generate_initial_object_position(&self) -> Point2 {
        // Note: use size enum or range here for width/height
        let mut rng = rand::thread_rng();
        let size = 12.0; // will only work for circle
        let half_width = self.width / 2.0;
        let half_height = self.height / 2.0;
        let position_x = rng.gen_range((-half_width + size)..(half_width - size));
        let position_y = rng.gen_range((-half_height + size)..(half_height - size));
        Point2::new(position_x, position_y)
    }

    // Every update
    pub fn update(&mut self, elapsed_time: f32) {
        self.update_objects();

        if self.objects.len() < MAX_OBJECTS && elapsed_time > self.next_update_time {
            self.add_object();

            self.next_update_time = elapsed_time + random_range(0.001, 0.002);
        }
    }

    // Calls update for each object
    fn update_objects(&mut self) {
        let mut object_ids_to_remove = Vec::new();
        for (key, object) in self.objects.iter_mut() {
            object.update();
            if object.has_died() {
                object_ids_to_remove.push(key.clone());
            }
        }

        // Note: Think about if I need ids
        // Note: I think originally I wanted children to be able to call things in object_system
        for object_id in object_ids_to_remove {
            self.objects.remove(&object_id);
        }
    }

    // Calls draw for each object
    pub fn draw_all(&self, draw: &Draw) {
        for value in self.objects.values() {
            value.draw(draw);
        }
    }
}
