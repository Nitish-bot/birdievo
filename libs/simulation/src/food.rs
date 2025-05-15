use crate::*;

#[derive(Debug)]
pub struct Food {
    position: Vec2,
}

impl Food {
    pub fn new(position: Vec2) -> Self {
        Self {
            position
        }
    }

    pub fn random(rng: &mut dyn RngCore) -> Self {
        Self::new(gen_vec2_range(0., 0.9, rng))
    }

    pub fn position(&self) -> Vec2 {
        self.position
    }

    pub fn set_position(&mut self, position: Vec2) {
        self.position = position;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
}
