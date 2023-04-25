// The Nature of Code
// Daniel Shiffman
// http://natureofcode.com
//
// example 4-03: Exercise Moving Particle System
use nannou::prelude::*;

mod particles;

use crate::particles::particle_system::*;

fn main() {
    nannou::app(model).update(update).run();
}

struct Model {
    ps: ParticleSystem,
}

fn model(app: &App) -> Model {
    app.new_window().size(1028, 640).view(view).build().unwrap();
    let (_w, h) = app.window_rect().w_h();
    let ps = ParticleSystem::new(pt2(0.0, (h as f32 / 2.0) - 50.0));
    Model { ps }
}

fn update(app: &App, m: &mut Model, _update: Update) {
    m.ps.origin = pt2(app.mouse.x, app.mouse.y);
    m.ps.add_particle();
    m.ps.update();
}

fn view(app: &App, m: &Model, frame: Frame) {
    // Begin drawing
    let draw = app.draw();
    draw.background().color(WHITE);

    m.ps.draw(&draw);

    // Write the result of our drawing to the window's frame.
    draw.to_frame(app, &frame).unwrap();
}