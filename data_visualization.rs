use std::collections::HashMap;
use std::path::Path;

use gnuplot::*;
use serde_json::Value;

pub fn visualize_data(data: Value) {
    let mut x_vals = Vec::new();
    let mut y_vals = Vec::new();

    for (i, datapoint) in data["datapoints"].as_array().unwrap().iter().enumerate() {
        x_vals.push(i as f64);
        y_vals.push(datapoint["y"].as_f64().unwrap());
    }

    let x_label = data["x_label"].as_str().unwrap();
    let y_label = data["y_label"].as_str().unwrap();
    let title = data["title"].as_str().unwrap();

    let mut fg = Figure::new();
    fg.axes2d()
        .set_title(title, &[])
        .set_x_label(x_label, &[])
        .set_y_label(y_label, &[])
        .lines(&x_vals, &y_vals, &[Color("red"), LineWidth(2.0)]);

    let file_name = format!("{}.png", data["title"]);
    let file_path = Path::new(&file_name);
    fg.save_to_png(file_path, 640, 480).unwrap();
}
