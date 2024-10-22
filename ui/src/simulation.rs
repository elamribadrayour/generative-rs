use crate::{animal::Animal, world::World};
use rand::thread_rng;

pub struct Simulation {
    world: World,
}

impl Simulation {
    pub fn random() -> Self {
        let mut rng = thread_rng();
        let world = World::random(&mut rng);
        Self { world }
    }

    pub fn get_world(&self) -> &World {
        &self.world
    }

    pub fn iter_animal(&mut self) -> impl Iterator<Item = &mut Animal> {
        self.world.iter_animal_mut()
    }

    pub fn process_collisions(&mut self) {
        self.world.process_collisions();
    }

    pub fn process_brains(&mut self) {
        self.world.process_brains();
    }

    pub fn evolve(&mut self) {}
}
