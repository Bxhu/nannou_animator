use nannou::prelude::*;

// A simple particle type
struct Particle {
    position: Point2,
    velocity: Vec2,
    acceleration: Vec2,
    life_span: f32,
}

impl Particle {
    fn new(l: Point2) -> Self {
        let acceleration = vec2(0.0, 0.05);
        let velocity = vec2(random_f32() * 2.0 - 1.0, random_f32() - 1.0);
        let position = l;
        let life_span = 255.0;
        Particle {
            acceleration,
            velocity,
            position,
            life_span,
        }
    }

    // Method to update position
    fn update(&mut self) {
        self.velocity += self.acceleration;
        self.position -= self.velocity;
        self.life_span -= 2.0;
    }

    // Method to display
    fn display(&self, draw: &Draw) {
        draw.ellipse()
            .xy(self.position)
            .w_h(12.0, 12.0)
            .rgba(0.5, 0.5, 0.5, self.life_span / 255.0)
            .stroke(rgba(0.0, 0.0, 0.0, self.life_span / 255.0))
            .stroke_weight(2.0);
    }

    // Is the particle still useful?
    fn is_dead(&self) -> bool {
        if self.life_span < 0.0 {
            true
        } else {
            false
        }
    }
}

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
