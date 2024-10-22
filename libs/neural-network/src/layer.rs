use crate::neuron::Neuron;
use anyhow::Error;
use rand::RngCore;

#[derive(Debug, Clone)]
pub struct Layer {
    neurons: Vec<Neuron>,
}

impl Layer {
    pub fn random(rng: &mut dyn RngCore, inputs_size: usize, output_size: usize) -> Self {
        let neurons = (0..output_size)
            .map(|_| Neuron::random(rng, inputs_size))
            .collect();

        Self { neurons }
    }

    pub fn propagate(&self, inputs: Vec<f32>) -> Result<Vec<f32>, Error> {
        let outputs = self
            .neurons
            .iter()
            .map(|x| x.propagate(&inputs).map_err(Error::msg))
            .collect::<Result<Vec<_>, Error>>()?;
        Ok(outputs)
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
        let layer = Layer::random(&mut rng, 3, 1);
        let bias = -0.6255188;
        let inputs = vec![1.0, 2.0, 3.0];
        let weights = vec![0.67383957, 0.8181262, 0.26284897];

        let output = layer.propagate(inputs).unwrap();
        approx::assert_relative_eq!(
            output[0],
            (1.0 * weights[0]) + (2.0 * weights[1]) + (3.0 * weights[2]) + bias
        );
    }
}
