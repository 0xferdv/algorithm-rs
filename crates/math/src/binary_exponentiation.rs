#[allow(unused)]
pub fn binary_exponentiation(mut base: u64, mut exponent: u32) -> u64 {
    let mut result_pow: u64 = 1;
    // 当指数大于0时继续循环
    while exponent > 0 {
        // 检查指数的最低位是否为1，如果是则将当前底数乘入结果
        if exponent & 1 == 1 {
            result_pow *= base;
        }
        // 指数右移一位（相当于除以2）
        exponent >>= 1;
        // 底数平方，为下一次迭代做准备
        base *= base;
    }

    result_pow
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        assert_eq!(binary_exponentiation(2, 3), 8);
        assert_eq!(binary_exponentiation(4, 12), 16777216);
        assert_eq!(binary_exponentiation(6, 12), 2176782336);
        assert_eq!(binary_exponentiation(10, 4), 10000);
        assert_eq!(binary_exponentiation(20, 3), 8000);
        assert_eq!(binary_exponentiation(3, 21), 10460353203);
    }

    #[test]
    fn up_to_ten() {
        for i in 0..10 {
            for j in 0..10 {
                println!("{i}, {j}");
                assert_eq!(binary_exponentiation(i, j), u64::pow(i, j))
            }
        }
    }
}
