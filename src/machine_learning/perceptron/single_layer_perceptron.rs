// Perceptron model implementation.
// A perceptron is a basic unit of a neural network, capable of binary classification.
// It takes inputs, applies weights, and passes the result through an activation function to produce an output.

use crate::machine_learning::perceptron::ActivationFunction;
use rand::prelude::*;

pub struct Perceptron {
    weights: Vec<f64>,                 // Weights assigned to each input feature
    learning_rate: f64,                // Learning rate for weight adjustment during training
    activation_fn: ActivationFunction, // Activation function used to compute the output
}

impl Perceptron {
    // Initialize a new perceptron with random weights, learning rate, and activation function
    pub fn new(input_size: usize, learning_rate: f64, activation_fn: ActivationFunction) -> Self {
        let mut rng = thread_rng();
        let weights: Vec<f64> = (0..input_size).map(|_| rng.gen_range(-1.0..1.0)).collect();
        Perceptron {
            weights,
            learning_rate,
            activation_fn,
        }
    }

    // Perform feedforward computation to produce an output based on input features
    fn feedforward(&self, inputs: &[f64]) -> f64 {
        let sum: f64 = inputs.iter().zip(&self.weights).map(|(&i, &w)| i * w).sum();
        self.activation_fn.activate(sum)
    }

    // Update weights based on the provided inputs and error
    fn update_weights(&mut self, inputs: &[f64], error: f64) {
        for (weight, &input) in self.weights.iter_mut().zip(inputs) {
            *weight += self.learning_rate * error * input;
        }
    }

    // Train the perceptron using the provided inputs and target outputs for a specified number of epochs
    pub fn train(&mut self, inputs: &[Vec<f64>], epochs: usize) {
        let mut rng = thread_rng();
        for _ in 0..epochs {
            let mut shuffled_inputs = inputs.to_vec();
            shuffled_inputs.shuffle(&mut rng);
            for input in &shuffled_inputs {
                let target = input.last().expect("No target value provided");
                let features = &input[..input.len() - 1];
                let prediction = self.feedforward(features);
                let error = target - prediction;
                self.update_weights(features, error);
            }
        }
    }

    // Test the perceptron with the provided inputs and expected outputs, returning accuracy
    pub fn test(&self, inputs: &[Vec<f64>], outputs: &[f64]) -> f64 {
        let mut correct_predictions = 0;
        for (input, &output) in inputs.iter().zip(outputs) {
            let features = &input[..input.len() - 1];
            let prediction = self.feedforward(features);
            if (prediction - output).abs() < 0.0001 {
                correct_predictions += 1;
            }
        }
        correct_predictions as f64 / inputs.len() as f64
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_perceptron() {
        // Create a Perceptron with the None activation function
        let mut perceptron = Perceptron::new(1, 0.1, ActivationFunction::None);

        // Generate some training data: pairs of input-output for f(x) = 2x
        let training_data: Vec<Vec<f64>> = vec![
            vec![0.0, 0.0],
            vec![1.0, 2.0],
            vec![2.0, 4.0],
            vec![3.0, 6.0],
            vec![8.0, 16.0],
        ];

        // Train the perceptron on the training data
        perceptron.train(&training_data, 100);

        // Test the perceptron
        let testing_data: Vec<Vec<f64>> = vec![
            vec![0.0, 0.0],
            vec![1.0, 2.0],
            vec![2.0, 4.0],
            vec![3.0, 6.0],
            vec![4.0, 8.0],
        ];

        let accuracy = perceptron.test(&testing_data, &[0.0, 2.0, 4.0, 6.0, 8.0]);

        assert_eq!(accuracy, 1.0);
    }
}
