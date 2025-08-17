use std::cmp::min;


/// 计算两个字符串之间的编辑距离（Levenshtein Distance）。
///
/// 使用动态规划方法构建完整的距离矩阵，计算从字符串 s1 转换到字符串 s2 所需的最少单字符编辑操作次数。
/// 操作包括：插入、删除或替换一个字符。
///
/// # 参数
///
/// * `s1`: 第一个字符串。
/// * `s2`: 第二个字符串。
///
/// # 返回值
///
/// 返回两个字符串之间的 Levenshtein 距离（usize 类型）。
#[allow(unused)]
pub fn native_levenshtein_distance(s1: &str, s2: &str) -> usize {
    // 初始化距离矩阵，第一行和第一列初始化为递增序列
    let distance_matrix: Vec<Vec<usize>> = (0..=s1.len())
        .map(|i| {
            (0..=s2.len())
                .map(|j| {
                    if i == 0 {
                        j
                    } else if j == 0 {
                        i
                    } else {
                        0
                    }
                })
                .collect()
        }).collect();

    // 填充距离矩阵
    let updated_matrix = (1..=s1.len()).fold(distance_matrix, |mut matrix, i| {
        (1..=s2.len()).fold(matrix, |mut inner_matrix, j| {
            // 当前字符是否相同，决定替换成本
            let cost = usize::from(s1.as_bytes()[i - 1] != s2.as_bytes()[j - 1]);
            // 计算当前位置的最小编辑距离
            inner_matrix[i][j] = (inner_matrix[i - 1][j - 1] + cost)
                .min(inner_matrix[i][j - 1] + 1)
                .min(inner_matrix[i - 1][j] + 1);
            inner_matrix
        })
    });

    // 返回右下角的值，即最终的编辑距离
    updated_matrix[s1.len()][s2.len()]
}

/// 计算两个字符串之间的编辑距离（Levenshtein Distance），使用空间优化的版本。
///
/// 通过只保留前一行的状态来减少内存使用，适用于较长字符串的高效计算。
///
/// # 参数
///
/// * `s1`: 第一个字符串。
/// * `s2`: 第二个字符串。
///
/// # 返回值
///
/// 返回两个字符串之间的 Levenshtein 距离（usize 类型）。
#[allow(unused)]
pub fn optimized_levenshtein_distance(s1: &str, s2: &str) -> usize {
    // 处理空字符串的情况
    if s1.is_empty() {
        return s2.len();
    }

    let l1 = s1.len();
    // 初始化前一行的距离数组
    let mut prev_dist: Vec<usize> = (0..=l1).collect();

    // 遍历第二个字符串的每个字符
    for (row, c2) in s2.chars().enumerate() {
        let mut prev_substitution_cost = prev_dist[0];
        // 更新第一列的值
        prev_dist[0] = row + 1;

        // 遍历第一个字符串的每个字符
        for (col, c1) in s1.chars().enumerate() {
            // 计算三种操作的成本
            let deletion_cost = prev_dist[col] + 1;
            let insertion_cost = prev_dist[col + 1] + 1;
            let substitution_cost = if c1 == c2 {
                prev_substitution_cost
            } else {
                prev_substitution_cost + 1
            };

            // 更新状态
            prev_substitution_cost = prev_dist[col + 1];
            prev_dist[col + 1] = _min3(deletion_cost, insertion_cost, substitution_cost);
        }
    }

    // 返回最后一个元素，即最终的编辑距离
    prev_dist[l1]
}

/// 返回三个 usize 值中的最小值。
///
/// # 参数
///
/// * `a` - 第一个值。
/// * `b` - 第二个值。
/// * `c` - 第三个值。
///
/// # 返回值
///
/// 返回 a、b、c 中的最小值。
#[inline]
fn _min3(a: usize, b: usize, c: usize) -> usize {
    min(min(a, b), c)
}


#[cfg(test)]
mod tests {
    const LEVENSHTEIN_DISTANCE_TEST_CASES: &[(&str, &str, usize)] = &[
        ("", "", 0),
        ("Hello, World!", "Hello, World!", 0),
        ("", "Rust", 4),
        ("horse", "ros", 3),
        ("tan", "elephant", 6),
        ("execute", "intention", 8),
    ];

    macro_rules! levenshtein_distance_tests {
        ($function:ident) => {
            mod $function {
                use super::*;

                fn run_test_case(string1: &str, string2: &str, expected_distance: usize) {
                    assert_eq!(super::super::$function(string1, string2), expected_distance);
                    assert_eq!(super::super::$function(string2, string1), expected_distance);
                    assert_eq!(super::super::$function(string1, string1), 0);
                    assert_eq!(super::super::$function(string2, string2), 0);
                }

                #[test]
                fn test_levenshtein_distance() {
                    for &(string1, string2, expected_distance) in
                        LEVENSHTEIN_DISTANCE_TEST_CASES.iter()
                    {
                        run_test_case(string1, string2, expected_distance);
                    }
                }
            }
        };
    }

    levenshtein_distance_tests!(native_levenshtein_distance);
    levenshtein_distance_tests!(optimized_levenshtein_distance);
}
