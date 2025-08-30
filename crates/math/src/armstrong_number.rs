/// 判断一个数字是否为阿姆斯特朗数（水仙花数）
///
/// 阿姆斯特朗数是指一个n位数，其各位数字的n次幂之和等于该数本身。
/// 例如：153 = 1³ + 5³ + 3³ = 1 + 125 + 27 = 153
///
/// # 参数
/// * `number` - 需要判断的无符号32位整数
///
/// # 返回值
/// 如果是阿姆斯特朗数返回true，否则返回false
#[allow(unused)]
pub fn is_armstrong_number(number: u32) -> bool {
    // 提取数字的每一位并存储到向量中
    let mut digits: Vec<u32> = Vec::new();
    let mut num: u32 = number;
    loop {
        digits.push(num % 10);
        num /= 10;
        if num ==0 {
            break;
        }
    }

    // 计算每位数字的n次幂之和（n为数字的位数）
    let sum_nth_power_of_digits: u32 = digits
        .iter()
        .map(|digit| digit.pow(digits.len() as u32))
        .sum();

    // 判断幂次和是否等于原数字
    sum_nth_power_of_digits == number
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_digit_armstrong_number() {
        assert!(is_armstrong_number(1))
    }
    #[test]
    fn two_digit_numbers_are_not_armstrong_numbers() {
        assert!(!is_armstrong_number(15))
    }
    #[test]
    fn three_digit_armstrong_number() {
        assert!(is_armstrong_number(153))
    }
    #[test]
    fn three_digit_non_armstrong_number() {
        assert!(!is_armstrong_number(105))
    }
    #[test]
    fn big_armstrong_number() {
        assert!(is_armstrong_number(912985153))
    }
}
