use anyhow::Error;
use rand::{Rng, RngCore};

#[derive(Debug)]
pub struct Neuron {
    weights: Vec<f32>,
    bias: f32,
}

impl Neuron {
    pub fn random(rng: &mut dyn RngCore, size: usize) -> Self {
        let bias = rng.gen_range(-1.0..=1.0);
        let weights = (0..size).map(|_| rng.gen_range(-1.0..=1.0)).collect();
        Self { weights, bias }
    }

    pub fn propagate(&self, inputs: &[f32]) -> Result<f32, Error> {
        assert!(!inputs.is_empty());
        assert!(!self.weights.is_empty());
        assert_eq!(inputs.len(), self.weights.len());

        let mut output = inputs
            .iter()
            .zip(self.weights.iter())
            .map(|(x, y)| x * y)
            .sum::<f32>();
        output += self.bias;
        output = output.max(0.0);
        Ok(output)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::SeedableRng;
    use rand_chacha::ChaCha8Rng;

    #[test]
    fn test_propagate() {
        let mut rng = ChaCha8Rng::from_seed(Default::default());
        let neuron = Neuron::random(&mut rng, 3);

        let bias = -0.6255188;
        let inputs = vec![1.0, 2.0, 3.0];
        let weights = vec![0.67383957, 0.8181262, 0.26284897];

        let output = neuron.propagate(&inputs).unwrap();
        approx::assert_relative_eq!(
            output,
            (inputs[0] * weights[0]) + (inputs[1] * weights[1]) + (inputs[2] * weights[2]) + bias
        );
    }
}
