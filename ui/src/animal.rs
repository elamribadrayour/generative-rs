use std::f32::consts::PI;

use nannou::glam::Vec2;
use nannou::math::map_range;
use rand::{Rng, RngCore};

use crate::eye::Eye;
use neural_network::{Network, Topology};

#[derive(Debug, Clone)]
pub struct Animal {
    eye: Eye,
    speed: f32,
    rotation: f32,
    brain: Network,
    position: Vec2,
    color: nannou::color::Srgb<u8>,
}

impl Animal {
    pub fn random(rng: &mut dyn RngCore) -> Self {
        let speed = 0.1;
        let rotation = rng.gen_range(-PI..PI);
        let color = nannou::color::CORNFLOWERBLUE;
        let eye = Eye::new(50.0, PI / 2.0, 10);
        let position = Vec2::new(rng.gen_range(-1.0..1.0), rng.gen_range(-1.0..1.0));
        let brain = Network::random(
            rng,
            &[
                Topology::new(eye.get_cells()),
                Topology::new(2 * eye.get_cells()),
                Topology::new(2),
            ],
        );
        Self {
            speed,
            rotation,
            color,
            eye,
            position,
            brain,
        }
    }

    pub fn get_barycenter(&self, win: nannou::prelude::Rect) -> Vec2 {
        let x = map_range(self.position.x, -1.0, 1.0, win.left(), win.right());
        let y = map_range(self.position.y, -1.0, 1.0, win.bottom(), win.top());
        Vec2::new(x, y)
    }

    pub fn get_position(&self) -> Vec2 {
        self.position
    }

    pub fn get_rotation(&self) -> f32 {
        self.rotation
    }

    pub fn get_speed(&self) -> f32 {
        self.speed
    }

    pub fn get_color(&self) -> nannou::color::Srgb<u8> {
        self.color
    }

    pub fn get_eye(&self) -> &Eye {
        &self.eye
    }

    pub fn get_fov(&self, win: nannou::prelude::Rect) -> Vec<Vec2> {
        let eye = self.get_eye();
        let object = self.get_barycenter(win);

        let eye_range = eye.get_range();
        let eye_angle = eye.get_angle();
        let rotation = self.get_rotation();

        let end_angle = rotation + eye_angle / 2.0;
        let start_angle = rotation - eye_angle / 2.0;

        let mut points = Vec::new();
        for i in 0..=100 {
            let t = i as f32 / 100.0;
            let angle = start_angle + t * (end_angle - start_angle);
            let x = object.x + eye_range * angle.cos();
            let y = object.y + eye_range * angle.sin();
            points.push(Vec2::new(x, y));
        }

        points
    }

    pub fn get_food_views(&self, foods: &[Vec2]) -> Vec<f32> {
        let mut cells = vec![0.0; self.eye.get_cells()];
        for food in foods {
            let distance = food.distance(self.get_position());
            if distance > self.eye.get_range() {
                continue;
            }

            let angle_range = self.eye.get_angle();
            let max_angle = self.get_rotation() + angle_range / 2.0;
            let min_angle = self.get_rotation() - angle_range / 2.0;

            let mut food_angle = (food.x - self.position.x).atan2(food.y - self.position.y);
            food_angle = (food_angle + PI * 2.0) % (PI * 2.0);
            if food_angle > max_angle || food_angle < min_angle {
                continue;
            }

            let cell_idx = ((food_angle - min_angle) / angle_range * self.eye.get_cells() as f32)
                .floor() as usize;
            cells[cell_idx] += 1.0;
        }

        cells
    }

    pub fn set_position(&mut self) {
        let dx = self.get_speed() * self.get_rotation().cos() * 0.01;
        let dy = self.get_speed() * self.get_rotation().sin() * 0.01;

        self.position.x += dx;
        self.position.y += dy;

        if self.position.x > 1.0 {
            self.position.x = -1.0;
        }
        if self.position.y > 1.0 {
            self.position.y = -1.0;
        }
    }

    pub fn set_rotation(&mut self, rotation: f32) {
        self.rotation = rotation;
    }

    pub fn set_speed(&mut self, speed: f32) {
        self.speed = speed.clamp(0.1, 1.0);
    }

    pub fn process_brain(&mut self, foods: &[Vec2]) {
        let food_views = self.get_food_views(foods);
        let brain_output = self.brain.propagate(food_views).unwrap();
        self.set_rotation(brain_output[0]);
        self.set_speed(brain_output[1]);
    }

    pub fn process_collisions(&mut self, foods: Vec<Vec2>) -> Vec<Vec2> {
        let mut to_remove = Vec::new();
        for food in foods.iter() {
            let distance = self.get_position().distance(*food);
            if approx::abs_diff_eq!(distance, 0.0, epsilon = 0.005) {
                to_remove.push(*food);
            }
        }
        to_remove
    }
}
