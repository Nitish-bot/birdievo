use crate::*;

#[derive(Debug)]
pub struct Animal {
    /// Contains co-ordinate of the animal within bounds 0..=1
    position: Vec2,
    /// Contains direction in terms of f32::consts::PI (180deg)
    rotation: f32,
    /// Contains speed in unit/s
    speed: f32,
    /// Contains the eye (cells)
    eye: Eye,
    /// Contains the neural network
    brain: Brain,
    /// Contains the # of foods eaten
    satiation: usize,
}

impl Animal {
    pub fn new(eye: Eye, brain: Brain, rng: &mut dyn RngCore) -> Self {
        Self {
            position: gen_vec2(rng),
            rotation: rng.gen_range(-PI..=PI),
            speed: 0.00005,
            eye,
            brain,
            satiation: 0,
        }
    }

    pub fn random(rng: &mut dyn RngCore) -> Self {
        let eye = Eye::default();

        let brain = Brain::random(rng, &eye);

        Self::new(eye, brain, rng)
    }

    pub fn as_chromosome(&self) -> ga::Chromosome {
        self.brain.as_chromosome()
    }

    pub fn from_chromosome(chromosome: ga::Chromosome, rng: &mut dyn RngCore) -> Self {
        let eye = Eye::default();
        let brain = Brain::from_chromosome(chromosome, &eye);

        Self::new(eye, brain, rng)
    }

    pub fn position(&self) -> Vec2 {
        self.position
    }

    pub fn rotation(&self) -> f32 {
        self.rotation
    }

    pub fn speed(&self) -> f32 {
        self.speed
    }

    pub fn satiation(&self) -> usize {
        self.satiation
    }

    pub fn set_position(&mut self, pos: Vec2) {
        self.position = pos;
    }

    pub fn process_motion(&mut self) {
        let rotation = self.rotation();
        let pos = self.position();

        let angle_vector = Vec2::new(rotation.cos(), rotation.sin());
        let displacement = angle_vector * self.speed();

        let new_pos = (pos + displacement).clamp(Vec2::splat(0.05), Vec2::splat(0.95));

        // We clamp the value of x and y co-ordinates
        // since the renderer uses a unit space
        self.set_position(new_pos)
    }

    pub fn process_collisions(&mut self, food: &mut Food, rng: &mut dyn RngCore) {
        let dist = self.position().distance(food.position());

        if dist <= 0.01 {
            self.satiation += 1;
            food.set_position(gen_vec2(rng));
        }
    }

    pub fn process_brains(&mut self, foods: &Vec<Food>) {
        let vision = self
            .eye
            .process_vision(self.position(), self.rotation(), foods);

        let response = self.brain.propogate(vision);

        let speed = response[0].clamp(-MAX_ACCEL, MAX_ACCEL);
        let rotation = response[1].clamp(-MAX_ROTATION, MAX_ROTATION);

        // We don't take the values absolute but add them
        // to existing values since our neural network doesn't
        // know absolute values of itself or food
        self.speed = (self.speed() + speed).clamp(MIN_SPEED, MAX_SPEED);
        self.rotation = self.rotation() + rotation;
    }
}
