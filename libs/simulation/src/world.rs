use crate::*;

#[derive(Debug)]
pub struct World {
    pub animals: Vec<Animal>,
    pub foods: Vec<Food>,
}

impl World {
    pub fn random(rng: &mut dyn RngCore) -> Self {
        let animals = (0..40).map(|_| Animal::random(rng)).collect();

        let foods = (0..40).map(|_| Food::random(rng)).collect();

        Self { animals, foods }
    }

    pub fn animals(&self) -> &[Animal] {
        &self.animals
    }

    pub fn foods(&self) -> &[Food] {
        &self.foods
    }

    pub fn step(&mut self, rng: &mut dyn RngCore) {
        for animal in &mut self.animals {
            animal.process_motion();
            animal.process_brains(&self.foods);
            for food in &mut self.foods {
                animal.process_collisions(food, rng);
            }
        }
    }
}
