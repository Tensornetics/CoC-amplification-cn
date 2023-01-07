mod amplification_matrix;
mod convolutional_layer;
mod data_security;
mod network;
mod data_interface;
mod data_visualization;

use amplification_matrix::AmplificationMatrix;
use convolutional_layer::ConvolutionalLayer;
use data_security::DataSecurity;
use network::Network;
use data_interface::DataInterface;
use data_visualization::DataVisualization;

fn main() {
    let amplification_matrix = AmplificationMatrix::new();
    let convolutional_layer = ConvolutionalLayer::new();
    let data_security = DataSecurity::new();
    let network = Network::new(amplification_matrix, convolutional_layer, data_security);
    let data_interface = DataInterface::new(network);
    let data_visualization = DataVisualization::new(data_interface);

    data_visualization.run();
}
