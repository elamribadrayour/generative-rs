mod animal;
mod eye;
mod food;
mod fov;
mod simulation;
mod world;

extern crate nannou;
use nannou::prelude::*;
use simulation::Simulation;

fn main() {
    nannou::app(model).update(update).simple_window(view).run();
}

struct Model {
    iteration: u32,
    simulation: Simulation,
}

fn model(_app: &App) -> Model {
    let simulation = Simulation::random();
    Model {
        iteration: 0,
        simulation,
    }
}

fn update(_app: &App, _model: &mut Model, _update: Update) {
    _model.simulation.iter_animal().for_each(|animal| {
        animal.set_position();
    });

    _model.iteration += 1;
    _model.simulation.process_collisions();
    _model.simulation.process_brains();
    _model.simulation.evolve();
}

fn view(app: &App, _model: &Model, frame: Frame) {
    frame.clear(BLANCHEDALMOND);
    let draw = app.draw();
    let win = app.window_rect();
    let world = _model.simulation.get_world();

    world.get_animals().iter().for_each(|animal| {
        let color = animal.get_color();
        let object = animal.get_barycenter(win);
        draw.ellipse()
            .x_y(object.x, object.y)
            .radius(10.0)
            .color(color);

        let points = animal.get_fov(win);
        draw.polyline()
            .weight(2.0) // Line thickness
            .points(points)
            .color(PINK);
    });

    world.get_food().iter().for_each(|food| {
        let pos = food.get_barycenter(win);
        draw.ellipse()
            .color(CADETBLUE)
            .radius(5.0)
            .x_y(pos.x, pos.y);
    });

    draw.to_frame(app, &frame).unwrap();
}
