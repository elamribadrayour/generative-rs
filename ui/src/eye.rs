use crate::fov::FieldOfView;

#[derive(Debug, Clone)]
pub struct Eye {
    fov: FieldOfView,
    cells: usize,
}

impl Eye {
    pub fn new(range: f32, angle: f32, cells: usize) -> Self {
        assert!(range > 0.0);
        assert!(angle > 0.0);
        assert!(cells > 0);
        Self {
            cells,
            fov: FieldOfView::new(range, angle),
        }
    }

    pub fn get_cells(&self) -> usize {
        self.cells
    }

    pub fn get_range(&self) -> f32 {
        self.fov.get_range()
    }

    pub fn get_angle(&self) -> f32 {
        self.fov.get_angle()
    }
}
