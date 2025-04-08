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

    pub fn world(&self) -> &World {
        &self.world
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

    pub fn animals(&self) -> &[Animal] {
        &self.animals
    }

    pub fn foods(&self) -> &[Food] {
        &self.foods
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
            position: gen_vec2(rng),
            angle: rng.r#gen(),
            speed: 0.002,
        }
    }

    pub fn position(&self) -> Vec2 {
        self.position
    }

    pub fn angle(&self) -> f32 {
        self.angle
    }
}

#[derive(Debug)]
pub struct Food {
    position: Vec2,
}

impl Food {
    fn random(rng: &mut dyn RngCore) -> Self { 
        Self {
            position: gen_vec2(rng),
        }
    }

    pub fn position(&self) -> Vec2 {
        self.position
    }
}

fn gen_vec2(rng: &mut dyn RngCore) -> Vec2 {
    Vec2::new(rng.r#gen(), rng.r#gen())
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
