use nannou::math::map_range;
use nannou::prelude::Point2;
use rand::{Rng, RngCore};

#[derive(Debug, Clone)]
pub struct Food {
    position: Point2,
}

impl Food {
    pub fn random(rng: &mut dyn RngCore) -> Self {
        Self {
            position: Point2::new(rng.gen_range(-1.0..1.0), rng.gen_range(-1.0..1.0)),
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
}

impl PartialEq for Food {
    fn eq(&self, other: &Self) -> bool {
        approx::abs_diff_eq!(self.position.x, other.position.x, epsilon = 1e-6)
            && approx::abs_diff_eq!(self.position.y, other.position.y, epsilon = 1e-6)
    }
}
