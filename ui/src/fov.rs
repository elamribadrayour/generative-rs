#[derive(Debug, Clone)]
pub struct FieldOfView {
    range: f32,
    angle: f32,
}

impl FieldOfView {
    pub fn new(range: f32, angle: f32) -> Self {
        Self { range, angle }
    }

    pub fn get_range(&self) -> f32 {
        self.range
    }

    pub fn get_angle(&self) -> f32 {
        self.angle
    }
}
