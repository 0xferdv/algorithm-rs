use std::collections::HashSet;

/// 错误类型枚举，用于表示唇语检查过程中可能出现的错误。
///
/// - `NonAlphabeticCharacter`: 输入字符串中包含非字母字符。
/// - `NonLowercaseMissingChar`: 指定缺失字符集中包含非小写字母。
#[derive(Debug, PartialEq, Eq)]
pub enum LipogramError {
    NonAlphabeticCharacter,
    NonLowercaseMissingChar,
}

/// 计算输入字符串中缺失的小写字母集合。
///
/// 该函数将输入字符串转换为小写，并提取其中所有英文字母，
/// 然后与完整的英文字母表进行差集运算，得到未出现的字母集合。
///
/// # 参数
/// * `in_str` - 需要分析的输入字符串。
///
/// # 返回值
/// 返回一个HashSet，包含在输入字符串中没有出现的所有小写字母。
fn compute_missing(in_str: &str) -> HashSet<char> {
    // 构造完整的英文字母表集合
    let alphabet: HashSet<char> = ('a'..='z').collect();

    // 提取输入字符串中的所有小写英文字母并构造集合
    let letters_used: HashSet<char> = in_str
        .to_lowercase()
        .chars()
        .filter(|c| c.is_ascii_alphabetic())
        .collect();

    // 计算字母表与已使用字母的差集，即缺失的字母
    alphabet.difference(&letters_used).cloned().collect()
}

/// 判断给定字符串是否为指定缺失字符的唇语（Lipogram）。
///
/// 唇语是指刻意避免使用某些特定字母的文本。此函数验证输入字符串
/// 是否恰好缺少指定的那些小写字母，并且不包含任何非英文字母字符。
///
/// # 参数
/// * `lipogram_str` - 待检测的字符串。
/// * `missing_chars` - 应该缺失的字符集合，必须全部是小写字母。
///
/// # 返回值
/// 如果字符串恰好缺少指定字符且无非法字符，则返回 `Ok(true)`；
/// 如果字符串不符合要求，则返回 `Ok(false)`；
/// 如果参数或输入有误，则返回相应的 `LipogramError` 错误。
#[allow(unused)]
pub fn lipogram(
    lipogram_str: &str,
    missing_chars: &HashSet<char>) -> Result<bool, LipogramError> {
    // 检查缺失字符集合中的所有字符是否都是小写字母
    for &c in missing_chars {
        if !c.is_lowercase() {
            return Err(LipogramError::NonLowercaseMissingChar);
        }
    }

    // 检查输入字符串中的所有字符是否都是ASCII英文字母
    for c in lipogram_str.chars() {
        if !c.is_ascii_alphabetic() {
            return Err(LipogramError::NonAlphabeticCharacter);
        }
    }

    // 计算实际缺失的字符集合并与期望值比较
    let missing = compute_missing(lipogram_str);
    Ok(missing == *missing_chars)
}

#[cfg(test)]
mod tests {
    use super::*;

    /// 测试宏：批量生成lipogram函数的单元测试用例。
    ///
    /// 每个测试用例由三部分组成：
    /// - 输入字符串
    /// - 期望缺失的字符集合
    /// - 期望的返回结果（Ok(true)/Ok(false)/Err）
    macro_rules! test_lipogram {
        ($($name:ident: $tc:expr,)*) => {
            $(
                #[test]
                fn $name() {
                    let (input, missing_chars, expected) = $tc;
                    assert_eq!(lipogram(input, &missing_chars), expected);
                }
            )*
        }
    }

    // 定义多个测试用例以验证lipogram函数的正确性
    test_lipogram! {
        perfect_pangram: (
            "The quick brown fox jumps over the lazy dog",
            HashSet::from([]),
            Ok(true)
        ),
        lipogram_single_missing: (
            "The quick brown fox jumped over the lazy dog",
            HashSet::from(['s']),
            Ok(true)
        ),
        lipogram_multiple_missing: (
            "The brown fox jumped over the lazy dog",
            HashSet::from(['q', 'i', 'c', 'k', 's']),
            Ok(true)
        ),
        long_lipogram_single_missing: (
            "A jovial swain should not complain of any buxom fair who mocks his pain and thinks it gain to quiz his awkward air",
            HashSet::from(['e']),
            Ok(true)
        ),
        invalid_non_lowercase_chars: (
            "The quick brown fox jumped over the lazy dog",
            HashSet::from(['X']),
            Err(LipogramError::NonLowercaseMissingChar)
        ),
        invalid_non_alphabetic_input: (
            "The quick brown fox jumps over the lazy dog 123@!",
            HashSet::from([]),
            Err(LipogramError::NonAlphabeticCharacter)
        ),
    }
}
