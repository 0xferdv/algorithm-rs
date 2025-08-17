use std::collections::HashMap;


#[derive(Debug, PartialEq)]
pub enum AnagramError{
    NonAlphabeticCharacter,
}

/// 检查两个字符串是否是字母异位词（anagram）。
///
/// 该函数会忽略空格，并将所有字母转换为小写进行比较。
/// 如果字符串中包含非字母字符，则返回错误。
///
/// # 参数
/// * `s` - 第一个待比较的字符串
/// * `t` - 第二个待比较的字符串
///
/// # 返回值
/// * `Ok(true)`: 如果两个字符串是异位词
/// * `Ok(false)`: 如果两个字符串不是异位词
/// * `Err(AnagramError::NonAlphabeticCharacter)`: 如果字符串中包含非字母字符
#[allow(unused)]
pub fn check_anagram(s: &str, t: &str) -> Result<bool, AnagramError> {
    let s_cleaned = clean_string(s)?;
    let t_cleaned = clean_string(t)?;
    Ok(char_count(&s_cleaned) == char_count(&t_cleaned))
}


/// 清理字符串：移除空格，转换为小写，并检查是否只包含字母。
///
/// # 参数
/// * `s` - 需要清理的字符串
///
/// # 返回值
/// * `Ok(String)`: 清理后的字符串（只有小写字母）
/// * `Err(AnagramError::NonAlphabeticCharacter)`: 如果字符串中包含非字母字符
fn clean_string(s: &str) -> Result<String, AnagramError> {
    s.chars()
        .filter(|c| !c.is_whitespace())
        .map(|c| {
            if c.is_alphabetic() {
                Ok(c.to_ascii_lowercase())
            } else {
                Err(AnagramError::NonAlphabeticCharacter)
            }
        }).collect()
}


/// 统计字符串中每个字符出现的次数。
///
/// # 参数
/// * `s` - 要统计的字符串
///
/// # 返回值
/// 返回一个HashMap，键为字符，值为该字符出现的次数
fn char_count(s: &str) -> HashMap<char, usize> {
    let mut res = HashMap::new();
    // 遍历每个字符并更新计数
    s.chars().for_each(|c| {
        *res.entry(c).or_insert(0) += 1;
    });
    res
}


#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! test_cases {
        ($($name:ident: $test_case:expr,)*) => {
            $(
                #[test]
                fn $name() {
                    let (s, t, expected) = $test_case;
                    assert_eq!(check_anagram(s, t), expected);
                    assert_eq!(check_anagram(t, s), expected);
                }
            )*
        }
    }

    test_cases! {
        empty_strings: ("", "", Ok(true)),
        empty_and_non_empty: ("", "Ted Morgan", Ok(false)),
        single_char_same: ("z", "Z", Ok(true)),
        single_char_diff: ("g", "h", Ok(false)),
        valid_anagram_lowercase: ("cheater", "teacher", Ok(true)),
        valid_anagram_with_spaces: ("madam curie", "radium came", Ok(true)),
        valid_anagram_mixed_cases: ("Satan", "Santa", Ok(true)),
        valid_anagram_with_spaces_and_mixed_cases: ("Anna Madrigal", "A man and a girl", Ok(true)),
        new_york_times: ("New York Times", "monkeys write", Ok(true)),
        church_of_scientology: ("Church of Scientology", "rich chosen goofy cult", Ok(true)),
        mcdonalds_restaurants: ("McDonald's restaurants", "Uncle Sam's standard rot", Err(AnagramError::NonAlphabeticCharacter)),
        coronavirus: ("coronavirus", "carnivorous", Ok(true)),
        synonym_evil: ("evil", "vile", Ok(true)),
        synonym_gentleman: ("a gentleman", "elegant man", Ok(true)),
        antigram: ("restful", "fluster", Ok(true)),
        sentences: ("William Shakespeare", "I am a weakish speller", Ok(true)),
        part_of_speech_adj_to_verb: ("silent", "listen", Ok(true)),
        anagrammatized: ("Anagrams", "Ars magna", Ok(true)),
        non_anagram: ("rat", "car", Ok(false)),
        invalid_anagram_with_special_char: ("hello!", "world", Err(AnagramError::NonAlphabeticCharacter)),
        invalid_anagram_with_numeric_chars: ("test123", "321test", Err(AnagramError::NonAlphabeticCharacter)),
        invalid_anagram_with_symbols: ("check@anagram", "check@nagaram", Err(AnagramError::NonAlphabeticCharacter)),
    }
}
