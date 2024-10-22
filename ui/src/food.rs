use nannou::glam::Vec2;
use nannou::math::map_range;
use rand::{Rng, RngCore};

#[derive(Debug, Clone)]
pub struct Food {
    position: Vec2,
}

impl Food {
    pub fn random(rng: &mut dyn RngCore) -> Self {
        Self {
            position: Vec2::new(rng.gen_range(-1.0..1.0), rng.gen_range(-1.0..1.0)),
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
}

impl PartialEq for Food {
    fn eq(&self, other: &Self) -> bool {
        approx::abs_diff_eq!(self.position.x, other.position.x, epsilon = 1e-6)
            && approx::abs_diff_eq!(self.position.y, other.position.y, epsilon = 1e-6)
    }
}
