#![allow(unused)]

/// 对两个已知点之间进行线性插值，计算给定 x 值对应的 y 值。
///
/// # 参数
/// * `x`: 需要插值的 x 值。
/// * `point0`: 第一个已知点，格式为 (x0, y0)。
/// * `point1`: 第二个已知点，格式为 (x1, y1)。
///
/// # 返回值
/// 返回在 x 处通过线性插值得到的 y 值。
pub fn linear_interpolation(x: f64, point0: (f64, f64), point1: (f64, f64)) -> f64 {
    point0.1 + (x - point0.0) * (point1.1 - point0.1) / (point1.0 - point0.0)
}

/// 使用拉格朗日多项式插值法，在给定的一组离散点基础上计算指定 x 值对应的插值结果。
///
/// # 参数
/// * `x`: 需要求解插值的 x 值。
/// * `defined_points`: 已知的数据点集合，每个点为 (x, y) 形式的元组。
///
/// # 返回值
/// 返回使用拉格朗日插值法计算出的 y 值。
pub fn lagrange_polynomial_interpolation(x: f64, defined_points: &Vec<(f64, f64)>) -> f64 {
    // 将输入点分离为 x 和 y 值向量
    let mut defined_x_values: Vec<f64> = Vec::new();
    let mut defined_y_values: Vec<f64> = Vec::new();
    for (x, y) in defined_points {
        defined_x_values.push(*x);
        defined_y_values.push(*y);
    }

    // 计算拉格朗日插值的总和
    let mut sum = 0.0;
    (0..defined_y_values.len()).for_each(|y_index| {
        let mut numerator = 1.0;
        let mut denominator = 1.0;
        for x_index in 0..defined_x_values.len() {
            if y_index == x_index {
                continue;
            }
            denominator *= defined_x_values[y_index] - defined_x_values[x_index];
            numerator *= x - defined_x_values[x_index];
        }
        sum += numerator / denominator * defined_y_values[y_index];
    });
    sum
}

#[cfg(test)]
mod tests {

    use std::assert_eq;

    use super::*;
    #[test]
    fn test_linear_interpolation() {
        let point1 = (0.0, 0.0);
        let point2 = (1.0, 1.0);
        let point3 = (2.0, 2.0);

        let x1 = 0.5;
        let x2 = 1.5;

        let y1 = linear_interpolation(x1, point1, point2);
        let y2 = linear_interpolation(x2, point2, point3);

        assert_eq!(y1, x1);
        assert_eq!(y2, x2);
        assert_eq!(
            linear_interpolation(x1, point1, point2),
            linear_interpolation(x1, point2, point1)
        );
    }

    #[test]
    fn test_lagrange_polynomial_interpolation() {
        let defined_points = vec![(0.0, 0.0), (1.0, 1.0), (2.0, 4.0), (3.0, 9.0)];

        assert_eq!(lagrange_polynomial_interpolation(1.0, &defined_points), 1.0);
        assert_eq!(lagrange_polynomial_interpolation(2.0, &defined_points), 4.0);
        assert_eq!(lagrange_polynomial_interpolation(3.0, &defined_points), 9.0);

        assert_eq!(
            lagrange_polynomial_interpolation(0.5, &defined_points),
            0.25
        );
        assert_eq!(
            lagrange_polynomial_interpolation(2.5, &defined_points),
            6.25
        );
    }
}
