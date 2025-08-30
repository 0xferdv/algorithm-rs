type Vector = Vec<f64>;

/// 计算两个向量之间的欧几里得距离
///
/// 该函数通过计算两个向量对应元素差值的平方和的平方根来得到欧几里得距离
///
/// # 参数
/// * `vector_1` - 第一个向量的引用
/// * `vector_2` - 第二个向量的引用
///
/// # 返回值
/// 返回两个向量之间的欧几里得距离
///
/// # 注意事项
/// 两个向量的长度必须相同，否则只会计算较短部分的距离
#[allow(unused)]
pub fn euclidean_distance(vector_1: &Vector, vector_2: &Vector) -> f64 {
    // 计算两个向量对应元素差值的平方和
    let squared_sum: f64 = vector_1
        .iter()
        .zip(vector_2.iter())
        .map(|(&x, &y)| (x - y).powi(2))
        .sum();
    // 对平方和开平方根得到欧几里得距离
    squared_sum.sqrt()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_euclidean_distance() {
        let vec1_2d = vec![1.0, 2.0];
        let vec2_2d = vec![4.0, 6.0];

        let result_2d = euclidean_distance(&vec1_2d, &vec2_2d);
        assert_eq!(result_2d, 5.0);

        let vec1_4d = vec![1.0, 2.0, 3.0, 4.0];
        let vec2_4d = vec![5.0, 6.0, 7.0, 8.0];

        let result_4d = euclidean_distance(&vec1_4d, &vec2_4d);
        assert_eq!(result_4d, 8.0);
    }
}
