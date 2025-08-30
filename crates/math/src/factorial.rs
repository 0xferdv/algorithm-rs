use num_bigint::BigUint;
use num_traits::One;
#[allow(unused_imports)]
use std::str::FromStr;

/// 计算给定非负整数的阶乘（使用迭代方式）
///
/// # 参数
/// * `number` - 要计算阶乘的非负整数
///
/// # 返回值
/// 返回 number 的阶乘结果
///
/// # 说明
/// 当 number 为 0 或 1 时，返回 1；
/// 否则返回从 2 到 number 所有整数的乘积
#[allow(unused)]
pub fn factorial(number: u64) -> u64 {
    if number == 0 || number == 1 {
        1
    } else {
        // 使用迭代方式计算阶乘：将 2 到 number 的所有数字相乘
        (2..=number).product()
    }
}

/// 计算给定非负整数的阶乘（使用递归方式）
///
/// # 参数
/// * `number` - 要计算阶乘的非负整数
///
/// # 返回值
/// 返回 number 的阶乘结果
///
/// # 说明
/// 当 number 为 0 或 1 时，返回 1；
/// 否则返回 number 与 (number-1) 阶乘的乘积
#[allow(unused)]
pub fn factorial_recursive(number: u64) -> u64 {
    if number == 0 || number == 1 {
        1
    } else {
        // 递归调用：number * (number-1)!
        number * factorial_recursive(number - 1)
    }
}

/// 使用大数运算计算给定非负整数的阶乘
///
/// # 参数
/// * `numbers` - 要计算阶乘的非负整数
///
/// # 返回值
/// 返回 numbers 的阶乘结果，类型为 BigUint 以支持大数运算
///
/// # 说明
/// 此函数适用于需要处理超出 u64 范围的大数阶乘计算
pub fn factorial_big_math(numbers: u32) -> BigUint {
    // 初始化结果为 1
    let mut result: BigUint = One::one();

    // 循环计算阶乘：result = 1 * 2 * 3 * ... * numbers
    for idx in 1..=numbers {
        result *= idx;
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_factorial() {
        assert_eq!(factorial(0), 1);
        assert_eq!(factorial(1), 1);
        assert_eq!(factorial(6), 720);
        assert_eq!(factorial(10), 3628800);
        assert_eq!(factorial(20), 2432902008176640000);
    }

    #[test]
    fn test_factorial_recursive() {
        assert_eq!(factorial_recursive(0), 1);
        assert_eq!(factorial_recursive(1), 1);
        assert_eq!(factorial_recursive(6), 720);
        assert_eq!(factorial_recursive(10), 3628800);
        assert_eq!(factorial_recursive(20), 2432902008176640000);
    }

    #[test]
    fn basic_factorial() {
        assert_eq!(factorial_big_math(10), BigUint::from_str("3628800").unwrap());
        assert_eq!(
            factorial_big_math(50),
            BigUint::from_str("30414093201713378043612608166064768844377641568960512000000000000")
                .unwrap()
        );
    }
}
