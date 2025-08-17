use std::cmp::{max, min};

/// 计算两个字符串之间的Jaro-Winkler距离
///
/// Jaro-Winkler距离是一种字符串相似度度量算法，它是Jaro距离的改进版本，
/// 特别适用于短字符串（如人名）的匹配。该算法考虑了字符匹配、转置和前缀相似性。
///
/// # 参数
/// * `s1` - 第一个字符串切片
/// * `s2` - 第二个字符串切片
///
/// # 返回值
/// 返回0.0到1.0之间的浮点数，表示两个字符串的相似度：
/// * 0.0 表示完全不相似
/// * 1.0 表示完全相同
///
/// # 算法说明
/// Jaro-Winkler距离计算分为以下几个步骤：
/// 1. 计算匹配字符：在一定范围内查找两个字符串中的匹配字符
/// 2. 计算转置：统计匹配字符中位置颠倒的数量
/// 3. 计算Jaro距离：基于匹配字符和转置数量计算基础相似度
/// 4. 计算前缀：统计两个字符串开头相同字符的数量
/// 5. 计算最终距离：结合Jaro距离和前缀长度得出最终结果
///
/// # 示例
///
/// let distance = jaro_winkler_distance("martha", "marhta");
/// assert_eq!(distance, 0.9611111111111111);
///
#[allow(unused)]
pub fn jaro_winkler_distance(s1: &str, s2: &str) -> f64 {
    // 处理空字符串的边界情况
    if s1.is_empty() || s2.is_empty() {
        return 0.0;
    }

    /// 获取两个字符串中的匹配字符
    ///
    /// 匹配字符定义为：在一定范围内的相同字符。范围由两个字符串长度的较小值决定。
    ///
    /// # 参数
    /// * `s1` - 第一个字符串
    /// * `s2` - 第二个字符串
    ///
    /// # 返回值
    /// 返回包含所有匹配字符的字符串
    fn get_matched_characters(s1: &str, s2: &str) -> String {
        // 创建s2的可变副本，用于标记已匹配的字符
        let mut s2 = s2.to_string();
        let mut matched: Vec<char> = Vec::new();
        // 计算匹配范围，通常为较短字符串长度的一半
        let limit = min(s1.len(), s2.len()) / 2;

        // 遍历s1中的每个字符
        for (idx, ch) in s1.chars().enumerate() {
            // 计算在s2中搜索的范围
            let left = max(0, idx as i32 - limit as i32) as usize;
            let right = min(idx + limit + 1, s2.len());

            // 如果在范围内找到匹配字符
            if s2[left..right].contains(ch) {
                matched.push(ch);
                // 标记已匹配的字符，防止重复匹配
                let a = &s2[0..s2.find(ch).expect("this exists")];
                let b = &s2[(s2.find(ch).expect("this exists") + 1)..];
                s2 = format!("{a} {b}");
            }
        }
        matched.iter().collect::<String>()
    }

    // 获取两个方向的匹配字符
    let matching_1 = get_matched_characters(s1, s2);
    let matching_2 = get_matched_characters(s2, s1);
    let match_count = matching_1.len();

    // 计算转置数量（位置颠倒的匹配字符对数）
    let transpositions = {
        let mut count = 0;
        for (c1, c2) in matching_1.chars().zip(matching_2.chars()) {
            if c1 != c2 {
                count += 1;
            }
        }
        count / 2  // 每对转置被计算了两次，所以除以2
    };

    // 计算Jaro距离
    let jaro: f64 = {
        // 如果没有匹配字符，相似度为0
        if match_count == 0 {
            return 0.0;
        }
        // Jaro距离公式：1/3 * (m/|s1| + m/|s2| + (m-t)/m)
        // 其中m是匹配字符数，t是转置数
        (1_f64 / 3_f64)
            * (match_count as f64 / s1.len() as f64
            + match_count as f64 / s2.len() as f64
            + (match_count - transpositions) as f64 / match_count as f64)
    };

    // 计算公共前缀长度（最多考虑前4个字符）
    let mut prefix_len = 0.0;
    let bound = min(min(s1.len(), s2.len()), 4);
    for (c1, c2) in s1[..bound].chars().zip(s2[..bound].chars()) {
        if c1 == c2 {
            prefix_len += 1.0;
        } else {
            break;
        }
    }

    // Jaro-Winkler距离公式：Jaro + (0.1 * prefix_length) * (1 - Jaro)
    // 其中0.1是缩放因子，通常设置为0.1
    jaro + (0.1 * prefix_len) * (1.0 - jaro)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_jaro_winkler_distance() {
        // 测试不相似字符串
        let a = jaro_winkler_distance("hello", "world");
        assert_eq!(a, 0.4666666666666666);

        // 测试相似字符串（仅有两个字符位置颠倒）
        let a = jaro_winkler_distance("martha", "marhta");
        assert_eq!(a, 0.9611111111111111);

        // 测试相似字符串（另一种字符位置颠倒）
        let a = jaro_winkler_distance("martha", "marhat");
        assert_eq!(a, 0.9611111111111111);

        // 测试完全相同的字符串
        let a = jaro_winkler_distance("test", "test");
        assert_eq!(a, 1.0);

        // 测试空字符串
        let a = jaro_winkler_distance("test", "");
        assert_eq!(a, 0.0);

        // 测试大小写和数字混合的字符串
        let a = jaro_winkler_distance("hello world", "HeLLo W0rlD");
        assert_eq!(a, 0.6363636363636364);
    }
}
