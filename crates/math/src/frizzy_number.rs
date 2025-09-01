#![allow(unused)]

/// 计算第n个"frizzy"数
///
/// 该函数通过将n转换为二进制表示，然后根据二进制位来计算加权和。
/// 每个二进制位对应的权重是base的幂次，幂次从0开始递增。
///
/// # 参数
/// * `base` - 基数，用于计算权重的底数
/// * `n` - 序号，用于确定二进制位模式
///
/// # 返回值
/// 返回计算得到的第n个frizzy数，类型为f64
pub fn get_nth_frizzy(base: i32, mut n: i32) -> f64 {
    let mut final1 = 0.0;
    let mut idx = 0;

    // 将n转换为二进制表示，通过不断除以2获取每一位二进制位
    // 对于每一位二进制位，如果为1则加上对应的权重(base^idx)
    while n > 0 {
        final1 += (base.pow(idx) as f64) * ((n % 2) as f64);
        idx += 1;
        n /= 2;
    }
    final1
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_nth_frizzy() {

        assert_eq!(get_nth_frizzy(3, 4), 9.0);

        assert_eq!(get_nth_frizzy(2, 5), 5.0);

        assert_eq!(get_nth_frizzy(4, 3), 5.0);

        assert_eq!(get_nth_frizzy(5, 2), 5.0);

        assert_eq!(get_nth_frizzy(6, 1), 1.0);
    }
}


