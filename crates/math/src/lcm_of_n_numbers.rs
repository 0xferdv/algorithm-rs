#![allow(unused)]

/// 计算一个 usize 数字切片的最小公倍数(LCM)
///
/// # 参数
/// * `nums` - 一个 usize 类型的切片，包含需要计算最小公倍数的数字
///
/// # 返回值
/// 返回切片中所有数字的最小公倍数
///
/// # 算法说明
/// 使用递归方法，通过 LCM(a,b) = a*b/GCD(a,b) 的公式计算
/// 对于多个数字，LCM(a,b,c) = LCM(a, LCM(b,c))
pub fn lcm(nums: &[usize]) -> usize {
    if nums.len() == 1 {
        return nums[0];
    }
    let a = nums[0];
    let b = lcm(&nums[1..]);
    a * b / gcd_of_two_numbers(a, b)
}

/// 使用欧几里得算法计算两个 usize 数字的最大公约数(GCD)
///
/// # 参数
/// *  - 第一个 usize 类型的数字
/// *  - 第二个 usize 类型的数字
///
/// # 返回值
/// 返回两个数字的最大公约数
///
/// # 算法说明
/// 使用递归实现的欧几里得算法：GCD(a,b) = GCD(b, a%b)，当 b=0 时返回 a
pub fn gcd_of_two_numbers(a: usize, b: usize) -> usize {
    if b == 0 {
        return a;
    }
    gcd_of_two_numbers(b, a % b)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lcm() {
        assert_eq!(lcm(&[1, 2, 3, 4, 5]), 60);
        assert_eq!(lcm(&[2, 3, 4, 5]), 60);
        assert_eq!(lcm(&[1, 2, 3, 4, 5, 6]), 60);
        assert_eq!(lcm(&[1, 2, 3, 4, 5, 6, 7, 8]), 840);
    }
}
