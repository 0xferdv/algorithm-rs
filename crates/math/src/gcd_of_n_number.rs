#![allow(unused)]

/// 计算一个 usize 数组中所有数字的最大公约数
///
/// # 参数
/// * `nums` - 一个 usize 类型的切片，包含需要计算最大公约数的数字
///
/// # 返回值
/// 返回所有数字的最大公约数，如果数组为空则返回 0
///
/// # 算法说明
/// 使用递归方式，将数组分解为第一个元素和剩余元素的最大公约数，
/// 然后计算这两个数的最大公约数
pub fn gcd(nums: &[usize]) -> usize {
    let len = nums.len();
    // 处理空数组情况
    if len == 0 {
        return 0;
    }
    let start = nums[0];
    let remain = gcd(&nums[1..]);
    gcd_of_two_numbers(start, remain)
}

/// 使用欧几里得算法计算两个数的最大公约数
///
/// # 参数
/// * `start` - 第一个 usize 数字
/// * `remain` - 第二个 usize 数字
///
/// # 返回值
/// 返回两个数字的最大公约数
///
/// # 算法说明
/// 基于欧几里得算法：gcd(a,b) = gcd(b, a mod b)，当 b 为 0 时，gcd 为 a
pub fn gcd_of_two_numbers(start: usize, remain: usize) -> usize {
    // 递归终止条件：当第二个数为 0 时，第一个数即为最大公约数
    if remain == 0 {
        return start;
    }
    // 递归调用：交换参数位置，第二个参数变为第一个数对第二个数的余数
    gcd_of_two_numbers(remain, start % remain)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        assert_eq!(gcd(&[1, 2, 3, 4, 5]), 1);
        assert_eq!(gcd(&[2, 4, 6, 8, 10]), 2);
        assert_eq!(gcd(&[3, 6, 9, 12, 15]), 3);
        assert_eq!(gcd(&[10]), 10);
        assert_eq!(gcd(&[21, 110]), 1);
    }
}
