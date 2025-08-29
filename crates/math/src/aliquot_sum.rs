/// 计算一个正整数的真因数之和（Aliquot Sum）
///
/// 真因数是指能整除给定正整数但不包括该数本身的正整数。
/// 例如：6的真因数是1, 2, 3，它们的和是6。
///
/// # 参数
/// * `number` - 要计算真因数之和的正整数
///
/// # 返回值
/// 返回给定数字的所有真因数的和
///
/// # Panics
/// 当输入为0时会panic，因为0不是正整数
#[allow(unused)]
pub fn aliquot_sum(number: u64) -> u64 {
    if number == 0 {
        panic!("Input has to be positive.")
    }
    // 计算从1到number/2的所有数字中能整除number的数的和
    // 优化：只需要检查到number/2，因为大于number/2的数不可能是真因数
    (1..=number / 2).filter(|&d| number % d == 0).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! test_aliquot_sum {
        ($($name:ident: $tc:expr,)*) => {
        $(
            #[test]
            fn $name() {
                let (number, expected) = $tc;
                assert_eq!(aliquot_sum(number), expected);
            }
        )*
        }
    }

    test_aliquot_sum! {
        test_with_1: (1, 0),
        test_with_2: (2, 1),
        test_with_3: (3, 1),
        test_with_4: (4, 1+2),
        test_with_5: (5, 1),
        test_with_6: (6, 6),
        test_with_7: (7, 1),
        test_with_8: (8, 1+2+4),
        test_with_9: (9, 1+3),
        test_with_10: (10, 1+2+5),
        test_with_15: (15, 9),
        test_with_343: (343, 57),
        test_with_344: (344, 316),
        test_with_500: (500, 592),
        test_with_501: (501, 171),
    }

    #[test]
    #[should_panic]
    fn panics_if_input_is_zero() {
        aliquot_sum(0);
    }
}
