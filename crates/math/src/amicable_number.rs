/// 查找小于给定数值 n 的所有亲和数对（Amicable Numbers）。
///
/// 亲和数对是指两个不同的正整数，其中每一个数都是另一个数的真因数之和。
/// 例如：220 和 284 是一对亲和数，因为：
/// - 220 的真因数之和为 284
/// - 284 的真因数之和为 220
///
/// # 参数
/// * `n` - 上限值，查找所有小于该值的亲和数对
///
/// # 返回值
/// * `Option<Vec<(u32, u32)>>` - 如果存在亲和数对，返回包含所有亲和数对的向量；
///                               如果不存在，则返回 None
#[allow(unused)]
pub fn amicable_numbers(n: u32) -> Option<Vec<(u32, u32)>> {
    // 预先计算每个数的真因数之和，factor_sums[i] 表示数字 i 的真因数之和
    let mut factor_sums = vec![0; n as usize];
    for idx in 1..n {
        // 从 idx 的倍数开始，将 idx 加到其倍数的因数和中
        for next_idx in (idx * 2..n).step_by(idx as usize) {
            factor_sums[next_idx as usize] += idx;
        }
    }

    // 存储找到的亲和数对
    let mut out = vec![(0, 0)];
    for (idx, number) in factor_sums.iter().enumerate() {
        // 检查是否构成亲和数对：
        // 1. number 必须小于 n
        // 2. number 的真因数之和必须等于当前索引 idx
        // 3. number 必须大于 idx（避免重复和完全数）
        if (*number < n) && (factor_sums[*number as usize] == idx as u32) && (*number > idx as u32) {
            out.push((idx as u32, *number));
        }
    }

    // 如果没有找到任何亲和数对，返回 None；否则返回结果向量
    if out.len() == 1 {
        None
    } else {
        out.remove(0);
        Some(out)
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test_amicable_numbers_below_n() {
        let expected_result = vec![
            (220, 284),
            (1184, 1210),
            (2620, 2924),
            (5020, 5564),
            (6232, 6368),
            (10744, 10856),
            (12285, 14595),
            (17296, 18416),
            (63020, 76084),
            (66928, 66992),
        ];

        let mut result = amicable_numbers(100_000).unwrap();

        assert_eq!(result.len(), 13);

        result = result[..10].to_vec();
        assert_eq!(result, expected_result);

        assert_eq!(amicable_numbers(100), None);
    }
}
