#[allow(unused)]
pub fn cross_entropy_loss(actual: &[f64], predicted: &[f64]) -> f64 {
    let mut loss: Vec<f64> = Vec::new();
    actual
        .iter()
        .zip(predicted.iter())
        .for_each(|(a, p)| loss.push(-a * p.ln()));
    loss.iter().sum::<f64>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cross_entropy_loss() {
        let test_vector_actual = vec![0., 1., 0., 0., 0., 0.];
        let test_vector = vec![0.1, 0.7, 0.1, 0.05, 0.05, 0.1];
        let loss = cross_entropy_loss(&test_vector_actual, &test_vector);
        assert_eq!(
            loss,
            0.35667494393873245
        );
    }
}
