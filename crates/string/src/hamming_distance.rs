#[derive(Debug, PartialEq)]
pub enum HammingDistanceError {
    InputStringHaveDifferentLength,
}

/// 计算两个字符串之间的汉明距离（Hamming Distance）
///
/// 汉明距离是指两个等长字符串在相同位置上不同字符的个数。
///
/// # 参数
/// * `s1` - 第一个字符串切片
/// * `s2` - 第二个字符串切片
///
/// # 返回值
/// * `Ok(usize)` - 返回两个字符串的汉明距离
/// * `Err(HammingDistanceError)` - 当两个字符串长度不同时返回错误
///
/// # 错误
/// * `HammingDistanceError::InputStringHaveDifferentLength` - 当输入的两个字符串长度不同时返回此错误
#[allow(unused)]
pub fn hamming_distance(s1: &str, s2: &str) -> Result<usize, HammingDistanceError> {
    // 检查两个字符串长度是否相等
    if s1.len() != s2.len() {
        return Err(HammingDistanceError::InputStringHaveDifferentLength);
    }

    // 通过逐字符比较计算汉明距离
    let distance = s1
        .chars()
        .zip(s2.chars())
        .filter(|(c1, c2)| c1 != c2)
        .count();
    Ok(distance)
}



#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! test_hamming_distance {
        ($($name:ident: $tc:expr,)*) => {
            $(
                #[test]
                fn $name() {
                    let (str_a, str_b, expected) = $tc;
                    assert_eq!(hamming_distance(str_a, str_b), expected);
                    assert_eq!(hamming_distance(str_b, str_a), expected);
                }
            )*
        }
    }

    test_hamming_distance! {
        empty_inputs: ("", "", Ok(0)),
        different_length: ("0", "", Err(HammingDistanceError::InputStringHaveDifferentLength)),
        length_1_inputs_identical: ("a", "a", Ok(0)),
        length_1_inputs_different: ("a", "b", Ok(1)),
        same_strings: ("rust", "rust", Ok(0)),
        regular_input_0: ("karolin", "kathrin", Ok(3)),
        regular_input_1: ("kathrin", "kerstin", Ok(4)),
        regular_input_2: ("00000", "11111", Ok(5)),
        different_case: ("x", "X", Ok(1)),
        strings_with_no_common_chars: ("abcd", "wxyz", Ok(4)),
        long_strings_one_diff: (&"a".repeat(1000), &("a".repeat(999) + "b"), Ok(1)),
        long_strings_many_diffs: (&("a".repeat(500) + &"b".repeat(500)), &("b".repeat(500) + &"a".repeat(500)), Ok(1000)),
        strings_with_special_chars_identical: ("!@#$%^", "!@#$%^", Ok(0)),
        strings_with_special_chars_diff: ("!@#$%^", "&*()_+", Ok(6)),
    }
}
