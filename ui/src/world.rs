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
        let mut to_remove = Vec::new();
        for animal in self.get_animals().iter() {
            for food in self.get_food().iter() {
                // Calculate distance
                let distance = animal.get_position().distance(food.get_position());
                if approx::abs_diff_eq!(distance, 0.0, epsilon = 0.01) {
                    to_remove.push(food);
                }
            }
        }

        let food_filtered: Vec<Food> = self
            .get_food()
            .iter()
            .filter(|f| !to_remove.contains(f))
            .cloned()
            .collect();
        self.food = food_filtered;
    }
}
