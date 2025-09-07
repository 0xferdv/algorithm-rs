#![allow(unused)]

/// 计算几何级数的前n项
///
/// 该函数根据给定的首项、公比和项数，生成一个几何级数序列。
/// 几何级数的定义是：每一项与前一项的比值相等，这个比值称为公比。
///
/// # 参数说明
/// * `nth_term`: 要生成的项数（取整数部分）
/// * `start_term_a`: 几何级数的首项
/// * `common_ration_r`: 几何级数的公比
///
/// # 返回值
/// 返回包含几何级数各项的向量，向量长度等于nth_term的整数部分
pub fn geometric_series(nth_term: f64, start_term_a: f64, common_ration_r: f64) -> Vec<f64> {
    let mut series = Vec::new();
    let mut multiple = 1.0;

    // 循环生成几何级数的每一项
    // 第一项为首项乘以公比的0次方（即首项本身）
    // 后续每一项为前一项乘以公比
    (0..(nth_term as i32)).for_each(|_| {
        series.push(start_term_a * multiple);
        multiple *= common_ration_r;
    });

    series
}

#[cfg(test)]
mod tests {
    use super::*;

    fn assert_approx_eq(a: f64, b: f64) {
        let epsilon = 1e-10;
        assert!((a - b).abs() < epsilon, "Expected {a}, found {b}");
    }

    #[test]
    fn test_geometric_series() {
        let result = geometric_series(4.0, 2.0, 2.0);
        assert_eq!(result.len(), 4);
        assert_approx_eq(result[0], 2.0);
        assert_approx_eq(result[1], 4.0);
        assert_approx_eq(result[2], 8.0);
        assert_approx_eq(result[3], 16.0);

        let result = geometric_series(4.1, 2.1, 2.1);
        assert_eq!(result.len(), 4);
        assert_approx_eq(result[0], 2.1);
        assert_approx_eq(result[1], 4.41);
        assert_approx_eq(result[2], 9.261);
        assert_approx_eq(result[3], 19.4481);

        let result = geometric_series(4.0, -2.0, 2.0);
        assert_eq!(result.len(), 4);
        assert_approx_eq(result[0], -2.0);
        assert_approx_eq(result[1], -4.0);
        assert_approx_eq(result[2], -8.0);
        assert_approx_eq(result[3], -16.0);
    }
}
