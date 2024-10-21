pub struct Topology {
    pub neurons: usize,
}

impl Topology {
    pub fn new(neurons: usize) -> Self {
        Self { neurons }
    }
}
