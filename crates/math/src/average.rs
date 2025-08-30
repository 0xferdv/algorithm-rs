use num_traits::Num;
use std::collections::{HashMap, HashSet};

/// 计算一个数值向量的总和。
///
/// # 参数
/// * `sequence` - 一个包含数值类型的向量
///
/// # 返回值
/// 返回向量中所有元素的总和
fn sum<T: Num + Copy>(sequence: Vec<T>) -> T {
    sequence
        .iter()
        .fold(T::zero(), |acc, &x| acc + x)
}

/// 计算一个数值向量的平均值。
///
/// # 参数
/// * `sequence` - 一个包含数值类型的向量
///
/// # 返回值
/// 如果向量不为空，返回平均值；否则返回None
#[allow(unused)]
pub fn mean<T: Num + Copy + num_traits::FromPrimitive>(sequence: Vec<T>) -> Option<T> {
    let len = sequence.len();
    if len == 0 {
        return None;
    }
    // 计算总和除以元素个数得到平均值
    Some(sum(sequence) / T::from_usize(len).unwrap())
}

/// 计算两个数值的平均值。
///
/// # 参数
/// * `a` - 第一个数值
/// * `b` - 第二个数值
///
/// # 返回值
/// 返回两个数值的平均值
fn mean_of_two<T: Num + Copy>(a: T, b: T) -> T {
    (a + b) / (T::one() + T::one())
}

/// 计算一个数值向量的中位数。
///
/// # 参数
/// * `sequence` - 一个包含数值类型的向量
///
/// # 返回值
/// 如果向量不为空，返回中位数；否则返回None
#[allow(unused)]
pub fn median<T: Num + Copy + PartialOrd>(mut sequence: Vec<T>) -> Option<T> {
    if sequence.is_empty() {
        return None;
    }
    // 对向量进行排序
    sequence.sort_by(|a, b| a.partial_cmp(b).unwrap());
    if sequence.len() % 2 == 1 {
        // 奇数个元素，返回中间元素
        let k = (sequence.len() + 1) / 2;
        Some(sequence[k - 1])
    } else {
        // 偶数个元素，返回中间两个元素的平均值
        let j = (sequence.len()) / 2;
        Some(mean_of_two(sequence[j - 1], sequence[j]))
    }
}

/// 计算一个向量中各元素出现的频次。
///
/// # 参数
/// * `sequence` - 一个包含可哈希元素的向量
///
/// # 返回值
/// 返回一个HashMap，键为元素值，值为该元素出现的次数
fn histogram<T: Eq + std::hash::Hash>(sequence: Vec<T>) -> HashMap<T, usize> {
    sequence
        .into_iter()
        .fold(HashMap::new(), |mut res, val| {
            *res.entry(val).or_insert(0) += 1;
            res
        })
}

/// 计算一个向量的众数（出现频次最高的元素）。
///
/// # 参数
/// * `sequence` - 一个包含可哈希元素的向量
///
/// # 返回值
/// 如果向量不为空，返回包含所有众数的HashSet；否则返回None
#[allow(unused)]
pub fn mode<T: Eq + std::hash::Hash>(sequence: Vec<T>) -> Option<HashSet<T>> {
    if sequence.is_empty() {
        return None;
    }
    // 计算元素频次直方图
    let hist = histogram(sequence);
    // 找到最大出现次数
    let max_count = *hist.values().max().unwrap();
    // 筛选出所有出现次数等于最大次数的元素
    Some(
        hist.into_iter()
            .filter(|(_, count)| *count == max_count)
            .map(|(value, _)| value)
            .collect(),
    )
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn median_test() {
        assert_eq!(median(vec![4, 53, 2, 1, 9, 0, 2, 3, 6]).unwrap(), 3);
        assert_eq!(median(vec![-9, -8, 0, 1, 2, 2, 3, 4, 6, 9, 53]).unwrap(), 2);
        assert_eq!(median(vec![2, 3]).unwrap(), 2);
        assert_eq!(median(vec![3.0, 2.0]).unwrap(), 2.5);
        assert_eq!(median(vec![1.0, 700.0, 5.0]).unwrap(), 5.0);
        assert!(median(Vec::<i32>::new()).is_none());
        assert!(median(Vec::<f64>::new()).is_none());
    }
    #[test]
    fn mode_test() {
        assert_eq!(
            mode(vec![4, 53, 2, 1, 9, 0, 2, 3, 6]).unwrap(),
            HashSet::from([2])
        );
        assert_eq!(
            mode(vec![-9, -8, 0, 1, 2, 2, 3, -1, -1, 9, -1, -9]).unwrap(),
            HashSet::from([-1])
        );
        assert_eq!(mode(vec!["a", "b", "a"]).unwrap(), HashSet::from(["a"]));
        assert_eq!(mode(vec![1, 2, 2, 1]).unwrap(), HashSet::from([1, 2]));
        assert_eq!(mode(vec![1, 2, 2, 1, 3]).unwrap(), HashSet::from([1, 2]));
        assert_eq!(mode(vec![1]).unwrap(), HashSet::from([1]));
        assert!(mode(Vec::<i32>::new()).is_none());
    }
    #[test]
    fn mean_test() {
        assert_eq!(mean(vec![2023.1112]).unwrap(), 2023.1112);
        assert_eq!(mean(vec![0.0, 1.0, 2.0, 3.0, 4.0]).unwrap(), 2.0);
        assert_eq!(
            mean(vec![-7.0, 4.0, 53.0, 2.0, 1.0, -9.0, 0.0, 2.0, 3.0, -6.0]).unwrap(),
            4.3
        );
        assert_eq!(mean(vec![1, 2]).unwrap(), 1);
        assert!(mean(Vec::<f64>::new()).is_none());
        assert!(mean(Vec::<i32>::new()).is_none());
    }
}
