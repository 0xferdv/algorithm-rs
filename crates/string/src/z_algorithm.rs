/// 计算从指定起始位置开始的 Z 值。
///
/// 该函数用于计算 input_string 中从 start_index 开始，
/// 与 pattern 匹配的最长前缀长度（即 Z 值）。
///
/// # 参数
/// - `input_string`: 要搜索的主字符串切片。
/// - `pattern`: 模式字符串切片。
/// - `start_index`: 在 input_string 中开始匹配的位置。
/// - `z_value`: 初始的 Z 值，通常为 0。
///
/// # 返回值
/// 返回从 start_index 开始匹配到的最长前缀长度。
#[allow(unused)]
fn calculate_z_value<T: Eq>(
    input_string: &[T],
    pattern: &[T],
    start_index: usize,
    mut z_value: usize,
) -> usize {
    let size = input_string.len();
    let pattern_size = pattern.len();
    while (start_index + z_value) < size && z_value < pattern_size {
        if input_string[start_index + z_value] != pattern[z_value] {
            break;
        }
        z_value += 1;
    }
    z_value
}

/// 根据之前的匹配结果初始化当前索引的 Z 值。
///
/// 利用 Z 算法中的已有信息，避免重复计算以提高效率。
///
/// # 参数
/// - `z_array`: 已经计算的部分 Z 数组。
/// - `i`: 当前要处理的索引。
/// - `match_end`: 当前已知匹配区间的右边界。
/// - `last_match`: 上一次匹配开始的索引。
///
/// # 返回值
/// 返回可以复用的初始 Z 值。
#[allow(unused)]
fn initialize_z_array_from_previous_match(
    z_array: &[usize],
    i: usize,
    match_end: usize,
    last_match: usize,
) -> usize {
    std::cmp::min(z_array[i - last_match], match_end - i + 1)
}

/// 查找所有完全匹配的位置。
///
/// 遍历 Z 数组，找出所有等于模式串长度的 Z 值对应的索引。
///
/// # 参数
/// - `z_array`: 完整的 Z 数组。
/// - `pattern_size`: 模式串的长度。
///
/// # 返回值
/// 返回所有完全匹配的起始索引组成的向量。
#[allow(unused)]
fn find_full_matches(z_array: &[usize], pattern_size: usize) -> Vec<usize> {
    z_array
        .iter()
        .enumerate()
        .filter_map(|(idx, &z_value)| (z_value == pattern_size).then_some(idx))
        .collect()
}

/// 使用 Z 算法在输入字符串中查找模式串的所有匹配位置。
///
/// 支持返回所有匹配（包括部分匹配）或仅返回完全匹配。
///
/// # 参数
/// - `input_string`: 要搜索的主字符串切片。
/// - `pattern`: 模式字符串切片。
/// - `start_index`: 开始匹配的索引。
/// - `only_full_matches`: 是否只返回完全匹配的位置。
///
/// # 返回值
/// 如果 only_full_matches 为 true，则返回完全匹配的起始索引；
/// 否则返回完整的 Z 数组。
fn match_with_z_array<T: Eq>(
    input_string: &[T],
    pattern: &[T],
    start_index: usize,
    only_full_matches: bool,
) -> Vec<usize> {
    let size = input_string.len();
    let pattern_size = pattern.len();
    let mut last_match: usize = 0;
    let mut match_end: usize = 0;
    let mut z_array = vec![0; size];

    // 遍历字符串并构建 Z 数组
    (start_index..size).for_each(|idx| {
        // 如果当前索引在已有匹配范围内，尝试复用之前的匹配信息
        if idx <= match_end {
            z_array[idx] = initialize_z_array_from_previous_match(&z_array, idx, match_end, last_match);
        }

        // 计算当前位置的实际 Z 值
        z_array[idx] = calculate_z_value(input_string, pattern, idx, z_array[idx]);

        // 更新匹配范围和起始位置
        if idx + z_array[idx] > match_end + 1 {
            match_end = idx + z_array[idx] + 1;
            last_match = idx;
        }
    });

    // 根据参数决定返回完整 Z 数组还是仅完全匹配位置
    if !only_full_matches {
        z_array
    } else {
        find_full_matches(&z_array, pattern_size)
    }
}

/// 构造给定字符串的 Z 数组。
///
/// Z 数组中每个元素表示从对应位置开始的子串与原字符串前缀的最大匹配长度。
///
/// # 参数
/// - `input`: 输入字符串切片。
///
/// # 返回值
/// 返回构造好的 Z 数组。
#[allow(unused)]
pub fn z_array<T: Eq>(input: &[T]) -> Vec<usize> {
    match_with_z_array(input, input, 1, false)
}

/// 在输入字符串中查找所有与模式串完全匹配的位置。
///
/// # 参数
/// - `input`: 主字符串切片。
/// - `pattern`: 要查找的模式串切片。
///
/// # 返回值
/// 返回所有完全匹配的起始索引组成的向量。
#[allow(unused)]
pub fn match_pattern<T: Eq>(input: &[T], pattern: &[T]) -> Vec<usize> {
    match_with_z_array(input, pattern, 0, true)
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! test_match_pattern {
        ($($name:ident: ($input:expr, $pattern:expr, $expected:expr),)*) => {
            $(
                #[test]
                fn $name() {
                    let (input, pattern, expected) = ($input, $pattern, $expected);
                    assert_eq!(match_pattern(input.as_bytes(), pattern.as_bytes()), expected);
                }
            )*
        };
    }

    macro_rules! test_z_array_cases {
        ($($name:ident: ($input:expr, $expected:expr),)*) => {
            $(
                #[test]
                fn $name() {
                    let (input, expected) = ($input, $expected);
                    assert_eq!(z_array(input.as_bytes()), expected);
                }
            )*
        };
    }

    test_match_pattern! {
        simple_match: ("abcabcabc", "abc", vec![0, 3, 6]),
        no_match: ("abcdef", "xyz", vec![]),
        single_char_match: ("aaaaaa", "a", vec![0, 1, 2, 3, 4, 5]),
        overlapping_match: ("abababa", "aba", vec![0, 2, 4]),
        full_string_match: ("pattern", "pattern", vec![0]),
        empty_pattern: ("nonempty", " ", vec![]),
        pattern_larger_than_text: ("small", "largerpattern", vec![]),
        repeated_pattern_in_text: (
            "aaaaaaaa",
            "aaa",
            vec![0, 1, 2, 3, 4, 5]
        ),
        pattern_not_in_lipsum: (
            concat!(
                "lorem ipsum dolor sit amet, consectetur ",
                "adipiscing elit, sed do eiusmod tempor ",
                "incididunt ut labore et dolore magna aliqua"
            ),
            ";alksdjfoiwer",
            vec![]
        ),
        pattern_in_lipsum: (
            concat!(
                "lorem ipsum dolor sit amet, consectetur ",
                "adipiscing elit, sed do eiusmod tempor ",
                "incididunt ut labore et dolore magna aliqua"
            ),
            "m",
            vec![4, 10, 23, 68, 74, 110]
        ),
    }

    // test_z_array_cases! {
    //     basic_z_array: ("aabaabab", vec![0, 1, 0, 4, 1, 0, 1, 0]),
    //     empty_string: ("", vec![]),
    //     single_char_z_array: ("a", vec![0]),
    //     repeated_char_z_array: ("aaaaaa", vec![0, 5, 4, 3, 2, 1]),
    // }
}
