/// 计算欧拉函数 φ(n) 的值。
///
/// 欧拉函数 φ(n) 表示小于或等于 n 的正整数中与 n 互质的数的个数。
/// 该函数使用质因数分解的方法来计算欧拉函数：
/// 对于一个正整数 n，若其质因数分解为 p1^k1 * p2^k2 * ... * pm^km，
/// 则 φ(n) = n * (1 - 1/p1) * (1 - 1/p2) * ... * (1 - 1/pm)
///
/// # 参数
/// * `n` - 要计算欧拉函数的正整数（u64 类型）
///
/// # 返回值
/// 返回 φ(n) 的值，即小于或等于 n 且与 n 互质的正整数个数（u64 类型）
#[allow(unused)]
pub fn euler_totient(n: u64) -> u64 {
    let mut result = n;
    let mut num = n;
    let mut p = 2;

    // 遍历所有可能的质因数 p，直到 p*p > num
    while p * p <= num {
        // 如果 p 是 num 的因数
        if num % p == 0 {
            // 将 num 中所有的 p 因子除尽
            while num % p == 0 {
                num /= p;
            }
            // 根据公式更新结果：result = result * (1 - 1/p)
            result -= result / p;
        }
        p += 1;
    }
    // 如果最后剩下的 num 大于 1，说明它是一个大于 sqrt(n) 的质因数
    if num > 1 {
        result -= result / num;
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;
    macro_rules! test_euler_totient {
        ($($name:ident: $test_case:expr,)*) => {
            $(
                #[test]
                fn $name() {
                    let (input, expected) = $test_case;
                    assert_eq!(euler_totient(input), expected)
                }
            )*
        };
    }

    test_euler_totient! {
        prime_2: (2, 1),
        prime_3: (3, 2),
        prime_5: (5, 4),
        prime_7: (7, 6),
        prime_11: (11, 10),
        prime_13: (13, 12),
        prime_17: (17, 16),
        prime_19: (19, 18),

        composite_6: (6, 2),     // 2 * 3
        composite_10: (10, 4),   // 2 * 5
        composite_15: (15, 8),   // 3 * 5
        composite_12: (12, 4),   // 2^2 * 3
        composite_18: (18, 6),   // 2 * 3^2
        composite_20: (20, 8),   // 2^2 * 5
        composite_30: (30, 8),   // 2 * 3 * 5

        prime_power_2_to_2: (4, 2),
        prime_power_2_to_3: (8, 4),
        prime_power_3_to_2: (9, 6),
        prime_power_2_to_4: (16, 8),
        prime_power_5_to_2: (25, 20),
        prime_power_3_to_3: (27, 18),
        prime_power_2_to_5: (32, 16),

        // Large numbers
        large_50: (50, 20),      // 2 * 5^2
        large_100: (100, 40),    // 2^2 * 5^2
        large_1000: (1000, 400), // 2^3 * 5^3
    }
}
