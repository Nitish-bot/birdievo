use crate::*;

pub struct AnimalAgent {
    chromosome: ga::Chromosome,
    fitness: f32,
}

impl ga::Individual for AnimalAgent {
    fn create(chromosome: ga::Chromosome) -> Self {
        Self {
            fitness: 0.,
            chromosome,
        }
    }

    fn chromosome(&self) -> &ga::Chromosome {
        &self.chromosome
    }

    fn fitness(&self) -> f32 {
        self.fitness
    }
}

impl AnimalAgent {
    pub fn from_animal(animal: &Animal) -> Self {
        Self {
            fitness: animal.satiation() as f32,
            chromosome: animal.as_chromosome(),
        }
    }

    pub fn into_animal(self, rng: &mut dyn RngCore) -> Animal {
        Animal::from_chromosome(self.chromosome, rng)
    }
}
