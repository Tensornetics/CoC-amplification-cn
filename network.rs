use ndarray::{Array, Array2};
use rand::Rng;

pub struct Network {
    pub layers: Vec<Array2<f32>>,
    pub weights: Vec<Array2<f32>>,
    pub biases: Vec<Array2<f32>>,
}

impl Network {
    pub fn new(sizes: &[usize]) -> Network {
        let mut rng = rand::thread_rng();
        let num_layers = sizes.len();
        let mut layers = Vec::with_capacity(num_layers);
        let mut weights = Vec::with_capacity(num_layers - 1);
        let mut biases = Vec::with_capacity(num_layers - 1);

        for i in 0..num_layers {
            layers.push(Array::zeros((sizes[i], 1)));
        }

        for i in 0..num_layers - 1 {
            let rows = sizes[i];
            let cols = sizes[i + 1];
            let mut w = Array::zeros((rows, cols));
            let mut b = Array::zeros((cols, 1));
            for j in 0..rows {
                for k in 0..cols {
                    w[[j, k]] = rng.gen_range(-1.0, 1.0);
                }
            }
            for j in 0..cols {
                b[[j, 0]] = rng.gen_range(-1.0, 1.0);
            }
            weights.push(w);
            biases.push(b);
        }

        Network {
            layers,
            weights,
            biases,
        }
    }

    pub fn feedforward(&self, input: &Array2<f32>) -> Array2<f32> {
        let mut activations = input.clone();
        for i in 0..self.weights.len() {
            let w = &self.weights[i];
            let b = &self.biases[i];
            let z = w.dot(&activations) + b;
            activations = sigmoid(&z);
        }
        activations
    }
}

fn sigmoid(z: &Array2<f32>) -> Array2<f32> {
    let e = z.mapv(|x| x.exp());
    &e / &(e + 1.0)
}
