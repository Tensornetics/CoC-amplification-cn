use ndarray::{Array, Array2};
use ndarray_linalg::{SolveH, Inverse};

pub struct AmplificationMatrix {
    conv_network: Array2<f32>,
}

impl AmplificationMatrix {
    pub fn new(conv_network: Array2<f32>) -> Self {
        AmplificationMatrix { conv_network }
    }

    pub fn apply(&self, data: Array2<f32>) -> Array2<f32> {
        self.conv_network.dot(&data)
    }

    pub fn invert(&self) -> Option<AmplificationMatrix> {
        let conv_network_inv = match self.conv_network.try_inverse() {
            Some(inv) => inv,
            None => return None,
        };
        Some(AmplificationMatrix::new(conv_network_inv))
    }
}
