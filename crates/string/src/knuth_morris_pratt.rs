/// 使用 Knuth-Morris-Pratt (KMP) 算法在文本中查找所有匹配的子串位置。
///
/// # 参数
/// * `text`: 要搜索的目标文本字符串。
/// * `pattern`: 要查找的模式字符串。
///
/// # 返回值
/// 返回一个包含所有匹配起始索引的向量。如果没有匹配项，则返回空向量。
#[allow(unused)]
pub fn knuth_morris_pratt(text: &str, pattern: &str) -> Vec<usize> {
    if text.is_empty() || pattern.is_empty() {
        return vec![];
    }
    let text_chars = text.chars().collect::<Vec<char>>();
    let pattern_chars = pattern.chars().collect::<Vec<char>>();
    let partial_match_table = build_partial_match_table(&pattern_chars);
    find_pattern(&text_chars, &pattern_chars, &partial_match_table)
}

/// 构建部分匹配表（也称为“失配函数”或“next数组”），用于 KMP 算法优化。
///
/// 该表记录了模式串中每个位置的最大公共前后缀长度，用于在匹配失败时决定模式串应跳过的字符数。
///
/// # 参数
/// * `pattern_chars`: 模式串的字符切片。
///
/// # 返回值
/// 返回一个与模式串等长的部分匹配表。
fn build_partial_match_table(pattern_chars: &[char]) -> Vec<usize> {
    let mut partial_match_table = vec![0];
    // 遍历模式串中的每个字符，构建部分匹配表
    pattern_chars
        .iter()
        .enumerate()
        .skip(1)
        .for_each(|(idx, &ch)| {
            let mut length = partial_match_table[idx - 1];
            // 当前字符不匹配时，回退到上一个可能的匹配位置
            while length > 0 && pattern_chars[length] != ch {
                length = partial_match_table[length - 1];
            }
            // 如果当前字符匹配，则增加匹配长度；否则设为0
            partial_match_table.push(if pattern_chars[length] == ch {
                length + 1
            } else {
                0
            })
        });
    partial_match_table
}

/// 在文本中查找所有匹配模式串的位置。
///
/// 利用部分匹配表来避免不必要的字符比较，提高匹配效率。
///
/// # 参数
/// * `text_chars`: 文本字符串的字符切片。
/// * `pattern_chars`: 模式串的字符切片。
/// * `partial_match_table`: 已构建的部分匹配表。
///
/// # 返回值
/// 返回一个包含所有匹配起始索引的向量。
fn find_pattern(
    text_chars: &[char],
    pattern_chars: &[char],
    partial_match_table: &[usize],
) -> Vec<usize> {
    let mut result_indices = vec![];
    let mut match_length = 0;
    // 遍历文本中的每个字符进行匹配
    text_chars
        .iter()
        .enumerate()
        .for_each(|(idx, &ch)| {
            // 匹配失败时，根据部分匹配表调整匹配长度
            while match_length > 0 && ch != pattern_chars[match_length] {
                match_length = partial_match_table[match_length - 1];
            }
            // 当前字符匹配成功，增加匹配长度
            if ch == pattern_chars[match_length] {
                match_length += 1;
            }
            // 完全匹配成功，记录匹配位置，并继续查找下一个可能的匹配
            if match_length == pattern_chars.len() {
                result_indices.push(idx + 1 - match_length);
                match_length = partial_match_table[match_length - 1];
            }
        });
    result_indices
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! test_knuth_morris_pratt {
        ($($name:ident: $inputs:expr,)*) => {
            $(
                #[test]
                fn $name() {
                    let (input, pattern, expected) = $inputs;
                    assert_eq!(knuth_morris_pratt(input, pattern), expected);
                }
            )*
        }
    }

    test_knuth_morris_pratt! {
        each_letter_matches: ("aaa", "a", vec![0, 1, 2]),
        a_few_seperate_matches: ("abababa", "ab", vec![0, 2, 4]),
        unicode: ("അഅഅ", "അ", vec![0, 1, 2]),
        unicode_no_match_but_similar_bytes: (
            &String::from_utf8(vec![224, 180, 133]).unwrap(),
            &String::from_utf8(vec![224, 180, 132]).unwrap(),
            vec![]
        ),
        one_match: ("ABC ABCDAB ABCDABCDABDE",  "ABCDABD", vec![15]),
        lots_of_matches: ("aaabaabaaaaa",  "aa", vec![0, 1, 4, 7, 8, 9, 10]),
        lots_of_intricate_matches: ("ababababa", "aba", vec![0, 2, 4, 6]),
        not_found0: ("abcde", "f", vec![]),
        not_found1: ("abcde", "ac", vec![]),
        not_found2: ("ababab", "bababa", vec![]),
        empty_string: ("", "abcdef", vec![]),
        empty_pattern: ("abcdef", "", vec![]),
        single_character_string: ("a", "a", vec![0]),
        single_character_pattern: ("abcdef", "d", vec![3]),
        pattern_at_start: ("abcdef", "abc", vec![0]),
        pattern_at_end: ("abcdef", "def", vec![3]),
        pattern_in_middle: ("abcdef", "cd", vec![2]),
        no_match_with_repeated_characters: ("aaaaaa", "b", vec![]),
        pattern_longer_than_string: ("abc", "abcd", vec![]),
        very_long_string: (&"a".repeat(10000), "a", (0..10000).collect::<Vec<usize>>()),
        very_long_pattern: (&"a".repeat(10000), &"a".repeat(9999), (0..2).collect::<Vec<usize>>()),
    }
}
