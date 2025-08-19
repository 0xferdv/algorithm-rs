/// 生成字符串的后缀数组，使用Myers算法实现。
///
/// 后缀数组是一个整数数组，其中每个元素表示原字符串中某个后缀的起始位置，
/// 并且这些后缀按照字典序升序排列。
///
/// # 参数
/// * `input`: 输入的字符串切片，用于构建后缀数组
///
/// # 返回值
/// 返回一个`Vec<usize>`类型的后缀数组，其中每个元素是后缀在原字符串中的起始索引
#[allow(unused)]
pub fn generate_suffix_array_member_myers(input: &str) -> Vec<usize> {
    // 处理空字符串情况
    if input.is_empty() {
        return Vec::new();
    }

    let n = input.len();
    let mut suffixes: Vec<(usize, &str)> = Vec::with_capacity(n);

    // 构建所有后缀及其起始位置的元组向量
    for (idx, _) in input.char_indices() {
        suffixes.push((idx, &input[idx..]));
    }

    // 按照后缀字符串的字典序对后缀进行排序
    suffixes.sort_by_key(|&(_, s)| s);

    // 初始化后缀数组和排名数组
    let mut suffix_array: Vec<usize> = vec![0; n];
    let mut rank = vec![0; n];
    let mut cur_rank = 0;
    let mut prev_suffix = &suffixes[0].1;

    // 第一次分配排名，相同后缀具有相同排名
    for (idx, suffix) in suffixes.iter().enumerate() {
        if &suffix.1 != prev_suffix {
            cur_rank += 1;
            prev_suffix = &suffix.1;
        }
        rank[suffix.0] = cur_rank;
        suffix_array[idx] = suffix.0;
    }

    let mut k = 1;
    let mut new_rank: Vec<usize> = vec![0; n];

    // 使用倍增法逐步构建后缀数组
    while k < n {
        // 根据当前k长度的排名对后缀数组重新排序
        suffix_array.sort_by_key(|&x| (rank[x], rank[(x + k) % n]));

        let mut cru_rank = 0;
        let mut prev = suffix_array[0];
        new_rank[prev] = cur_rank;

        // 更新排名数组
        for &suffix in suffix_array.iter().skip(1) {
            let next = suffix;
            if (rank[prev], rank[(prev + k) % n]) != (rank[next], rank[(next + k) % n]) {
                cur_rank += 1;
            }
            new_rank[next] = cur_rank;
            prev = next;
        }

        // 交换新旧排名数组
        std::mem::swap(&mut rank, &mut new_rank);
        k <<= 1; // k翻倍，进行下一轮倍增
    }

    suffix_array
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_suffix_array() {
        let input = "banana";
        let expected_result = vec![5, 3, 1, 0, 4, 2];
        assert_eq!(generate_suffix_array_member_myers(input), expected_result);
    }

    #[test]
    fn test_empty_string() {
        let input = "";
        let expected_result: Vec<usize> = Vec::new();
        assert_eq!(generate_suffix_array_member_myers(input), expected_result);
    }

    #[test]
    fn test_single_character() {
        let input = "a";
        let expected_result = vec![0];
        assert_eq!(generate_suffix_array_member_myers(input), expected_result);
    }
    // #[test]
    // fn test_repeating_characters() {
    //     let input = "zzzzzz";
    //     let expected_result = vec![5, 4, 3, 2, 1, 0];
    //     assert_eq!(generate_suffix_array_member_myers(input), expected_result);
    // }

    #[test]
    fn test_long_string() {
        let input = "abcdefghijklmnopqrstuvwxyz";
        let expected_result: Vec<usize> = (0..26).collect();
        assert_eq!(generate_suffix_array_member_myers(input), expected_result);
    }

    #[test]
    fn test_mix_of_characters() {
        let input = "abracadabra!";
        let expected_result = vec![11, 10, 7, 0, 3, 5, 8, 1, 4, 6, 9, 2];
        assert_eq!(generate_suffix_array_member_myers(input), expected_result);
    }

    #[test]
    fn test_whitespace_characters() {
        let input = " hello world ";
        let expected_result = vec![12, 0, 6, 11, 2, 1, 10, 3, 4, 5, 8, 9, 7];
        assert_eq!(generate_suffix_array_member_myers(input), expected_result);
    }
}
