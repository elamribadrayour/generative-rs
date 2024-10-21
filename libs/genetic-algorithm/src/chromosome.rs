#[derive(Debug, Clone)]
pub struct Chromosome {
    genes: Vec<f32>,
}

impl Chromosome {
    pub fn new(genes: Vec<f32>) -> Self {
        Self { genes }
    }

    pub fn len(&self) -> usize {
        self.genes.len()
    }

    pub fn is_empty(&self) -> bool {
        self.genes.is_empty()
    }

    pub fn iter(&self) -> impl Iterator<Item = &f32> {
        self.genes.iter()
    }

    pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut f32> {
        self.genes.iter_mut()
    }

    pub fn get(&self, index: usize) -> f32 {
        if index >= self.genes.len() {
            panic!("Index out of bounds");
        }
        self.genes[index]
    }
}
