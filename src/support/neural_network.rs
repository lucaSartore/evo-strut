use anyhow::{Result, bail};
use nalgebra::{DMatrix, DVector};
use serde::{Serialize, Serializer, ser::SerializeStruct};

use crate::{evolution::Random, support::random_distribution::RandomDistribution};

pub type NetworkValue = f32;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub enum ActivationFunction {
    Sigmoid,
    Tanh,
    Relu,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NetworkWeightInitialization {
    Xavier,
    He,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct LayerTopology {
    size: usize,
    activation_function: ActivationFunction,
}

impl LayerTopology {
    pub fn new(size: usize, activation_function: ActivationFunction) -> Result<Self> {
        if size == 0 {
            bail!("neural network layers must have at least one neuron");
        }

        Ok(Self {
            size,
            activation_function,
        })
    }

    pub fn size(&self) -> usize {
        self.size
    }

    pub fn activation_function(&self) -> ActivationFunction {
        self.activation_function
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct NetworkTopology {
    input_size: usize,
    layers: Vec<LayerTopology>,
}

impl NetworkTopology {
    pub fn new(input_size: usize, layers: Vec<LayerTopology>) -> Result<Self> {
        if input_size == 0 {
            bail!("neural network input size must be greater than zero, got {input_size}");
        }
        if layers.is_empty() {
            bail!("neural network topology must contain at least one layer");
        }

        Ok(Self { input_size, layers })
    }

    pub fn input_size(&self) -> usize {
        self.input_size
    }

    pub fn output_size(&self) -> usize {
        self.layers.last().map(LayerTopology::size).unwrap_or(0)
    }

    pub fn layers(&self) -> &[LayerTopology] {
        &self.layers
    }

    pub fn parameter_count(&self) -> usize {
        let mut previous_layer_size = self.input_size;
        self.layers
            .iter()
            .map(|layer| {
                let weights = previous_layer_size * layer.size();
                previous_layer_size = layer.size();
                weights + layer.size()
            })
            .sum()
    }
}

#[derive(Debug, Clone)]
pub struct NeuralNetworkLayer {
    pub weights: DMatrix<NetworkValue>,
    pub biases: DVector<NetworkValue>,
}

impl NeuralNetworkLayer {
    pub fn new(weights: DMatrix<NetworkValue>, biases: DVector<NetworkValue>) -> Result<Self> {
        if weights.nrows() == 0 || weights.ncols() == 0 {
            bail!(
                "neural network layer weights must have at least one row and one column, got {}x{}",
                weights.nrows(),
                weights.ncols()
            );
        }
        if biases.len() != weights.nrows() {
            bail!(
                "neural network layer bias length mismatch: expected {}, got {}",
                weights.nrows(),
                biases.len()
            );
        }

        Ok(Self { weights, biases })
    }

    pub fn weights(&self) -> &DMatrix<NetworkValue> {
        &self.weights
    }

    pub fn biases(&self) -> &DVector<NetworkValue> {
        &self.biases
    }
}

#[derive(Debug, Clone)]
pub struct NeuralNetwork {
    pub topology: NetworkTopology,
    pub layers: Vec<NeuralNetworkLayer>,
}

impl Serialize for NeuralNetwork {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("NeuralNetwork", 2)?;
        state.serialize_field("topology", &self.topology)?;
        state.serialize_field("parameters", &self.parameters())?;
        state.end()
    }
}

impl NeuralNetwork {
    pub fn new(topology: NetworkTopology, layers: Vec<NeuralNetworkLayer>) -> Result<Self> {
        validate_layers_for_topology(&topology, &layers)?;
        Ok(Self { topology, layers })
    }

    pub fn random(
        topology: NetworkTopology,
        weight_initialization: NetworkWeightInitialization,
        rand: &Random,
    ) -> Result<Self> {
        let mut layers = Vec::with_capacity(topology.layers.len());
        let mut fan_in = topology.input_size;

        for layer_topology in topology.layers.iter() {
            let fan_out = layer_topology.size();
            let limit = match weight_initialization {
                NetworkWeightInitialization::Xavier => {
                    (6.0 / (fan_in + fan_out) as NetworkValue).sqrt()
                }
                NetworkWeightInitialization::He => (6.0 / fan_in as NetworkValue).sqrt(),
            };

            let weights = DMatrix::from_fn(fan_out, fan_in, |_, _| rand.next_f32(-limit, limit));
            let biases = DVector::zeros(fan_out);
            layers.push(NeuralNetworkLayer { weights, biases });
            fan_in = fan_out;
        }

        Self::new(topology, layers)
    }

    pub fn from_parameters(topology: NetworkTopology, parameters: &[NetworkValue]) -> Result<Self> {
        if parameters.len() != topology.parameter_count() {
            bail!(
                "neural network parameter count mismatch: expected {}, got {}",
                topology.parameter_count(),
                parameters.len()
            );
        }

        let mut layers = Vec::with_capacity(topology.layers.len());
        let mut parameter_offset = 0;
        let mut fan_in = topology.input_size;

        for layer_topology in topology.layers.iter() {
            let fan_out = layer_topology.size();
            let weights_len = fan_in * fan_out;
            let weights = DMatrix::from_column_slice(
                fan_out,
                fan_in,
                &parameters[parameter_offset..parameter_offset + weights_len],
            );
            parameter_offset += weights_len;

            let biases = DVector::from_column_slice(
                &parameters[parameter_offset..parameter_offset + fan_out],
            );
            parameter_offset += fan_out;

            layers.push(NeuralNetworkLayer { weights, biases });
            fan_in = fan_out;
        }

        Self::new(topology, layers)
    }

    pub fn topology(&self) -> &NetworkTopology {
        &self.topology
    }

    pub fn layers(&self) -> &[NeuralNetworkLayer] {
        &self.layers
    }

    pub fn parameters(&self) -> Vec<NetworkValue> {
        let mut parameters = Vec::with_capacity(self.topology.parameter_count());
        for layer in self.layers.iter() {
            parameters.extend_from_slice(layer.weights.as_slice());
            parameters.extend_from_slice(layer.biases.as_slice());
        }
        parameters
    }

    pub fn set_parameters(&mut self, parameters: &[NetworkValue]) -> Result<()> {
        let replacement = Self::from_parameters(self.topology.clone(), parameters)?;
        self.layers = replacement.layers;
        Ok(())
    }

    pub fn evaluate(&self, inputs: &[NetworkValue]) -> Result<Vec<NetworkValue>> {
        if inputs.len() != self.topology.input_size {
            bail!(
                "neural network input size mismatch: expected {}, got {}",
                self.topology.input_size,
                inputs.len()
            );
        }

        let mut values = DVector::from_column_slice(inputs);
        for (layer, layer_topology) in self.layers.iter().zip(self.topology.layers.iter()) {
            values = &layer.weights * values + &layer.biases;
            values.apply(|value| {
                *value = layer_topology.activation_function().apply(*value);
            });
        }

        Ok(values.as_slice().to_vec())
    }

    pub fn crossover(
        &self,
        other: &Self,
        settings: &NetworkCrossoverSettings,
        rand: &Random,
    ) -> Result<Self> {
        if self.topology != other.topology {
            bail!("cannot crossover neural networks with different topologies");
        }

        let self_parameters = self.parameters();
        let other_parameters = other.parameters();
        let mut child_parameters = Vec::with_capacity(self_parameters.len());

        match settings.strategy {
            NetworkCrossoverStrategy::Uniform => {
                for (a, b) in self_parameters.iter().zip(other_parameters.iter()) {
                    child_parameters.push(if rand.random_choice(0.5) { *a } else { *b });
                }
            }
            NetworkCrossoverStrategy::SinglePoint => {
                let split_at = rand.next_in_range_usize(0, self_parameters.len());
                child_parameters.extend_from_slice(&self_parameters[..split_at]);
                child_parameters.extend_from_slice(&other_parameters[split_at..]);
            }
            NetworkCrossoverStrategy::Arithmetic => {
                for (a, b) in self_parameters.iter().zip(other_parameters.iter()) {
                    child_parameters.push(
                        settings.arithmetic_alpha * *a + (1.0 - settings.arithmetic_alpha) * *b,
                    );
                }
            }
        }

        Self::from_parameters(self.topology.clone(), &child_parameters)
    }

    pub fn mutate(&mut self, settings: &NetworkMutationSettings, rand: &Random) -> Result<()> {
        let mut parameters = self.parameters();
        for parameter in parameters.iter_mut() {
            if !rand.random_choice(settings.rates.mutation_probability) {
                continue;
            }

            if rand.random_choice(settings.rates.reset_probability) {
                *parameter = rand.next_distribution(&settings.reset);
            } else {
                *parameter += rand.next_distribution(&settings.perturbation);
            }
        }

        self.set_parameters(&parameters)
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct NetworkMutationRates {
    pub mutation_probability: f32,
    pub reset_probability: f32,
}

impl NetworkMutationRates {
    pub fn new(mutation_probability: f32, reset_probability: f32) -> Result<Self> {
        validate_probability("mutation_probability", mutation_probability)?;
        validate_probability("reset_probability", reset_probability)?;

        Ok(Self {
            mutation_probability,
            reset_probability,
        })
    }
}

#[derive(Debug, Clone)]
pub struct NetworkMutationSettings {
    pub rates: NetworkMutationRates,
    pub perturbation: RandomDistribution,
    pub reset: RandomDistribution,
}

impl NetworkMutationSettings {
    pub fn new(
        rates: NetworkMutationRates,
        perturbation: RandomDistribution,
        reset: RandomDistribution,
    ) -> Self {
        Self {
            rates,
            perturbation,
            reset,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NetworkCrossoverStrategy {
    Uniform,
    SinglePoint,
    Arithmetic,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct NetworkCrossoverSettings {
    pub strategy: NetworkCrossoverStrategy,
    pub arithmetic_alpha: f32,
}

impl NetworkCrossoverSettings {
    pub fn uniform() -> Self {
        Self {
            strategy: NetworkCrossoverStrategy::Uniform,
            arithmetic_alpha: 0.5,
        }
    }

    pub fn single_point() -> Self {
        Self {
            strategy: NetworkCrossoverStrategy::SinglePoint,
            arithmetic_alpha: 0.5,
        }
    }

    pub fn arithmetic(arithmetic_alpha: f32) -> Result<Self> {
        validate_probability("arithmetic_alpha", arithmetic_alpha)?;
        Ok(Self {
            strategy: NetworkCrossoverStrategy::Arithmetic,
            arithmetic_alpha,
        })
    }
}

impl ActivationFunction {
    fn apply(self, value: NetworkValue) -> NetworkValue {
        match self {
            ActivationFunction::Sigmoid => 1.0 / (1.0 + (-value).exp()),
            ActivationFunction::Tanh => value.tanh(),
            ActivationFunction::Relu => value.max(0.0),
        }
    }
}

fn validate_probability(name: &'static str, value: NetworkValue) -> Result<()> {
    if !(0.0..=1.0).contains(&value) {
        bail!("{name} must be between 0.0 and 1.0, got {value}");
    }

    Ok(())
}

fn validate_layers_for_topology(
    topology: &NetworkTopology,
    layers: &[NeuralNetworkLayer],
) -> Result<()> {
    if layers.len() != topology.layers.len() {
        bail!(
            "neural network layer count mismatch: expected {}, got {}",
            topology.layers.len(),
            layers.len()
        );
    }

    let mut expected_cols = topology.input_size;
    for (layer_index, (layer, layer_topology)) in
        layers.iter().zip(topology.layers.iter()).enumerate()
    {
        let expected_rows = layer_topology.size();
        if layer.weights.nrows() != expected_rows || layer.weights.ncols() != expected_cols {
            bail!(
                "neural network layer {layer_index} weight shape mismatch: expected {expected_rows}x{expected_cols}, got {}x{}",
                layer.weights.nrows(),
                layer.weights.ncols()
            );
        }
        if layer.biases.len() != expected_rows {
            bail!(
                "neural network layer {layer_index} bias length mismatch: expected {expected_rows}, got {}",
                layer.biases.len()
            );
        }
        expected_cols = expected_rows;
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn topology() -> NetworkTopology {
        NetworkTopology::new(
            2,
            vec![
                LayerTopology::new(3, ActivationFunction::Relu).unwrap(),
                LayerTopology::new(1, ActivationFunction::Sigmoid).unwrap(),
            ],
        )
        .unwrap()
    }

    #[test]
    fn parameter_count_includes_weights_and_biases() {
        assert_eq!(topology().parameter_count(), 13);
    }

    #[test]
    fn parameter_round_trip_preserves_order() {
        let topology = topology();
        let parameters = (0..topology.parameter_count())
            .map(|value| value as NetworkValue)
            .collect::<Vec<_>>();

        let network = NeuralNetwork::from_parameters(topology, &parameters).unwrap();

        assert_eq!(network.parameters(), parameters);
    }

    #[test]
    fn evaluate_uses_layer_activation_functions() {
        let topology = NetworkTopology::new(
            2,
            vec![
                LayerTopology::new(2, ActivationFunction::Relu).unwrap(),
                LayerTopology::new(1, ActivationFunction::Tanh).unwrap(),
            ],
        )
        .unwrap();
        let network = NeuralNetwork::from_parameters(
            topology,
            &[1.0, -1.0, 1.0, 1.0, 0.0, 0.0, 1.0, 1.0, 0.0],
        )
        .unwrap();

        let output = network.evaluate(&[1.0, 0.0]).unwrap();

        assert!((output[0] - 1.0_f32.tanh()).abs() < 0.0001);
    }
}
