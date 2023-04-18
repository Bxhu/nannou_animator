use nannou::prelude::*;
use crate::particle::*;

pub struct ParticleSystem {
    particles: Vec<Particle>,
    pub origin: Point2,
}

impl ParticleSystem {
    pub fn new(position: Point2) -> Self {
        let origin = position;
        let particles = Vec::new();
        ParticleSystem { origin, particles }
    }

    pub fn add_particle(&mut self) {
        self.particles.push(Particle::new(self.origin));
    }

    pub fn update(&mut self) {
        for i in (0..self.particles.len()).rev() {
            self.particles[i].update();
            if self.particles[i].is_dead() {
                self.particles.remove(i);
            }
        }
    }

    pub fn draw(&self, draw: &Draw) {
        for p in self.particles.iter() {
            p.display(&draw);
        }
    }
}
