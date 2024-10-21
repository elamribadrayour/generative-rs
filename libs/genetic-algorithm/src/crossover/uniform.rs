use super::method::Method;
use crate::chromosome::Chromosome;
use rand::RngCore;

pub struct Uniform;

impl Method for Uniform {
    fn crossover(&self, rng: &mut dyn RngCore, chromosomes: Vec<&Chromosome>) -> Chromosome {
        assert!(
            chromosomes.len() >= 2,
            "Need at least 2 parents for crossover"
        );
        let size = chromosomes[0].len();
        assert!(
            chromosomes.iter().all(|x| x.len() == size),
            "All parents should have the same nb of chromoses"
        );

        let chromosome: Vec<f32> = (0..size)
            .map(|i| {
                let index = rng.next_u32() as usize % chromosomes.len();
                chromosomes[index].get(i)
            })
            .collect();

        Chromosome::new(chromosome)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::chromosome::Chromosome;
    use rand::SeedableRng;
    use rand_chacha::ChaCha8Rng;

    #[test]
    fn test_crossover() {
        let mut rng = ChaCha8Rng::from_seed(Default::default());
        let parent1 = Chromosome::new(vec![1.0, 2.0, 3.0]);
        let parent2 = Chromosome::new(vec![4.0, 5.0, 6.0]);
        let child = Uniform.crossover(&mut rng, vec![&parent1, &parent2]);
        assert_eq!(child.get(0), 1.0);
        assert_eq!(child.get(1), 5.0);
        assert_eq!(child.get(2), 6.0);
    }
}
