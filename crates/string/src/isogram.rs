use std::collections::HashMap;

/// 错误类型枚举，用于表示判断isogram过程中可能遇到的错误。
#[derive(Debug, PartialEq, Eq)]
pub enum IsogramError {
    /// 输入字符串中包含非字母字符（除了空格）时返回此错误。
    NonAlphabeticCharacter,
}

/// 统计字符串中每个英文字母出现的次数。
///
/// 该函数会将输入字符串转换为小写，并忽略空格。如果遇到非英文字母且非空格的字符，
/// 则返回错误。
///
/// # 参数
/// * `s` - 要统计的字符串引用。
///
/// # 返回值
/// 成功时返回一个HashMap，键是字符，值是该字符出现的次数；
/// 如果字符串中包含非法字符，则返回`IsogramError::NonAlphabeticCharacter`错误。
fn counter_letter(s: &str) -> Result<HashMap<char, usize>, IsogramError> {
    let mut letter_counts = HashMap::new();
    for ch in s.to_ascii_lowercase().chars() {
        // 检查是否为非法字符（非字母且非空格）
        if !ch.is_ascii_alphabetic() && !ch.is_whitespace() {
            return Err(IsogramError::NonAlphabeticCharacter);
        }
        // 只统计英文字母
        if ch.is_ascii_alphabetic() {
            *letter_counts.entry(ch).or_insert(0) += 1;
        }
    }
    Ok(letter_counts)
}

/// 判断一个字符串是否是isogram（异构词）。
///
/// isogram是指一个单词或短语中没有重复字母（不区分大小写，忽略空格）。
/// 如果字符串中包含非英文字母字符（除了空格），则返回错误。
///
/// # 参数
/// * `s` - 待检测的字符串引用。
///
/// # 返回值
/// 如果是isogram返回Ok(true)，否则返回Ok(false)；
/// 如果包含非法字符，则返回Err(IsogramError::NonAlphabeticCharacter)。
pub fn is_isogram(s: &str) -> Result<bool, IsogramError> {
    let letter_counts = counter_letter(s)?;
    // 检查所有字母的出现次数是否都为1
    Ok(letter_counts.values().all(|&count| count == 1))
}


#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! isogram_tests {
        ($($name:ident: $tc:expr,)*) => {
            $(
                #[test]
                fn $name() {
                    let (input, expected) = $tc;
                    assert_eq!(is_isogram(input), expected);
                }
            )*
        };
    }

    isogram_tests! {
        isogram_simple: ("isogram", Ok(true)),
        isogram_case_insensitive: ("Isogram", Ok(true)),
        isogram_with_spaces: ("a b c d e", Ok(true)),
        isogram_mixed: ("Dermatoglyphics", Ok(true)),
        isogram_long: ("Subdermatoglyphic", Ok(true)),
        isogram_german_city: ("Malitzschkendorf", Ok(true)),
        perfect_pangram: ("Cwm fjord bank glyphs vext quiz", Ok(true)),
        isogram_sentences: ("The big dwarf only jumps", Ok(true)),
        isogram_french: ("Lampez un fort whisky", Ok(true)),
        isogram_portuguese: ("Velho traduz sim", Ok(true)),
        isogram_spanis: ("Centrifugadlos", Ok(true)),
        invalid_isogram_with_repeated_char: ("hello", Ok(false)),
        invalid_isogram_with_numbers: ("abc123", Err(IsogramError::NonAlphabeticCharacter)),
        invalid_isogram_with_special_char: ("abc!", Err(IsogramError::NonAlphabeticCharacter)),
        invalid_isogram_with_comma: ("Velho, traduz sim", Err(IsogramError::NonAlphabeticCharacter)),
        invalid_isogram_with_spaces: ("a b c d a", Ok(false)),
        invalid_isogram_with_repeated_phrase: ("abcabc", Ok(false)),
        isogram_empty_string: ("", Ok(true)),
        isogram_single_character: ("a", Ok(true)),
        invalid_isogram_multiple_same_characters: ("aaaa", Ok(false)),
        invalid_isogram_with_symbols: ("abc@#$%", Err(IsogramError::NonAlphabeticCharacter)),
    }
}
