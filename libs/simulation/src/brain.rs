use crate::*;

#[derive(Debug)]
pub struct Brain {
    nn: nn::Network,
}

impl Brain {
    pub fn random(rng: &mut dyn RngCore, eye: &Eye) -> Self {
        let nn = nn::Network::random(rng, &Self::topology(eye));

        Self { nn }
    }

    pub fn as_chromosome(&self) -> ga::Chromosome {
        self.nn.weights().collect()
    }

    pub fn from_chromosome(chromosome: ga::Chromosome, eye: &Eye) -> Self {
        Self {
            nn: nn::Network::from_weights(&Self::topology(eye), chromosome),
        }
    }

    pub fn topology(eye: &Eye) -> [nn::LayerTopology; 3] {
        [
            // The Input Layer
            //
            // Because our eye returns Vec<f32>, and our neural
            // network works on Vec<f32>, we can pass-through
            // numbers from eye into the neural network directly.
            nn::LayerTopology {
                neurons: eye.cells(),
            },
            // The Hidden Layer
            //
            // The rule of thumb is to start with a single hidden
            // layer that has somewhat more neurons that the input
            // layer and see how well the network performs.
            nn::LayerTopology {
                neurons: 2 * eye.cells(),
            },
            // The Output Layer
            //
            // Since the brain will control our bird's speed and
            // rotation, this gives us two numbers = two neurons.
            nn::LayerTopology { neurons: 2 },
        ]
    }

    pub fn propogate(&self, vision: Vec<f32>) -> Vec<f32> {
        self.nn.propogate(vision)
    }
}
