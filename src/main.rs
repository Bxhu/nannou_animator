use nannou::prelude::*;

mod circles;
mod helpers;
mod object_system;

use crate::object_system::*;

fn main() {
    nannou::app(model).update(update).run();
}

struct Model {
    os: ObjectSystem,
}

fn model(app: &App) -> Model {
    app.new_window().size(1536, 960).view(view).build().unwrap();
    let (_w, _h) = app.window_rect().w_h();
    let object_system = ObjectSystem::new(pt2(0.0, 0.0), _w, _h);
    Model { os: object_system }
}

fn update(_app: &App, _model: &mut Model, _update: Update) {
    let elapsed_time = _app.duration.since_start.as_secs_f32();
    if elapsed_time > _model.os.next_update_time {
        _model.os.add_object();

        _model.os.next_update_time = elapsed_time + random_range(0.03, 0.07);
    }
    _model.os.update();
}

fn view(app: &App, model: &Model, frame: Frame) {
    // Begin drawing
    let draw = app.draw();
    draw.background().color(WHITE);

    model.os.draw_all(&draw);

    // Write the result of our drawing to the window's frame.
    draw.to_frame(app, &frame).unwrap();
}
