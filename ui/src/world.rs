use crate::animal::Animal;
use crate::food::Food;
use rand::RngCore;

pub struct World {
    animals: Vec<Animal>,
    food: Vec<Food>,
}

impl World {
    pub fn random(rng: &mut dyn RngCore) -> Self {
        Self {
            animals: (0..20).map(|_| Animal::random(rng)).collect(),
            food: (0..100).map(|_| Food::random(rng)).collect(),
        }
    }

    pub fn get_animals(&self) -> &[Animal] {
        &self.animals
    }

    pub fn get_food(&self) -> &[Food] {
        &self.food
    }

    pub fn iter_animal_mut(&mut self) -> impl Iterator<Item = &mut Animal> {
        self.animals.iter_mut()
    }

    pub fn process_collisions(&mut self) {
        let food_positions: Vec<_> = self.food.iter().map(|f| f.get_position()).collect();
        let mut to_remove = Vec::new();
        for animal in self.iter_animal_mut() {
            let to_remove_new = animal.process_collisions(food_positions.clone());
            to_remove.extend(to_remove_new);
        }

        let food_filtered: Vec<Food> = self
            .get_food()
            .iter()
            .filter(|f| !to_remove.contains(&f.get_position()))
            .cloned()
            .collect();
        self.food = food_filtered;
    }

    pub fn process_brains(&mut self) {
        let food_positions: Vec<_> = self.food.iter().map(|f| f.get_position()).collect();
        for animal in &mut self.iter_animal_mut() {
            animal.process_brain(&food_positions);
        }
    }
}
