use crate::chromosome::Chromosome;
use rand::RngCore;

pub trait Method {
    #[allow(unused)]
    fn mutate(&self, rng: &mut dyn RngCore, chromosome: &mut Chromosome);
}
