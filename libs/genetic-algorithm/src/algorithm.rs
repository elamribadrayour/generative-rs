use crate::chromosome::Chromosome;
use crate::crossover;
use crate::individual::Individual;
use crate::mutation;
use crate::selection;
pub struct Algorithm<S> {
    selection: S,
    crossover: Box<dyn crossover::Method>,
    mutation: Box<dyn mutation::Method>,
}

impl<S> Algorithm<S>
where
    S: selection::Method,
{
    pub fn new(
        selection: S,
        crossover: impl crossover::Method + 'static,
        mutation: impl mutation::Method + 'static,
    ) -> Self {
        Self {
            selection,
            crossover: Box::new(crossover),
            mutation: Box::new(mutation),
        }
    }

    pub fn evolve<I>(&mut self, rng: &mut dyn rand::RngCore, population: &[I]) -> Vec<I>
    where
        I: Individual,
    {
        assert!(!population.is_empty());
        let new_population: Vec<I> = (0..population.len())
            .map(|_| {
                let parents = [
                    self.selection.select(rng, population),
                    self.selection.select(rng, population),
                ];
                let chromosomes: Vec<&Chromosome> =
                    parents.iter().map(|i| i.chromosome()).collect();
                let mut child = self.crossover.crossover(rng, chromosomes.clone());
                self.mutation.mutate(rng, &mut child);
                I::create(child)
            })
            .collect();
        new_population
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::crossover::Uniform;
    use crate::mutation::Gaussian;
    use crate::selection::RouletteWheel;
    use crate::test_individual::TestIndividual;
    use rand::SeedableRng;
    use rand_chacha::ChaCha8Rng;

    #[test]
    fn test_evolve() {
        let mut rng = ChaCha8Rng::from_seed(Default::default());
        let mut algorithm = Algorithm::new(RouletteWheel, Uniform, Gaussian::new(0.0, 1.0));
        let population = vec![
            TestIndividual::new(2.0, Chromosome::new(vec![1.0, 2.0, 3.0])),
            TestIndividual::new(1.0, Chromosome::new(vec![4.0, 5.0, 6.0])),
            TestIndividual::new(4.0, Chromosome::new(vec![7.0, 8.0, 9.0])),
            TestIndividual::new(3.0, Chromosome::new(vec![10.0, 11.0, 12.0])),
        ];
        println!("{:?}", population);
        let actual = algorithm.evolve(&mut rng, &population);
        let expected = vec![
            TestIndividual::new(0.0, Chromosome::new(vec![10.068467, 10.933714, 2.7788165])),
            TestIndividual::new(0.0, Chromosome::new(vec![0.65548456, 2.3935525, 3.3651116])),
            TestIndividual::new(0.0, Chromosome::new(vec![5.9696646, 9.69647, 10.542083])),
            TestIndividual::new(0.0, Chromosome::new(vec![10.018915, 1.1410468, 4.5561466])),
        ];

        assert!(actual
            .iter()
            .zip(&expected)
            .all(|(a, e)| approx::relative_eq!(a.fitness(), e.fitness())));
        assert!(actual
            .iter()
            .zip(&expected)
            .all(|(a, e)| a.chromosome().len() == e.chromosome().len()));
        assert!(actual.iter().zip(&expected).all(|(a, e)| a
            .chromosome()
            .iter()
            .zip(e.chromosome().iter())
            .all(|(ai, ei)| approx::relative_eq!(ai, ei))));
    }
}
