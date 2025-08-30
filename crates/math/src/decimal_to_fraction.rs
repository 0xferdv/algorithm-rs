/// 将一个浮点数转换为最简分数形式。
///
/// 该函数接收一个 `f64` 类型的十进制数，将其表示为分子和分母组成的元组，
/// 并确保返回的分数是最简形式（即分子和分母的最大公约数为1）。
///
/// # 参数
///
/// * `decimal`: 需要转换的十进制数（f64 类型）
///
/// # 返回值
///
/// 返回一个元组 `(i64, i64)`，其中第一个元素是分子，第二个是分母。
///
/// # 示例
///
///
/// let result = decimal_to_fraction(1.5);
/// assert_eq!(result, (3, 2));
///
#[allow(unused)]
pub fn decimal_to_fraction(decimal: f64) -> (i64, i64) {
    // 提取小数部分
    let fractional_part = decimal - decimal.floor();

    // 如果没有小数部分，直接返回整数形式
    if fractional_part == 0.0 {
        (decimal as i64, 1)
    } else {
        // 计算小数点后的位数
        let number_of_frac_digits = decimal
            .to_string()
            .split('.')
            .nth(1)
            .unwrap_or("")
            .len();

        // 将小数转换为整数形式作为初始分子
        let numerator = (decimal * 10f64.powi(number_of_frac_digits as i32)) as i64;
        // 初始分母为10的相应次幂
        let denominator = 10i64.pow(number_of_frac_digits as u32);

        // 使用欧几里得算法计算最大公约数(GCD)
        let mut divisor = denominator;
        let mut dividend = numerator;
        while divisor != 0 {
            let r = dividend % divisor;
            dividend = divisor;
            divisor = r;
        }
        let gcd = dividend.abs();

        // 化简分数
        let numerator = numerator / gcd;
        let denominator = denominator / gcd;

        (numerator, denominator)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_decimal_to_fraction_1() {
        assert_eq!(decimal_to_fraction(2.0), (2, 1));
    }

    #[test]
    fn test_decimal_to_fraction_2() {
        assert_eq!(decimal_to_fraction(89.45), (1789, 20));
    }

    #[test]
    fn test_decimal_to_fraction_3() {
        assert_eq!(decimal_to_fraction(67.), (67, 1));
    }

    #[test]
    fn test_decimal_to_fraction_4() {
        assert_eq!(decimal_to_fraction(45.2), (226, 5));
    }

    #[test]
    fn test_decimal_to_fraction_5() {
        assert_eq!(decimal_to_fraction(1.5), (3, 2));
    }

    #[test]
    fn test_decimal_to_fraction_6() {
        assert_eq!(decimal_to_fraction(6.25), (25, 4));
    }
}

