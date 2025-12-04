#![allow(unused)]

/// 对输入向量应用 Leaky ReLU 激活函数
///
/// Leaky ReLU 是 ReLU 激活函数的变体，当输入值小于 0 时，
/// 输出为 alpha * input，而不是完全置为 0。
/// 这样可以避免神经元死亡问题。
///
/// 参数:
/// * `vector`: 输入的 f64 类型向量，表示需要处理的数据
/// * `alpha`: 负数部分的斜率系数，通常是一个很小的正数（如 0.01）
///
/// 返回值:
/// 返回应用 Leaky ReLU 函数后的新向量，保持原始向量不变
pub fn leaky_relu(vector: &Vec<f64>, alpha: f64) -> Vec<f64> {
    let mut _vector = vector.to_owned();
    // 遍历向量中的每个元素，对负数应用 Leaky ReLU 变换
    (&mut _vector).into_iter().for_each(|value| {
        if *value < 0.0 {
            *value *= alpha;
        }
    });

    _vector
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_leaky_relu() {
        let test_vector = vec![-10., 2., -3., 4., -5., 10., 0.05];
        let alpha = 0.01;
        dbg!(
            leaky_relu(&test_vector, alpha),
            vec![-0.1, 2.0, -0.03, 4.0, -0.05, 10.0, 0.05]
        );
    }
}
