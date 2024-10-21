use crate::chromosome::Chromosome;
use crate::mutation::Method;
use rand::RngCore;
use rand_distr::{Distribution, Normal};

#[derive(Debug, Clone)]
pub struct Gaussian {
    mean: f32,
    std_dev: f32,
}

impl Gaussian {
    pub fn new(mean: f32, std_dev: f32) -> Self {
        Self { mean, std_dev }
    }
}

impl Method for Gaussian {
    fn mutate(&self, rng: &mut dyn RngCore, chromosome: &mut Chromosome) {
        let normal = Normal::new(self.mean, self.std_dev).unwrap();
        for gene in chromosome.iter_mut() {
            *gene += normal.sample(rng);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::SeedableRng;
    use rand_chacha::ChaCha8Rng;

    #[test]
    fn test_mutate() {
        let mut rng = ChaCha8Rng::from_seed(Default::default());
        let mut chromosome = Chromosome::new(vec![1.0, 2.0, 3.0]);
        Gaussian {
            mean: 0.0,
            std_dev: 1.0,
        }
        .mutate(&mut rng, &mut chromosome);
        approx::assert_relative_eq!(chromosome.get(0), 2.3776972);
        approx::assert_relative_eq!(chromosome.get(1), 2.4053469);
        approx::assert_relative_eq!(chromosome.get(2), 1.8037311);
    }
}
