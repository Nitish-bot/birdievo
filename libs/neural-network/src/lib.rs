/// This is my implementation of a neural network in rust
/// that simulates how an agent (bird) identifies and runs
/// towards a target (bird food) to get a positive feedback (eat it).
use rand::{Rng, RngCore};
use std::iter::*;

#[macro_use]
mod utils;

#[derive(Debug)]
pub struct Network {
    layers: Vec<Layer>,
}

#[derive(Debug)]
pub struct LayerTopology {
    pub neurons: usize,
}

impl Network {
    pub fn random(rng: &mut dyn RngCore, layers: &[LayerTopology]) -> Self {
        let layers = layers
            .windows(2)
            .map(|layers| Layer::random(rng, layers[0].neurons, layers[1].neurons))
            .collect();

        Self { layers }
    }

    pub fn from_weights(
        layers: &[LayerTopology],
        weights: impl IntoIterator<Item = f32>,
    ) -> Self {
        assert!(layers.len() > 1);
        
        let mut weights = weights.into_iter();
        
        let layers = layers
            .windows(2)
            .map(|layers| {
                Layer::from_weights(
                    layers[0].neurons,
                    layers[1].neurons,
                    &mut weights,
                )
            })
            .collect();

        if weights.next().is_some() {
            panic!("More weights than necessary")
        }

        Self { layers }
    }

    pub fn propogate(&self, inputs: Vec<f32>) -> Vec<f32> {
        self.layers
            .iter()
            .fold(inputs, |inputs, layer| layer.propogate(inputs))
    }

    pub fn weights(&self) -> impl Iterator<Item = f32> + '_{
        self.layers
            .iter()
            .flat_map(|layer| layer.neurons.iter())
            .flat_map(|neuron| once(&neuron.bias).chain(&neuron.weights))
            .copied()
    }
}

#[derive(Debug)]
struct Layer {
    neurons: Vec<Neuron>,
}

impl Layer {
    fn random(rng: &mut dyn RngCore, input_size: usize, output_size: usize) -> Self {
        let neurons = (0..output_size)
            .map(|_| Neuron::random(rng, input_size))
            .collect();

        Self { neurons }
    }

    fn from_weights(
        input_size: usize,
        output_size: usize,
        weights: &mut dyn Iterator<Item = f32>,
    ) -> Self {
        let neurons = (0..output_size)
            .map(|_| Neuron::from_weights(input_size, weights))
            .collect();

        Self { neurons }
    }

    fn propogate(&self, inputs: Vec<f32>) -> Vec<f32> {
        self.neurons
            .iter()
            .map(|neuron| neuron.propogate(&inputs))
            .collect()
    }
}

#[derive(Debug)]
struct Neuron {
    bias: f32,
    weights: Vec<f32>,
}

impl Neuron {
    fn random(rng: &mut dyn RngCore, input_size: usize) -> Self {
        let bias = rng.gen_range(-1.0..=1.0);

        let weights = (0..input_size).map(|_| rng.gen_range(-1.0..=1.)).collect();

        Self { bias, weights }
    }

    fn from_weights(
        input_size: usize,
        weights: &mut dyn Iterator<Item = f32>,
    ) -> Self {
        let bias = weights.next().expect("Not enough weights");

        let weights = (0..input_size)
            .map(|_| weights.next().expect("Not enought weights"))
            .collect();

        Self { bias, weights }
    }

    fn propogate(&self, inputs: &[f32]) -> f32 {
        assert_eq!(inputs.len(), self.weights.len());

        let output = self
            .weights
            .iter()
            .zip(inputs)
            .map(|(weight, input)| input * weight)
            .sum::<f32>();

        (output + self.bias).max(0.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::SeedableRng;
    use rand_chacha::ChaCha8Rng;

    #[test]
    fn random() {
        // We should get the same neurons for the same seed
        let mut rng = ChaCha8Rng::from_seed(Default::default());
        let neuron = Neuron::random(&mut rng, 4);

        assert_eq!(
            neuron.weights.as_slice(),
            &[0.67383957, 0.8181262, 0.26284897, 0.5238807]
        );
    }

    #[test]
    fn propogate() {
        let neuron = Neuron {
            bias: 0.5,
            weights: vec![-0.3, 0.8],
        };

        assert_almost_eq!(neuron.propogate(&[-10., -10.]), 0.0);

        assert_almost_eq!(
            neuron.propogate(&[0.5, 1.0]),
            (-0.3 * 0.5) + (0.8 * 1.0) + 0.5
        );
    }

    #[test]
    fn weights() {
        let network = Network {
            layers: vec![
                Layer {
                    neurons: vec![Neuron {
                        bias: 0.1,
                        weights: vec![0.2, 0.3, 0.4],
                    }],
                },
                Layer {
                    neurons: vec![Neuron {
                        bias: 0.5,
                        weights: vec![0.6, 0.7, 0.8],
                    }],
                },
            ]
        };

        let actual: Vec<f32>  = network.weights().collect();
        let expected = vec![0.1, 0.2, 0.3, 0.4, 0.5, 0.6, 0.7, 0.8];

        for i in 0..expected.len() {
            assert_almost_eq!(actual[i], expected[i])
        }
    }

    #[test]
    fn from_weights() {
        let layers = &[
            LayerTopology { neurons: 3 },
            LayerTopology { neurons: 2 },
        ];

        let weights = vec![0.1, 0.2, 0.3, 0.4, 0.5, 0.6, 0.7, 0.8];
        let network = Network::from_weights(layers, weights.clone());
        let actual: Vec<_> = network.weights().collect();
    
        for i in 0..actual.len() {
            assert_almost_eq!(weights[i], actual[i]);
        }
    }
}
