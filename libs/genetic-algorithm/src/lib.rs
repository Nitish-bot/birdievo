/// This is an implementation of a genetic algorithm in Rust.
/// Genetic algorithm is one that estimates and clocks current
/// solutions, and then improves them using the best of the bunch
use rand::{ Rng, RngCore };
use rand::prelude::SliceRandom;
use std::ops::Index;

pub struct GeneticAlgorithm<S> {
    selection_method: S,
    crossover_method: Box<dyn CrossoverMethod>,
    mutation_method: Box<dyn MutationMethod>,
}

impl<S> GeneticAlgorithm<S>
where
    S: SelectionMethod
{
    pub fn new(
        selection_method: S,
        crossover_method: impl CrossoverMethod + 'static,
        mutation_method: impl MutationMethod + 'static
    ) -> Self {
        Self { 
            selection_method,
            crossover_method: Box::new(crossover_method),
            mutation_method: Box::new(mutation_method),
        }
    }

    pub fn evolve<I>(&self, rng: &mut dyn RngCore, population: &[I]) -> Vec<I>
    where
        I: Individual,
    {
        assert!(!population.is_empty());

        (0..population.len())
            .map(|_| {
                let parent_a = self.selection_method.select(rng, population).chromosome();
                let parent_b = self.selection_method.select(rng, population).chromosome();
                let mut child = self.crossover_method.crossover(rng, parent_a, parent_b);
                
                self.mutation_method.mutate(rng, &mut child);

                I::create(child)
            })
            .collect()
    }
}

pub struct RouletteWheelSelection;

impl SelectionMethod for RouletteWheelSelection {
    fn select<'a, I>(&self, rng: &mut dyn RngCore, population: &'a [I]) -> &'a I
    where
        I: Individual,
    {
        let total_fitness: f32 = population
            .iter()
            .map(|individual| individual.fitness())
            .sum();

        loop {
            let indiv = population
                .choose(rng)
                .expect("population must not be empty");

            let indiv_share = indiv.fitness() / total_fitness;

            if rng.gen_bool(indiv_share as f64) {
                return indiv;
            }
        }
    }
}

pub struct Chromosome {
    genes: Vec<f32>,
}

impl Chromosome {
    pub fn len(&self) -> usize {
        self.genes.len()
    }

    pub fn iter(&self) -> impl Iterator<Item = &f32> {
        self.genes.iter()
    }

    pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut f32> {
        self.genes.iter_mut()
    }
}

impl Index<usize> for Chromosome {
    type Output = f32;

    fn index(&self, index: usize) -> &Self::Output {
        &self.genes[index]
    }
}

impl IntoIterator for Chromosome {
    type Item = f32;
    type IntoIter = std::vec::IntoIter<f32>;

    fn into_iter(self) -> Self::IntoIter {
        self.genes.into_iter()
    }
}

impl FromIterator<f32> for Chromosome {
    fn from_iter<T: IntoIterator<Item = f32>>(iter: T) -> Self {
        Self {
            genes: iter.into_iter().collect(),
        }
    }
}

pub struct UniformCrossover;

impl CrossoverMethod for UniformCrossover {
    fn crossover(
        &self,
        rng: &mut dyn RngCore,
        parent_a: &Chromosome,
        parent_b: &Chromosome,
    ) -> Chromosome {
        assert_eq!(parent_a.len(), parent_b.len());

        parent_a
            .iter()
            .zip(parent_b.iter())
            .map(|(&a, &b)| if rng.gen_bool(0.5) { a } else { b })
            .collect()
    }
}

#[derive(Clone, Debug)]
pub struct GaussianMutation {
    /// Probability of changing a gene
    probability: f32,

    /// Magnitude of the change
    /// - 0.0 = genes will not be modified
    /// - 3.0 = genes will be += or -= by at most 3.0
    coefficient: f32,
}

impl GaussianMutation {
    pub fn new(probability: f32, coefficient: f32) -> Self {
        assert!(probability >= 0. && probability <= 1.);

        Self { probability, coefficient }
    }
}

impl MutationMethod for GaussianMutation {
    fn mutate(&self, rng: &mut dyn RngCore, child: &mut Chromosome) {
        for gene in child.iter_mut() {
            let sign = if rng.gen_bool(0.5) { -1. } else { 1. };

            if rng.gen_bool(self.probability as f64) {
                *gene += sign * self.coefficient * rng.r#gen::<f32>();
            }
        }
    }
}

// Traits for individual agents and selection methods
pub trait Individual {
    fn create(chromosome: Chromosome) -> Self;
    fn fitness(&self) -> f32;
    fn chromosome(&self) -> &Chromosome;
}

pub trait SelectionMethod {
    fn select<'a, I>(&self, rn: &mut dyn RngCore, population: &'a [I]) -> &'a I
    where
        I: Individual;
}

pub trait CrossoverMethod {
    fn crossover(
        &self,
        rng: &mut dyn RngCore,
        parent_a: &Chromosome,
        parent_b: &Chromosome,
    ) -> Chromosome;
}

pub trait MutationMethod {
    fn mutate(&self, rng: &mut dyn RngCore, child: &mut Chromosome);
}


