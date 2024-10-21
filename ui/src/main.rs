mod animal;
mod food;
mod simulation;
mod world;

extern crate nannou;
use nannou::prelude::*;
use simulation::Simulation;
// use genetic_algorithm::{selection, crossover, mutation, Algorithm};

fn main() {
    nannou::app(model).update(update).simple_window(view).run();
}

struct Model {
    simulation: Simulation,
    // algorithm: Algorithm<selection::RouletteWheel>,
}

fn model(_app: &App) -> Model {
    let simulation = Simulation::random();
    // let algorithm = Algorithm::new(
    //     selection::RouletteWheel,
    //     crossover::Uniform,
    //     mutation::Gaussian::new(0.0, 0.1),
    // );
    Model {
        // algorithm,
        simulation,
    }
}

fn update(_app: &App, _model: &mut Model, _update: Update) {
    _model.simulation.iter_animal().for_each(|animal| {
        animal.set_position();
    });

    _model.simulation.process_collisions();
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
