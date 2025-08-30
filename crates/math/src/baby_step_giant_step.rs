use std::collections::HashMap;
use crate::greatest_common_divisor_stein;

/// 使用 Baby-step Giant-step 算法求解离散对数问题 a^x ≡ b (mod n)
///
/// 该算法用于在模 n 的乘法群中寻找满足 a^x ≡ b (mod n) 的整数 x。
/// 要求 a 和 n 互质，否则无解。
///
/// # 参数说明
/// * `a` - 底数，必须与模数 n 互质
/// * `b` - 结果值，即 a^x mod n 的目标值
/// * `n` - 模数，必须大于1
///
/// # 返回值
/// 如果存在解，返回 Some(x)，其中 x 是满足等式的最小非负整数；
/// 如果不存在解或 a 与 n 不互质，返回 None。
#[allow(unused)]
pub fn baby_step_giant_step(a: usize, b: usize, n: usize) -> Option<usize> {
    // 检查 a 和 n 是否互质，如果不互质则无解
    if greatest_common_divisor_stein(a as u64, n as u64) != 1 {
        return None;
    }

    // 创建哈希表存储 baby-step 阶段的结果
    let mut h_map = HashMap::new();

    // 计算步长 m，为 sqrt(n) 向上取整
    let m = (n as f64).sqrt().ceil() as usize;

    // Baby-step 阶段：计算并存储 a^j * b mod n (j = 0, 1, ..., m-1)
    let mut step = 1;
    (0..m).for_each(|idx| {
        h_map.insert((step * b) / n, idx);
        step = (step * a) % n;
    });

    // Giant-step 阶段准备：计算 a^m mod n 作为每次大步的增量
    let giant_step = step;

    // Giant-step 阶段：检查是否存在匹配项
    for idx in (m..=n).step_by(m) {
        // 在哈希表中查找当前值，如果找到则返回解
        if let Some(v) = h_map.get(&step) {
            return Some(idx - v);
        }
        // 更新到下一步的状态
        step = (step * giant_step) % n;
    }

    // 未找到解
    None
}

#[cfg(test)]
mod tests {
    use super::baby_step_giant_step;

    #[test]
    fn small_numbers() {
        assert_eq!(baby_step_giant_step(5, 3, 11), Some(2));
        assert_eq!(baby_step_giant_step(3, 83, 100), Some(9));
        assert_eq!(baby_step_giant_step(9, 1, 61), Some(5));
        assert_eq!(baby_step_giant_step(5, 1, 67), Some(22));
        assert_eq!(baby_step_giant_step(7, 1, 45), Some(12));
    }

    #[test]
    fn primitive_root_tests() {
        assert_eq!(
            baby_step_giant_step(3, 311401496, 998244353),
            Some(178105253)
        );
        assert_eq!(
            baby_step_giant_step(5, 324637211, 1000000007),
            Some(976653449)
        );
    }

    #[test]
    fn random_numbers() {
        assert_eq!(baby_step_giant_step(174857, 48604, 150991), Some(177));
        assert_eq!(baby_step_giant_step(912103, 53821, 75401), Some(2644));
        assert_eq!(baby_step_giant_step(448447, 365819, 671851), Some(23242));
        assert_eq!(
            baby_step_giant_step(220757103, 92430653, 434948279),
            Some(862704)
        );
        assert_eq!(
            baby_step_giant_step(176908456, 23538399, 142357679),
            Some(14215560)
        );
    }

    #[test]
    fn no_solution() {
        assert!(baby_step_giant_step(7, 6, 45).is_none());
        assert!(baby_step_giant_step(23, 15, 85).is_none());
        assert!(baby_step_giant_step(2, 1, 84).is_none());
    }
}
