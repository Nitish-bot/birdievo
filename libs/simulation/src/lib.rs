// This library simulates
mod animal;
mod animal_agent;
mod brain;
mod eye;
mod food;
mod world;

use self::animal_agent::*;
pub use self::{animal::*, brain::*, eye::*, food::*, world::*};
use glam::Vec2;
use lib_genetic_algorithm as ga;
use lib_neural_network as nn;
use rand::{Rng, RngCore};
use std::f32::consts::*;

const MIN_SPEED: f32 = 0.0001;
const MAX_SPEED: f32 = 0.005;
const MAX_ACCEL: f32 = 0.0005;
const MAX_ROTATION: f32 = FRAC_PI_4;
// The max age of birds before transferring
// it's data into the ga
const GENERATION_LENGTH: usize = 2500;

pub struct Simulation {
    world: World,
    ga: ga::GeneticAlgorithm<ga::RouletteWheelSelection>,
    age: usize,
}

impl Simulation {
    pub fn random(rng: &mut dyn RngCore) -> Self {
        let world = World::random(rng);

        let ga = ga::GeneticAlgorithm::new(
            ga::RouletteWheelSelection,
            ga::UniformCrossover,
            ga::GaussianMutation::new(0.01, 0.3),
        );

        Self { world, ga, age: 0 }
    }

    pub fn world(&self) -> &World {
        &self.world
    }

    pub fn step(&mut self, rng: &mut dyn RngCore) -> Option<ga::Statistics> {
        self.world.step(rng);

        self.age += 1;

        if self.age > GENERATION_LENGTH {
            Some(self.evolve(rng))
        } else {
            None
        }
    }

    /// Fast forward to current generation's end
    pub fn train(&mut self, rng: &mut dyn RngCore) -> ga::Statistics {
        loop {
            if let Some(summary) = self.step(rng) {
                return summary;
            }
        }
    }

    pub fn evolve(&mut self, rng: &mut dyn RngCore) -> ga::Statistics {
        self.age = 0;

        let current_poplu: Vec<_> = self
            .world
            .animals
            .iter()
            .map(AnimalAgent::from_animal)
            .collect();

        let (evolved_poplu, stats) = self.ga.evolve(rng, &current_poplu);

        self.world.animals = evolved_poplu
            .into_iter()
            .map(|agent| agent.into_animal(rng))
            .collect();

        for food in &mut self.world.foods {
            food.set_position(gen_vec2(rng));
        }

        stats
    }
}

pub fn gen_vec2(rng: &mut dyn RngCore) -> Vec2 {
    gen_vec2_range(0., 1., rng)
}

pub fn gen_vec2_range(start: f32, end: f32, rng: &mut dyn RngCore) -> Vec2 {
    Vec2::new(rng.gen_range(start..=end), rng.gen_range(start..=end))
}
