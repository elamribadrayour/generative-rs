use crate::layer::Layer;
use crate::topology::Topology;
use anyhow::Error;
use rand::RngCore;

#[derive(Debug)]
pub struct Network {
    layers: Vec<Layer>,
}

impl Network {
    pub fn random(rng: &mut dyn RngCore, topology: &[Topology]) -> Self {
        assert!(!topology.is_empty());
        assert!(topology.len() > 1);

        let layers = topology
            .windows(2)
            .map(|layers| Layer::random(rng, layers[0].neurons, layers[1].neurons))
            .collect();

        Self { layers }
    }

    pub fn propagate(&self, mut inputs: Vec<f32>) -> Result<Vec<f32>, Error> {
        for layer in self.layers.iter() {
            inputs = layer.propagate(inputs).map_err(Error::msg)?;
        }
        Ok(inputs)
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
        let topology = vec![Topology::new(3), Topology::new(1)];
        let neural_network = Network::random(&mut rng, &topology);
        let bias = -0.6255188;
        let inputs = vec![1.0, 2.0, 3.0];
        let weights = vec![0.67383957, 0.8181262, 0.26284897];

        let output = neural_network.propagate(inputs).unwrap();
        approx::assert_relative_eq!(
            output[0],
            (1.0 * weights[0]) + (2.0 * weights[1]) + (3.0 * weights[2]) + bias
        );
    }
}
