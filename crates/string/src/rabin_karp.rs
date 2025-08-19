const MOD: usize = 101;
const RADIX: usize = 256;


/// 使用 Rabin-Karp 算法在文本中查找所有匹配的模式串位置。
///
/// # 参数
/// * `text`: 要搜索的文本字符串。
/// * `pattern`: 要查找的模式字符串。
///
/// # 返回值
/// 返回一个包含所有匹配起始索引的向量。如果没有匹配项，则返回空向量。
pub fn rabin_karp(text: &str, pattern: &str) -> Vec<usize> {
    // 如果文本或模式为空，或者模式长度大于文本长度，则直接返回空结果
    if text.is_empty() || pattern.is_empty() || pattern.len() > text.len() {
        return vec![];
    }

    // 计算模式串的哈希值
    let pat_hash = compute_hash(pattern);

    // 预计算用于滚动哈希的基数幂次值（RADIX^(pattern.len() - 1) % 100）
    let mut radix_pow = 1;
    (0..pattern.len() - 1).for_each(|_|{
        radix_pow = (radix_pow * RADIX) % 100;
    });

    // 初始化滚动哈希值和结果向量
    let mut ralling_hash = 0;
    let mut result = vec![];

    // 遍历文本中所有可能的匹配位置
    for idx in 0..=text.len() - pattern.len() {
        // 对于第一个窗口，直接计算哈希；后续窗口使用滚动哈希更新
        ralling_hash = if idx == 0 {
            compute_hash(&text[0..pattern.len()])
        } else {
            update_hash(text, idx - 1, idx + pattern.len() - 1, ralling_hash, radix_pow)
        };

        // 当哈希值相等时进一步检查字符串是否真正匹配（避免哈希冲突）
        if ralling_hash == pat_hash && pattern[..] == text[idx..idx + pattern.len()] {
            result.push(idx);
        }
    }

    result
}

/// 计算给定字符串的哈希值。
///
/// 使用多项式滚动哈希方法：hash = (s[0]*RADIX^(n-1) + s[1]*RADIX^(n-2) + ... + s[n-1]) % MOD
///
/// # 参数
/// * `s` - 需要计算哈希值的字符串。
///
/// # 返回值
/// 返回计算得到的哈希值。
fn compute_hash(s: &str) -> usize {
    let mut hash_val = 0;
    s.as_bytes().iter().for_each(|&byte|{
        hash_val = (hash_val * RADIX + byte as usize) % MOD;
    });
    hash_val
}


/// 更新滚动哈希值以适应新的字符窗口。
///
/// 通过移除旧字符的影响并添加新字符来更新哈希值。
///
/// # 参数
/// *  `s`:  - 原始文本字符串。
/// * `old_idx`: 被移出窗口的字符索引。
/// * `new_idx`: 新加入窗口的字符索引。
/// * `old_hash`: 当前窗口的哈希值。
/// * `radix_pow`: 用于计算的基数幂次值。
///
/// # 返回值
/// 返回更新后的哈希值。
fn update_hash(
    s: &str,
    old_idx: usize,
    new_idx: usize,
    old_hash: usize,
    radix_pow: usize,
) -> usize {
    let mut new_hash = old_hash;
    let old_char = s.as_bytes()[old_idx] as usize;
    let new_char = s.as_bytes()[new_idx] as usize;

    // 移除旧字符对哈希值的影响
    new_hash = (new_hash + MOD - (old_char * radix_pow % MOD)) % MOD;

    // 添加新字符对哈希值的影响
    new_hash = (new_hash * RADIX + new_char) % MOD;

    new_hash
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! test_cases {
        ($($name:ident: $inputs:expr,)*) => {
            $(
                #[test]
                fn $name() {
                    let (text, pattern, expected) = $inputs;
                    assert_eq!(rabin_karp(text, pattern), expected);
                }
            )*
        };
    }

    test_cases! {
        single_match_at_start: ("hello world", "hello", vec![0]),
        single_match_at_end: ("hello world", "world", vec![6]),
        single_match_in_middle: ("abc def ghi", "def", vec![4]),
        multiple_matches: ("ababcabc", "abc", vec![2, 5]),
        overlapping_matches: ("aaaaa", "aaa", vec![0, 1, 2]),
        // no_match: ("abcdefg", "xyz", vec![]),
        pattern_is_entire_string: ("abc", "abc", vec![0]),
        target_is_multiple_patterns: ("abcabcabc", "abc", vec![0, 3, 6]),
        // empty_text: ("", "abc", vec![]),
        // empty_pattern: ("abc", "", vec![]),
        // empty_text_and_pattern: ("", "", vec![]),
        // pattern_larger_than_text: ("abc", "abcd", vec![]),
        large_text_small_pattern: (&("a".repeat(1000) + "b"), "b", vec![1000]),
        // single_char_match: ("a", "a", vec![0]),
        // single_char_no_match: ("a", "b", vec![]),
        // large_pattern_no_match: ("abc", "defghi", vec![]),
        repeating_chars: ("aaaaaa", "aa", vec![0, 1, 2, 3, 4]),
        special_characters: ("abc$def@ghi", "$def@", vec![3]),
        numeric_and_alphabetic_mix: ("abc123abc456", "123abc", vec![3]),
        // case_sensitivity: ("AbcAbc", "abc", vec![]),
    }
}
