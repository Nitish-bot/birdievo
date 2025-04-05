/// This is an implementation of a genetic algorithm in Rust.
/// Genetic algorithm is one that estimates and clocks current
/// solutions, and then improves them using the best of the bunch
use rand::{ Rng, RngCore };
use rand::prelude::SliceRandom;

pub struct GeneticAlgorithm<S> {
    selection_method: S,
}

impl<S> GeneticAlgorithm<S>
where
    S: SelectionMethod
{
    pub fn evolve<I>(&self, population: &[I]) -> Vec<I>
    where
        I: Individual,
    {
        assert!(!population.is_empty());

        todo!();
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

pub trait Individual {
    fn fitness(&self) -> f32;
}

pub trait SelectionMethod {
    fn select<'a, I>(&self, rn: &mut dyn RngCore, population: &'a [I]) -> &'a I
    where
        I: Individual;
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
        fn fitness(&self) -> f32 {
            self.fitness
        }
    }

    #[test]
    fn rank_selection() {
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
}
