use std::collections::HashMap;

/// 构建坏字符表，用于Boyer-Moore算法中的坏字符规则。
///
/// # 参数
/// * `pat` - 模式串的字符切片
///
/// # 返回值
/// 返回一个HashMap，键为字符，值为该字符在模式串中最后出现的位置索引
#[allow(unused)]
fn build_bad_char_table(pat: &[char]) -> HashMap<char, isize> {
    let mut bad_char_table = HashMap::new();
    pat.iter().enumerate().for_each(|(i, &ch)| {
        bad_char_table.insert(ch, i as isize);
    });
    bad_char_table
}

/// 计算匹配成功时的移动距离
///
/// 当模式串与文本完全匹配时，根据坏字符规则计算下一次移动的距离
///
/// # 参数
/// * `shift` - 当前模式串在文本中的起始位置
/// * `pat_len` - 模式串长度
/// * `text_len` - 文本长度
/// * `bat_char_table` - 坏字符表
/// * `text` - 文本字符数组
///
/// # 返回值
/// 返回模式串应该移动的距离
#[allow(unused)]
fn calc_match_shift(
    shift: isize,
    pat_len: isize,
    text_len: isize,
    bat_char_table: &HashMap<char, isize>,
    text: &[char],
) -> isize {
    // 如果模式串已经超出文本范围，则只移动1位
    if shift + pat_len >= text_len {
        return 1;
    }
    let next_ch = text[(shift + pat_len) as usize];
    pat_len - bat_char_table.get(&next_ch).unwrap_or(&-1)
}

/// 计算匹配失败时的移动距离
///
/// 当模式串与文本在某个位置不匹配时，根据坏字符规则计算下一次移动的距离
///
/// # 参数
/// * `mis_idx` - 不匹配位置在模式串中的索引
/// * `shift` - 当前模式串在文本中的起始位置
/// * `text` - 文本字符数组
/// * `bad_char_table` - 坏字符表
///
/// # 返回值
/// 返回模式串应该移动的距离
#[allow(unused)]
fn calc_mismatch_shift(
    mis_idx: isize,
    shift: isize,
    text: &[char],
    bad_char_table: &HashMap<char, isize>,
) -> isize {
    let mis_ch = text[(shift + mis_idx) as usize];
    let bad_char_shift = bad_char_table.get(&mis_ch).unwrap_or(&-1);
    std::cmp::max(1, mis_idx - bad_char_shift)
}

/// 使用Boyer-Moore算法在文本中搜索所有模式串出现的位置
///
/// # 参数
/// * `text` - 要搜索的文本
/// * `pat` - 要搜索的模式串
///
/// # 返回值
/// 返回一个包含所有匹配位置索引的向量
#[allow(unused)]
pub fn boyer_moore_search(text: &str, pat: &str) -> Vec<usize> {
    let mut positions = Vec::new();
    let text_len = text.len() as isize;
    let pat_len = pat.len() as isize;

    // 处理边界情况：空文本、空模式串或模式串长度大于文本长度
    if text_len == 0 || pat_len == 0  || pat_len > text_len {
        return positions;
    }

    // 将字符串转换为字符向量以便处理
    let pat: Vec<char> = pat.chars().collect();
    let text: Vec<char> = text.chars().collect();

    // 构建坏字符表
    let bad_char_table = build_bad_char_table(&pat);

    let mut shift = 0;

    // 主循环：在文本中移动模式串进行匹配
    while shift <= text_len - pat_len {
        // 从右到左比较字符
        let mut j = pat_len - 1;
        while j >= 0 && pat[j as usize] == text[(shift + j) as usize] {
            j -= 1;
        }

        // 如果完全匹配
        if j < 0 {
            positions.push(shift as usize);
            // 计算匹配后的移动距离
            shift += calc_match_shift(shift, pat_len, text_len, &bad_char_table, &text);
        } else {
            // 如果匹配失败，计算失败后的移动距离
            shift += calc_mismatch_shift(j, shift, &text, &bad_char_table);
        }
    }

    positions
}


#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! boyer_moore_tests {
        ($($name:ident: $tc:expr,)*) => {
            $(
                #[test]
                fn $name() {
                    let (text, pattern, expected) = $tc;
                    assert_eq!(boyer_moore_search(text, pattern), expected);
                }
            )*
        };
    }

    boyer_moore_tests! {
        test_simple_match: ("AABCAB12AFAABCABFFEGABCAB", "ABCAB", vec![1, 11, 20]),
        test_no_match: ("AABCAB12AFAABCABFFEGABCAB", "FFF", vec![]),
        test_partial_match: ("AABCAB12AFAABCABFFEGABCAB", "CAB", vec![3, 13, 22]),
        test_empty_text: ("", "A", vec![]),
        test_empty_pattern: ("ABC", "", vec![]),
        test_both_empty: ("", "", vec![]),
        test_pattern_longer_than_text: ("ABC", "ABCDEFG", vec![]),
        test_single_character_text: ("A", "A", vec![0]),
        test_single_character_pattern: ("AAAA", "A", vec![0, 1, 2, 3]),
        test_case_sensitivity: ("ABCabcABC", "abc", vec![3]),
        test_overlapping_patterns: ("AAAAA", "AAA", vec![0, 1, 2]),
        test_special_characters: ("@!#$$%^&*", "$$", vec![3]),
        test_numerical_pattern: ("123456789123456", "456", vec![3, 12]),
        test_partial_overlap_no_match: ("ABCD", "ABCDE", vec![]),
        test_single_occurrence: ("XXXXXXXXXXXXXXXXXXPATTERNXXXXXXXXXXXXXXXXXX", "PATTERN", vec![18]),
        test_single_occurrence_with_noise: ("PATPATPATPATTERNPAT", "PATTERN", vec![9]),
    }
}
