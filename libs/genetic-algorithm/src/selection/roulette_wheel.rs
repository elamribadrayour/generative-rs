use crate::individual::Individual;
use crate::selection::Method;
use rand::seq::SliceRandom;

pub struct RouletteWheel;

impl Method for RouletteWheel {
    fn select<'a, I>(&self, rng: &mut dyn rand::RngCore, population: &'a [I]) -> &'a I
    where
        I: Individual,
    {
        population
            .choose_weighted(rng, |x| x.fitness())
            .expect("population is empty")
    }
}

#[cfg(test)]
mod tests {
    use std::vec;

    use super::*;
    use crate::chromosome::Chromosome;
    use crate::test_individual::TestIndividual;
    use rand::SeedableRng;
    use rand_chacha::ChaCha8Rng;

    #[test]
    fn test_roulette_wheel_selection() {
        let mut rng = ChaCha8Rng::from_seed(Default::default());
        let population = vec![
            TestIndividual::new(2.0, Chromosome::new(vec![1.0, 2.0, 3.0])),
            TestIndividual::new(1.0, Chromosome::new(vec![4.0, 5.0, 6.0])),
            TestIndividual::new(4.0, Chromosome::new(vec![7.0, 8.0, 9.0])),
            TestIndividual::new(3.0, Chromosome::new(vec![10.0, 11.0, 12.0])),
        ];
        let mut scores = vec![0; 4];
        (0..100).for_each(|_| {
            let actual = RouletteWheel.select(&mut rng, &population);
            scores[population
                .iter()
                .position(|x| x.fitness() == actual.fitness())
                .unwrap()] += 1;
        });
        assert!(scores[2] > scores[3]);
        assert!(scores[3] > scores[0]);
        assert!(scores[0] > scores[1]);
    }
}
