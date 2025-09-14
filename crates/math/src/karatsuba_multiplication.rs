#![allow(unused)]

use std::cmp::max;
use num_traits::Saturating;

const TEN: i128 = 10;

/// 使用分治法实现的大整数乘法函数（Karatsuba算法）
///
/// 该函数通过递归地将大整数拆分为较小的部分，使用Karatsuba算法来提高乘法效率。
/// 当数字小于10时直接相乘，否则将其分割为两部分进行递归计算。
///
/// # 参数
/// * `num1` - 第一个要相乘的i128整数
/// * `num2` - 第二个要相乘的i128整数
///
/// # 返回值
/// 返回两个输入整数的乘积
pub fn multiply(num1: i128, num2: i128) -> i128 {
    _multiply(num1, num2)
}

/// 实现Karatsuba乘法算法的核心函数
///
/// 该函数使用分治策略将大整数分解为更小的部分进行计算。
/// 对于小数字直接相乘，对于大数字则按照Karatsuba算法公式进行递归处理。
///
/// # 参数
/// * `num1` - 第一个要相乘的i128整数
/// * `num2` - 第二个要相乘的i128整数
///
/// # 返回值
/// 返回两个输入整数的乘积
fn _multiply(num1: i128, num2: i128) -> i128 {
    // 基本情况：当任一数字小于10时，直接相乘返回结果
    if num1 < 10 || num2 < 10 {
        return num1 * num2;
    }

    // 将数字转换为字符串并规范化长度，确保两数具有相同的位数
    let mut num1_str = num1.to_string();
    let mut num2_str = num2.to_string();
    let n = max(num1_str.len(), num2_str.len());
    num1_str = normalize(num1_str, n);
    num2_str = normalize(num2_str, n);

    // 将两个数字分别拆分为高位和低位部分
    let a = &num1_str[0..n / 2];
    let b = &num1_str[n / 2..];
    let c = &num2_str[0..n / 2];
    let d = &num2_str[n / 2..];

    // 递归计算ac、bd以及(a+b)(c+d)的乘积
    let ac = _multiply(a.parse().unwrap(), c.parse().unwrap());
    let bd = _multiply(b.parse().unwrap(), d.parse().unwrap());
    let a_b: i128 = a.parse::<i128>().unwrap() + b.parse::<i128>().unwrap();
    let c_d: i128 = c.parse::<i128>().unwrap() + d.parse::<i128>().unwrap();
    let ad_bc = _multiply(a_b, c_d) - (ac + bd);

    // 根据Karatsuba算法公式组合结果：ac * 10^(2*m) + (ad+bc) * 10^m + bd
    let m = n / 2 + n % 2;
    (TEN.pow(2 * m as u32) * ac) + (TEN.pow(m as u32) * ad_bc) + (bd)
}

/// 将字符串表示的数字规范化为指定长度
///
/// 通过在字符串前面添加前导零，使数字字符串达到指定的长度。
///
/// # 参数
/// * `a` - 需要规范化的数字字符串
/// * `n` - 目标长度
///
/// # 返回值
/// 返回规范化后的字符串，长度为n，前面可能包含前导零
fn normalize(mut a: String, n: usize) -> String {
    let padding = n.saturating_sub(a.len());
    a.insert_str(0, &"0".repeat(padding));
    a
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_1() {
        let n1: i128 = 314159265;
        let n2: i128 = 314159265;
        let ans = multiply(n1, n2);
        assert_eq!(ans, n1 * n2);
    }

    #[test]
    fn test_2() {
        let n1: i128 = 3141592653589793232;
        let n2: i128 = 2718281828459045233;
        let ans = multiply(n1, n2);
        assert_eq!(ans, n1 * n2);
    }

    #[test]
    fn test_3() {
        let n1: i128 = 123456789;
        let n2: i128 = 101112131415;
        let ans = multiply(n1, n2);
        assert_eq!(ans, n1 * n2);
    }
}
