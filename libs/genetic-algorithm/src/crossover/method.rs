use crate::chromosome::Chromosome;
use rand::RngCore;

pub trait Method {
    #[allow(unused)]
    fn crossover(&self, rng: &mut dyn RngCore, chromosomes: Vec<&Chromosome>) -> Chromosome;
}
