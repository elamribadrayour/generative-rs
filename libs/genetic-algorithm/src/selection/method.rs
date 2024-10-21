use crate::individual::Individual;
use rand::RngCore;

pub trait Method {
    #[allow(unused)]
    fn select<'a, I>(&self, rng: &mut dyn RngCore, population: &'a [I]) -> &'a I
    where
        I: Individual;
}
