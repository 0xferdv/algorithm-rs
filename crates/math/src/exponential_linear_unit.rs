use std::f64::consts::E;

/// Exponential Linear Unit (ELU) 激活函数实现
///
/// 该函数对输入向量中的每个元素应用 ELU 激活函数变换
/// 对于正数保持不变，对于负数应用指数变换
///
/// # 参数
/// * `vector` - 输入的 f64 类型向量，包含待处理的数值
/// * `alpha` - ELU 函数的缩放参数，控制负数部分的形状
///
/// # 返回值
/// 返回经过 ELU 变换后的新向量，正数保持不变，负数按公式 alpha * (e^x - 1) 变换
#[allow(unused)]
pub fn exponential_linear_unit(vector: &Vec<f64>, alpha: f64) -> Vec<f64> {
    let mut _vector = vector.to_owned();
    // 遍历向量中的每个元素，对负数应用 ELU 变换
    for val in &mut _vector {
        if val < &mut 0. {
            *val *= alpha * (E.powf(*val) - 1.);
        }
    }
    _vector
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_exponential_linear_unit() {
        let test_vector = vec![-10., 2., -3., 4., -5., 10., 0.05];
        let alpha = 0.01;
        assert_eq!(
            exponential_linear_unit(&test_vector, alpha),
            vec![
                0.09999546000702375,
                2.0,
                0.028506387948964082,
                4.0,
                0.049663102650045726,
                10.0,
                0.05
            ]
        );
    }
}
