#![allow(unused)]
/// 计算Huber损失函数值
///
/// Huber损失是均方误差和平均绝对误差的结合，对于小的残差使用平方损失，
/// 对于大的残差使用线性损失，这样既保持了平方损失的光滑性，又增强了对异常值的鲁棒性。
///
/// 参数:
/// * `actual`: 真实值的切片引用
/// * `predicted`: 预测值的切片引用
/// * `delta`: Huber损失的阈值参数，决定从平方损失切换到线性损失的边界
///
/// 返回值:
/// 返回计算得到的Huber损失值
pub fn huber_loss(actual: &[f64], predicted: &[f64], delta: f64) -> f64 {
    let mut loss: Vec<f64> = Vec::new();
    // 遍历真实值和预测值，计算每个样本的Huber损失
    for (a, p) in actual.iter().zip(predicted.iter()) {
        if (a - p).abs() <= delta {
            // 当残差绝对值小于等于delta时，使用平方损失
            loss.push(0.5 * (a - p).powf(2.));
        } else {
            // 当残差绝对值大于delta时，使用线性损失
            loss.push(delta * (a - p).abs() - (0.5 * delta));
        }
    }
    // 对所有样本的损失求和
    loss.iter().sum::<f64>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_huber_loss() {
        let test_vector_actual = vec![1.0, 2.0, 3.0, 4.0, 5.0];
        let test_vector = vec![5.0, 7.0, 9.0, 11.0, 13.0];
        assert_eq!(huber_loss(&test_vector_actual, &test_vector, 1.0), 27.5);
    }
}
