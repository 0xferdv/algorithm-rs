use std::cmp::min;

/// 使用 Marcher 算法查找字符串中最长回文子串。
///
/// # 参数
/// * `s` - 输入的字符串
///
/// # 返回值
/// 返回输入字符串中的最长回文子串
#[allow(unused)]
pub fn marcher(s: String) -> String {
    let len = s.len();
    // 如果字符串长度小于等于1，直接返回原字符串
    if len <= 1 {
        return s;
    }

    // 预处理字符串，在每个字符之间插入特殊字符 '#'，便于统一处理奇偶长度回文
    let mut chars: Vec<char> = Vec::with_capacity(s.len() * 2 + 1);
    for c in s.chars() {
        chars.push('#');
        chars.push(c);
    }
    chars.push('#');

    // 存储每个位置为中心的最长回文半径
    let mut length_of_palindrome = vec![1usize; chars.len()];
    // 当前回文中心和右边界
    let mut cur_center = 0;
    let mut right_from_cur_center: usize = 0;

    // 遍历处理后的字符数组，计算每个位置的最长回文半径
    for idx in 0..chars.len() {
        // 利用回文的对称性优化计算
        if right_from_cur_center > idx && idx > cur_center {
            length_of_palindrome[idx] = min(
                right_from_cur_center - idx,
                length_of_palindrome[cur_center * 2 - idx]
            );
            // 如果当前回文扩展超过了右边界，则更新中心和右边界
            if length_of_palindrome[idx] + idx >= right_from_cur_center {
                cur_center = idx;
                right_from_cur_center = idx + length_of_palindrome[idx];
                // 如果已经到达数组末尾，提前结束循环
                if right_from_cur_center >= chars.len() - 1 {
                    break;
                } else {
                    continue;
                }
            }
        }

        // 从当前已知半径开始，尝试扩展回文
        let mut radius:usize = (length_of_palindrome[idx] - 1) / 2;
        radius += 1;
        while idx >= radius && idx + radius <= chars.len() - 1 && chars[idx - radius] == chars[idx + radius] {
            length_of_palindrome[idx] += 2;
            radius += 1;
        }
    }

    // 找到具有最大回文半径的中心位置
    let center_of_max = length_of_palindrome
        .iter()
        .enumerate()
        .max_by_key(|(_, value)| **value)
        .map(|(idx, _)| idx)
        .unwrap();

    // 计算最长回文的实际半径，并提取对应的子串
    let radius_of_max = (length_of_palindrome[center_of_max] - 1) / 2;
    let answer = &chars[(center_of_max - radius_of_max)..=(center_of_max + radius_of_max)]
        .iter()
        .collect::<String>();

    // 移除预处理时添加的特殊字符 '#'，得到最终结果
    answer.replace('#', "")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_longest_palindrome_by_marcher() {
        assert_eq!(marcher("babad".to_string()), "aba".to_string());
        assert_eq!(marcher("cbbd".to_string()), "bb".to_string());
        assert_eq!(marcher("a".to_string()), "a".to_string());

        let ac_ans = marcher("ac".to_string());
        assert!(ac_ans == *"a" || ac_ans == *"c");
    }
}
