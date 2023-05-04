use nannou::prelude::*;

mod circles;
mod object_system;

use crate::object_system::*;

fn main() {
    nannou::app(model).update(update).run();
}

struct Model {
    object_system: ObjectSystem,
}

fn model(app: &App) -> Model {
    app.new_window().size(1536, 960).view(view).build().unwrap();
    let (_w, _h) = app.window_rect().w_h();
    let object_system = ObjectSystem::new(pt2(0.0, 0.0));
    Model { object_system }
}

fn update(app: &App, model: &mut Model, _update: Update) {
    model.object_system.origin = pt2(app.mouse.x, app.mouse.y);
    model.object_system.add_object();
    model.object_system.update();
}

fn view(app: &App, model: &Model, frame: Frame) {
    // Begin drawing
    let draw = app.draw();
    draw.background().color(WHITE);

    model.object_system.draw(&draw);

    // Write the result of our drawing to the window's frame.
    draw.to_frame(app, &frame).unwrap();
}
