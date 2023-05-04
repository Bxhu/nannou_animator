use nannou::prelude::*;

mod circles;
mod object_system;

use crate::object_system::*;

fn main() {
    nannou::app(model).update(update).run();
}

struct Model {
    object_system: ObjectSystem,
    next_update_time: f32,
}

fn model(app: &App) -> Model {
    app.new_window().size(1536, 960).view(view).build().unwrap();
    let (_w, _h) = app.window_rect().w_h();
    let object_system = ObjectSystem::new(pt2(0.0, 0.0));
    let next_update_time = 1.0;
    Model {
        object_system,
        next_update_time,
    }
}

fn update(_app: &App, _model: &mut Model, _update: Update) {
    let elapsed_time = _app.duration.since_start.as_secs_f32();
    if elapsed_time > _model.next_update_time {
        _model.object_system.add_object();

        _model.next_update_time = elapsed_time + random_range(0.03, 0.07);
    }
    _model.object_system.update();
}

fn view(app: &App, model: &Model, frame: Frame) {
    // Begin drawing
    let draw = app.draw();
    draw.background().color(WHITE);

    model.object_system.draw(&draw);

    // Write the result of our drawing to the window's frame.
    draw.to_frame(app, &frame).unwrap();
}
