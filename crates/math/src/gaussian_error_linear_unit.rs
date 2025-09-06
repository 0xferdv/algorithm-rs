#![allow(unused)]

use std::f64::consts::{E, PI};

/// 计算双曲正切函数(tanh)的值
///
/// # 参数
/// * `vector` - 输入的f64数值
///
/// # 返回值
/// 返回输入值的双曲正切函数结果
fn tanh(vector: f64) -> f64 {
    (2. / (1. + E.powf(-2. * vector.to_owned()))) - 1.
}

/// Gaussian Error Linear Unit (GELU)激活函数
/// GELU是一种常用的神经网络激活函数，公式为: x * 0.5 * (1 + tanh(√(2/π) * (x + 0.044715 * x^3)))
///
/// # 参数
/// * `vector` - 输入的f64向量，表示神经网络层的输出值
///
/// # 返回值
/// 返回经过GELU激活函数处理后的f64向量
pub fn gaussian_error_linear_unit(vector: &Vec<f64>) -> Vec<f64> {
    let  mut gelu_vec = vector.to_owned();
    // 对向量中的每个元素应用GELU激活函数
    for value in &mut gelu_vec {
        *value = *value
            * 0.5
            * (1. + tanh(f64::powf(2. / PI, 0.5) * (*value + 0.044715 * value.powf(3.))));
    }
    gelu_vec
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gaussian_error_linear_unit() {
        let test_vector = vec![-10., 2., -3., 4., -5., 10., 0.05];
        assert_eq!(
            gaussian_error_linear_unit(&test_vector),
            vec![
                -0.0,
                1.9545976940877752,
                -0.0036373920817729943,
                3.9999297540518075,
                -2.2917961972623857e-7,
                10.0,
                0.025996938238622008
            ]
        );
    }
}
