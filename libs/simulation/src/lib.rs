// This library simulates
use std::f32::{self, consts::{ FRAC_PI_2, PI}};
use glam::{ Affine2, Vec2 };
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

    pub fn step(&mut self) {
        for animal in &mut self.world.animals {
            // We add PI / 2 radians to the angle since our 
            // 2D rendering canvas is a co-ordinate system rotated
            // by PI / 2 radians
            // TLDR: x and y increase in towards the bottom right
            let angle = animal.angle() + FRAC_PI_2;
            let pos = animal.position();
            
            let angle_vector = Vec2::new(angle.cos(), angle.sin());
            let displacement = pos + angle_vector * animal.speed;
            
            // We clamp the value of x and y co-ordinates 
            // since the renderer uses a unit space
            animal.position = displacement.clamp(Vec2::splat(0.05), Vec2::splat(0.95));
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

    pub fn animals(&self) -> &[Animal] {
        &self.animals
    }

    pub fn foods(&self) -> &[Food] {
        &self.foods
    }
}

#[derive(Debug)]
pub struct Animal {
    /// Contains co-ordinate of the animal within bounds 0..=1
    position: Vec2,
    /// Contains direction in terms of f32::consts::PI (180deg)
    angle: f32,
    /// Contains speed in unit/s
    speed: f32,
}

impl Animal {
    pub fn random(rng: &mut dyn RngCore) -> Self {
        Self {
            position: gen_vec2(rng),
            angle: rng.gen_range(0.0..=(2. * PI)),
            speed: 0.001,
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
}
