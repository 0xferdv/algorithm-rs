/// 快速幂算法实现，用于计算 (base^power) % modulus
///
/// 该函数使用二进制指数法来高效计算大数幂运算的模值，时间复杂度为 O(log power)
///
/// # 参数
/// * `base` - 底数，必须大于等于1
/// * `power` - 指数
/// * `modulus` - 模数
///
/// # 返回值
/// 返回 (base^power) % modulus 的结果
///
/// # 算法原理
/// 将指数转换为二进制表示，通过不断平方底数并根据二进制位决定是否累乘到结果中
#[allow(unused)]
pub fn fast_power(mut base: usize, mut power: usize, modulus: usize) -> usize {
    assert!(base >= 1);
    let mut res = 1;

    // 使用二进制指数法进行快速幂计算
    while power > 0 {
        // 如果当前位为1，则将当前base乘入结果
        if power & 1 == 1 {
            res = (res * base) % modulus;
        }
        // base自乘并取模，为下一位计算做准备
        base = (base * base) % modulus;
        // 右移一位，处理下一个二进制位
        power >>= 1;
    }
    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        const MOD: usize = 1000000007;
        assert_eq!(fast_power(2, 1, MOD), 2);
        assert_eq!(fast_power(2, 2, MOD), 4);
        assert_eq!(fast_power(2, 4, MOD), 16);
        assert_eq!(fast_power(3, 4, MOD), 81);
        assert_eq!(fast_power(2, 100, MOD), 976371285);
    }
}
