#[cfg(test)]
use crate::chromosome::Chromosome;

#[cfg(test)]
use crate::individual::Individual;

#[cfg(test)]
#[derive(Debug, Clone)]
pub struct TestIndividual {
    fitness: f32,
    chromosome: Chromosome,
}

#[cfg(test)]
impl TestIndividual {
    pub fn new(fitness: f32, chromosome: Chromosome) -> Self {
        TestIndividual {
            fitness: fitness,
            chromosome,
        }
    }
}

#[cfg(test)]
impl Individual for TestIndividual {
    fn fitness(&self) -> f32 {
        self.fitness
    }

    fn chromosome(&self) -> &Chromosome {
        &self.chromosome
    }

    fn create(chromosome: Chromosome) -> Self {
        TestIndividual::new(0.0, chromosome)
    }
}
