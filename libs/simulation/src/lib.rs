// This library simulates
use glam::Vec2;
use rand::{Rng, RngCore};

pub struct Simulation {
    world: World,
}

impl Simulation {
    pub fn random(rng: &mut dyn RngCore) -> Self {
        Self {
            world: World::random(rng),
        }
    }
}

#[derive(Debug)]
pub struct World{
    animals: Vec<Animal>,
    foods: Vec<Food>,
}

impl World {
    pub fn random(rng: &mut dyn RngCore) -> Self {
        let animals = (0..40)
            .map(|_| Animal::random(rng))
            .collect();

        let foods = (0..40)
            .map(|_| Food::random(rng))
            .collect();

        Self { animals, foods }
    }
}

#[derive(Debug)]
pub struct Animal {
    // Contains co-ordinate of the animal
    position: Vec2,
    // Contains direction in terms of f32::consts::PI (180deg)
    angle: f32,
    // Contains speed in unit/s
    speed: f32,
}

impl Animal {
    pub fn random(rng: &mut dyn RngCore) -> Self {
        Self {
            position: rng.r#gen(),
            angle: rng.r#gen(),
            speed: 0.002,
        }
    }
}

#[derive(Debug)]
pub struct Food {
    position: Vec2,
}

impl Food {
    fn random(rng: &mut dyn RngCore) -> Self { 
        Self {
            position: rng.r#gen::<Vec2>(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
