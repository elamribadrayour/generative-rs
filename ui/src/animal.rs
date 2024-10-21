use nannou::math::map_range;
use nannou::prelude::Point2;
use rand::{Rng, RngCore};

#[derive(Debug, Clone)]
pub struct Animal {
    speed: f32,
    rotation: f32,
    position: Point2,
    color: nannou::color::Srgb<u8>,
}

impl Animal {
    pub fn random(rng: &mut dyn RngCore) -> Self {
        Self {
            speed: 0.1,
            rotation: rng.gen_range(-1.0..1.0),
            position: Point2::new(rng.gen_range(-1.0..1.0), rng.gen_range(-1.0..1.0)),
            color: nannou::color::CORNFLOWERBLUE,
            // color: nannou::color::Srgb::new(rng.gen_range(0..255), rng.gen_range(0..255), rng.gen_range(0..255)),
        }
    }

    pub fn get_barycenter(&self, win: nannou::prelude::Rect) -> Point2 {
        let x = map_range(self.position.x, -1.0, 1.0, win.left(), win.right());
        let y = map_range(self.position.y, -1.0, 1.0, win.bottom(), win.top());
        Point2::new(x, y)
    }

    pub fn get_position(&self) -> Point2 {
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

    pub fn set_position(&mut self) {
        let dx = self.get_speed() * self.get_rotation().sin() * 0.01;
        let dy = self.get_speed() * self.get_rotation().cos() * 0.01;

        self.position.x += dx;
        self.position.y += dy;

        if self.position.x > 1.0 {
            self.position.x = -1.0;
        }
        if self.position.y > 1.0 {
            self.position.y = -1.0;
        }
    }
}
