/// 查找一个数字的第k个因子
///
/// 该函数用于查找给定正整数n的所有因子，并返回第k个因子（按升序排列）。
/// 如果因子数量不足k个，则返回-1。
///
/// # 参数
/// * `n` - 要查找因子的正整数
/// * `k` - 要返回的因子序号（从1开始计数）
///
/// # 返回值
/// 返回n的第k个因子，如果不存在则返回-1
#[allow(unused)]
pub fn kth_factor(n: i32, k: i32) -> i32 {
    let mut factors: Vec<i32> = Vec::new();
    let k = (k as usize) - 1;

    // 遍历从1到n的所有数字，查找n的因子
    for i in 1..=n {
        if n % i == 0 {
            factors.push(i);
        }
        // 每次找到因子后检查是否已找到第k个因子，如果是则直接返回
        if let Some(number) = factors.get(k) {
            return *number;
        }
    }
    -1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(kth_factor(12, 3), 3);
    }

    #[test]
    fn test_2() {
        assert_eq!(kth_factor(7, 2), 7);
    }

    #[test]
    fn test_3() {
        assert_eq!(kth_factor(4, 4), -1);
    }

    #[test]
    fn test_4() {
        assert_eq!(kth_factor(950, 5), 19);
    }
}
