/// 计算给定函数在指定区间内的近似面积（数值积分）
///
/// 使用梯形法则来估算函数曲线下的面积。将区间分割为多个小梯形，
/// 然后累加每个梯形的面积得到总面积的近似值。
///
/// # 参数
/// * `start`: 积分区间的起始点
/// * `end`: 积分区间的结束点
/// * `func`: 要进行积分的函数，接受一个f64参数并返回f64值
/// * `step_count`: 将区间分割成的梯形数量，越多精度越高
///
/// # 返回值
/// 返回函数在指定区间内曲线下的近似面积
#[allow(unused)]
pub fn area_under_curve(
    start: f64,
    end: f64,
    func: fn(f64) -> f64,
    step_count: usize,
) -> f64 {
    // 确保起始点小于结束点，如果顺序相反则交换
    let (start, end) = if start > end {
        (end, start)
    } else {
        (start, end)
    };

    // 计算每个梯形的宽度
    let step_length: f64 = (end - start) / step_count as f64;
    let mut area = 0f64;

    // 初始化第一个点的函数值
    let mut fx1 = func(start);
    let mut fx2: f64;

    // 遍历每个分割点，计算梯形面积并累加
    for eval_point in (1..=step_count).map(|x| (x as f64 * step_length) + start) {
        fx2 = func(eval_point);
        // 使用梯形面积公式：(上底 + 下底) * 高 / 2
        area += (fx2 + fx1).abs() * step_length * 0.5;
        fx1 = fx2;
    }

    area
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_linear_func() {
        assert_eq!(area_under_curve(1f64, 2f64, |x| x, 10), 1.5000000000000002);
    }

    #[test]
    fn test_quadratic_func() {
        assert_eq!(
            area_under_curve(1f64, 2f64, |x| x * x, 1000),
            2.333333500000005
        );
    }

    #[test]
    fn test_zero_length() {
        assert_eq!(area_under_curve(0f64, 0f64, |x| x * x, 1000), 0.0);
    }

    #[test]
    fn test_reverse() {
        assert_eq!(
            area_under_curve(1f64, 2f64, |x| x, 10),
            area_under_curve(2f64, 1f64, |x| x, 10)
        );
    }
}
