/// 返回通过在字符串前面添加字符所能得到的最短回文串。
///
/// 该函数使用 KMP 算法的思想，通过计算原字符串与其反转字符串之间的最长前缀后缀匹配，
/// 来确定需要在前面添加多少字符才能构成回文。
///
/// # 参数
/// * `s` - 输入的字符串切片
///
/// # 返回值
/// 返回构造出的最短回文串
#[allow(unused)]
pub fn shortest_palindrome(s: &str) -> String {
    // 如果输入为空，则直接返回空字符串
    if s.is_empty() {
        return "".to_string();
    }

    // 将原始字符串转换为字符向量
    let original_chars: Vec<char> = s.chars().collect();

    // 计算原始字符串的后缀数组（用于 KMP 匹配优化）
    let suffix_table = compute_suffix(&original_chars);

    // 构造原始字符串的反转版本
    let mut reversed_chars: Vec<char> = s.chars().rev().collect();

    // 计算原始字符串与反转字符串的前缀匹配表
    let prefix_match = compute_prefix_match(&original_chars, &reversed_chars, &suffix_table);

    // 根据匹配结果，将原字符串中未匹配的部分追加到反转字符串末尾
    reversed_chars.append(&mut original_chars[prefix_match[original_chars.len() - 1]..].to_vec());

    // 将结果字符向量转换为字符串并返回
    reversed_chars.into_iter().collect()
}

/// 使用 KMP 算法预处理字符串，生成每个位置对应的最长相等前后缀长度。
///
/// # 参数
/// * `chars`: 字符数组切片
///
/// # 返回值
/// 返回一个与输入等长的 usize 向量，表示每个位置的最长相等前后缀长度
#[allow(unused)]
pub fn compute_suffix(chars: &[char]) -> Vec<usize> {
    let mut suffix = vec![0; chars.len()];
    (1..chars.len()).for_each(|idx| {
        let mut next_idx = suffix[idx - 1];
        while next_idx > 0 && chars[idx] != chars[next_idx] {
            next_idx = suffix[next_idx - 1];
        }
        suffix[idx] = next_idx + (chars[next_idx] == chars[idx]) as usize;
    });
    suffix
}

/// 计算两个字符串之间的前缀匹配长度表。
///
/// 该函数模拟 KMP 算法中的匹配过程，用来找出反转字符串与原字符串的最大重叠部分。
///
/// # 参数
/// * `original`: 原始字符串的字符数组
/// * `reversed`: 反转后的字符串的字符数组
/// * `suffix`: 原始字符串的后缀表（用于快速跳转）
///
/// # 返回值
/// 返回一个匹配表，表示每一位上的最大匹配长度
#[allow(unused)]
pub fn compute_prefix_match(original: &[char], reversed: &[char], suffix: &[usize]) -> Vec<usize> {
    let mut match_table = vec![0; original.len()];
    match_table[0] = usize::from(original[0] == reversed[0]);
    (1..original.len()).for_each(|idx| {
        let mut next_idx = match_table[idx - 1];
        while next_idx > 0 && reversed[idx] != original[next_idx] {
            next_idx = suffix[next_idx - 1];
        }
        match_table[idx] = next_idx + usize::from(reversed[idx] == original[next_idx]);
    });
    match_table
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::palindrome;

    macro_rules! test_shortest_palindrome {
        ($($name:ident: $inputs:expr,)*) => {
            $(
                #[test]
                fn $name() {
                    let (input, expected) = $inputs;
                    assert!(palindrome(expected));
                    assert_eq!(shortest_palindrome(input), expected);
                    assert_eq!(shortest_palindrome(expected), expected);
                }
            )*
        }
    }

    test_shortest_palindrome! {
        empty: ("", ""),
        extend_left_1: ("aacecaaa", "aaacecaaa"),
        extend_left_2: ("abcd", "dcbabcd"),
        unicode_1: ("അ", "അ"),
        unicode_2: ("a牛", "牛a牛"),
        single_char: ("x", "x"),
        already_palindrome: ("racecar", "racecar"),
        extend_left_3: ("abcde", "edcbabcde"),
        extend_left_4: ("abca", "acbabca"),
        long_string: ("abcdefg", "gfedcbabcdefg"),
        repetitive: ("aaaaa", "aaaaa"),
        complex: ("abacdfgdcaba", "abacdgfdcabacdfgdcaba"),
    }
}
