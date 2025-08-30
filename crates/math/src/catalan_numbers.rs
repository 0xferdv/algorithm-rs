const MOD: i64 = 1_000_000_007;
const MAX: usize = 1_005;

/// 初始化并计算卡塔兰数列
///
/// 使用动态规划方法计算前 MAX 个卡塔兰数，结果对 MOD 取模
/// 卡塔兰数的递推公式为: C(n) = Σ(i=0 to n-1) C(i) * C(n-1-i)
///
/// # 返回值
/// 返回包含前 MAX 个卡塔兰数的向量，索引对应数列位置
#[allow(unused)]
pub fn init_catalan() -> Vec<i64> {
    let mut catalan = vec![0; MAX];
    catalan[0] = 1;
    catalan[1] = 1;

    // 使用动态规划计算卡塔兰数列
    // 对于每个位置 idx，计算所有可能的二叉树分解组合数
    (2..MAX).for_each(|idx| {
        catalan[idx] = 0;
        (0..idx).for_each(|next_idx| {
            catalan[idx] += catalan[next_idx] * catalan[idx - next_idx - 1] % MOD;
            if catalan[idx] >= MOD {
                catalan[idx] -= MOD;
            }
        });
    });
    catalan
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_catalan() {
        let catalan = init_catalan();

        assert_eq!(catalan[0], 1);

        assert_eq!(catalan[1], 1);

        assert_eq!(catalan[5], 42);

        assert_eq!(catalan[10], 16796);

        assert_eq!(catalan[15], 9694845);

        println!("All tests passed!");
    }
}
