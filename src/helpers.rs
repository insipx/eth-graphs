pub fn average(values: Vec<f64>) -> f64 {
    Iterator::sum::<f64>(values.iter()) / values.len() as f64
}
