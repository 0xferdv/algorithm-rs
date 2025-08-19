use std::cmp::Ordering;

/// 后缀结构体，用于构建后缀数组
///
/// # 字段说明
/// * `index` - 后缀在原字符串中的起始位置
/// * `rank` - 用于排序的元组，包含当前排名和下一个字符的排名
#[derive(Clone)]
struct Suffix {
    index: usize,
    rank: (i32, i32),
}

impl Suffix {

    /// 比较两个后缀的大小
    ///
    /// # 参数
    /// * `b` - 要比较的另一个后缀
    ///
    /// # 返回值
    /// 返回两个后缀的比较结果 Ordering
    fn cmp(&self, b: &Self) -> Ordering {
        let a = self;
        let ((a1, a2), (b1, b2)) = (a.rank, b.rank);
        match a1.cmp(&b1) {
            Ordering::Equal => {
                if a2 < b2 {
                    Ordering::Less
                } else {
                    Ordering::Greater
                }
            }
            o => o,
        }
    }
}

/// 生成字符串的后缀数组
///
/// 使用倍增算法构建后缀数组，时间复杂度 O(n log n)
///
/// # 参数
/// * `txt` - 输入的字符串切片
///
/// # 返回值
/// 返回后缀数组，其中每个元素表示按字典序排序后后缀在原字符串中的起始位置
#[allow(unused)]
pub fn generate_suffix_array(txt: &str) -> Vec<usize> {
    let n = txt.len();
    let mut suffixes: Vec<Suffix> = vec![
        Suffix {
            index: 0,
            rank: (-1, -1)
        };
        n
    ];

    // 初始化后缀数组，设置每个后缀的起始位置和前两个字符的排名
    for (idx, suf) in suffixes.iter_mut().enumerate() {
        suf.index = idx;
        suf.rank.0 = (txt.chars().nth(idx).expect("this should exist") as u32 - 'a' as u32) as i32;
        suf.rank.1 = if (idx + 1) < n {
            (txt.chars().nth(idx + 1).expect("this should exist") as u32 - 'a' as u32) as i32
        } else {
            -1
        }
    }

    // 根据初始排名对后缀进行排序
    suffixes.sort_by(|a, b| a.cmp(b));
    let mut ind = vec![0; n];
    let mut k = 4;

    // 倍增过程：逐步增加比较的字符长度
    while k < 2 * n {
        let mut rank = 0;
        let mut prev_rank = suffixes[0].rank.0;

        // 更新排名
        for i in 1..n {
            if suffixes[i].rank.0 == prev_rank && suffixes[i].rank.1 == suffixes[i - 1].rank.1 {
                prev_rank = suffixes[i].rank.0;
                suffixes[i].rank.0 = rank;
            } else {
                prev_rank = suffixes[i].rank.0;
                rank += 1;
                suffixes[i].rank.0 = rank;
            }
            ind[suffixes[i].index] = i;
        }

        // 设置下一个块的排名
        for i in 0..n {
            let next_idx = suffixes[i].index + (k / 2);
            suffixes[i].rank.1 = if next_idx < n {
                suffixes[ind[next_idx]].rank.0
            } else {
                -1
            }
        }

        // 重新排序
        suffixes.sort_by(|a, b| a.cmp(b));
        k *= 2;
    }

    // 构建最终的后缀数组
    let mut suffix_arr = Vec::new();
    for suf in suffixes {
        suffix_arr.push(suf.index);
    }
    suffix_arr
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_suffix_array() {
        let a = generate_suffix_array("banana");
        assert_eq!(a, vec![5, 3, 1, 0, 4, 2]);
    }
}