#[cfg(test)]
mod tests {
    use super::*;
    use rand::SeedableRng;
    use rand_chacha::ChaCha8Rng;
    use std::collections::BTreeMap;
    use std::iter::FromIterator;

    #[derive(Clone, Debug)]
    struct TestIndividual {
        fitness: f32,
    }

    impl TestIndividual {
        fn new(fitness: f32) -> Self {
            Self { fitness }
        }
    }

    impl Individual for TestIndividual {
        fn create(chromosome: Chromosome) -> Self {
            todo!()
        }

        fn fitness(&self) -> f32 {
            self.fitness
        }

        fn chromosome(&self) -> &Chromosome {
            panic!("not supported for test")
        }
    }

    #[test]
    fn roulette_wheel_selection() {
        let mut rng = ChaCha8Rng::from_seed(Default::default());

        let population = vec![
            TestIndividual::new(2.),
            TestIndividual::new(1.),
            TestIndividual::new(4.),
            TestIndividual::new(3.),
        ];

        let mut actual_histogram = BTreeMap::new();

        for _ in 0..1000 {
            let fitness = RouletteWheelSelection
                .select(&mut rng, &population)
                .fitness() as i32;

            *actual_histogram
                .entry(fitness)
                .or_insert(0) += 1;
        }
        
        let expected_histogram = BTreeMap::from_iter([
            (1, 97),
            (2, 208),
            (3, 302),
            (4, 393),
        ]);

        assert_eq!(actual_histogram, expected_histogram);
    }

    #[test]
    fn uniform_crossover() {
        let mut rng = ChaCha8Rng::from_seed(Default::default());
        
        let parent_a = (1..100).map(|n| n as f32).collect();
        let parent_b = (1..100).map(|n| -n as f32).collect();
        
        let child = UniformCrossover.crossover(&mut rng, &parent_a, &parent_b);

        let diff_a = child.iter().zip(parent_a.into_iter()).filter(|(c, p)| *c != p).count();
        let diff_b = child.iter().zip(parent_b.into_iter()).filter(|(c, p)| *c != p).count();

        assert_eq!(diff_a, 49);
        assert_eq!(diff_b, 50);
    }

    mod gaussian_mutation {
        use super::*;

        fn actual(probability: f32, coefficient: f32) -> Vec<f32> {
            let mut rng = ChaCha8Rng::from_seed(Default::default());
            let mut child = vec![1., 2., 3., 4., 5.].into_iter().collect();

            GaussianMutation::new(probability, coefficient).mutate(&mut rng, &mut child);

            child.into_iter().collect()
        }
        mod given_zero_chance {
            use approx::assert_relative_eq;

            fn actual(coefficient: f32) -> Vec<f32> {
                super::actual(0.0, coefficient)
            }

            mod and_zero_coefficient {
                use super::*;

                #[test]
                fn doesnt_change_original_chromosome() {
                    let actual = actual(0.);
                    let expected = vec![1., 2., 3., 4., 5.];

                    assert_relative_eq!(actual.as_slice(), expected.as_slice());
                }
            }
            mod and_non_zero_coefficient {
                use super::*;

                #[test]
                fn doesnt_change_original_chromosome() {
                    let actual = actual(0.5);
                    let expected = vec![1., 2., 3., 4., 5.];

                    assert_relative_eq!(actual.as_slice(), expected.as_slice());
                }
            }
        }
        mod given_fifty_percent_probability {
            use approx::assert_relative_eq;

            fn actual(coefficient: f32) -> Vec<f32> {
                super::actual(0.5, coefficient)
            }

            mod and_zero_coefficient {
                use super::*;

                #[test]
                fn doesnt_change_original_chromosome() {
                    let actual = actual(0.);
                    let expected = vec![1., 2., 3., 4., 5.];

                    assert_relative_eq!(actual.as_slice(), expected.as_slice());                    
                }
            }
            mod and_non_zero_coefficient {
                use super::*;
                #[test]
                fn slightly_changes_original_chromosome() {
                    let actual = actual(0.5);
                    let expected = vec![1.0, 1.7756249, 3.0, 4.1596804, 5.0];

                    assert_relative_eq!(actual.as_slice(), expected.as_slice());
                }
            }
        }
        mod given_max_probability {
            use approx::assert_relative_eq;
            
            fn actual(coefficient: f32) -> Vec<f32> {
                super::actual(1.0, coefficient)
            }
            mod and_zero_coefficient {
                use super::*;

                #[test]
                fn doesnt_change_original_chromosome() {
                    let actual = actual(0.);
                    let expected = vec![1., 2., 3., 4., 5.];

                    assert_relative_eq!(actual.as_slice(), expected.as_slice());
                }
            }
            mod and_non_zero_coefficient {
                use super::*;

                #[test]
                fn entirely_changes_original_chromosome() {
                    let actual = actual(0.5);
                    let expected = vec![1.4545316, 2.1162078, 2.7756248, 3.9505124, 4.638691];

                    assert_relative_eq!(actual.as_slice(), expected.as_slice());
                }
            }
        }
    }
}
