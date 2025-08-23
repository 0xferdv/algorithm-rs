use std::cmp::{Ordering, PartialOrd};
use sorting::partition;

/// 查找切片中第k小的元素（k从1开始计数）
///
/// # 参数
/// * `input` - 待查找的可变切片，函数执行后切片内容可能被重新排列
/// * `k`     - 要查找的位置，从1开始计数
///
/// # 返回值
/// 如果切片为空则返回None，否则返回第k小的元素
///
/// # 泛型约束
/// T必须实现PartialOrd和Copy trait
#[allow(unused)]
pub fn kth_smallest<T>(input: &mut [T], k: usize) -> Option<T>
where
    T: PartialOrd + Copy
{
    if input.is_empty() {
        return None;
    }
    let kth = _kth_smallest(input, k, 0, input.len() - 1);
    Some(kth)
}

/// 使用快速选择算法递归查找第k小的元素
///
/// # 参数
/// * `input` - 待查找的可变切片
/// * `k`     - 要查找的位置，从1开始计数
/// * `lo`    - 搜索范围的起始索引
/// * `hi`    - 搜索范围的结束索引
///
/// # 返回值
/// 返回第k小的元素
///
/// # 算法思路
/// 通过partition函数将切片分为两部分，根据pivot的位置决定在左半部分还是右半部分继续查找
fn _kth_smallest<T>(input: &mut [T], k: usize, lo: usize, hi: usize) -> T
where
    T: PartialOrd + Copy
{
    // 基本情况：当搜索范围只有一个元素时，直接返回该元素
    if lo == hi {
        return input[lo];
    }

    // 对当前范围进行分区操作，返回pivot的位置
    let pivot = partition(input, lo, hi);

    // 计算pivot在当前搜索范围内的相对位置（从1开始）
    let idx = pivot - lo + 1;

    // 根据k与pivot位置的关系决定下一步搜索方向
    match k.cmp(&idx) {
        // 如果k等于pivot的位置，说明找到了第k小的元素
        Ordering::Equal => input[pivot],
        // 如果k大于pivot的位置，说明目标元素在右半部分
        Ordering::Greater => _kth_smallest(input, k - idx, pivot + 1, hi),
        // 如果k小于pivot的位置，说明目标元素在左半部分
        Ordering::Less => _kth_smallest(input, k, lo, pivot - 1),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty() {
        let mut zero: [u8; 0] = [];
        let first = kth_smallest(&mut zero, 1);

        assert_eq!(None, first);
    }

    #[test]
    fn one_element() {
        let mut one = [1];
        let first = kth_smallest(&mut one, 1);

        assert_eq!(1, first.unwrap());
    }

    #[test]
    fn many_elements() {
        // 0 1 3 4 5 7 8 9 9 10 12 13 16 17
        let mut many = [9, 17, 3, 16, 13, 10, 1, 5, 7, 12, 4, 8, 9, 0];

        let first = kth_smallest(&mut many, 1);
        let third = kth_smallest(&mut many, 3);
        let sixth = kth_smallest(&mut many, 6);
        let fourteenth = kth_smallest(&mut many, 14);

        assert_eq!(0, first.unwrap());
        assert_eq!(3, third.unwrap());
        assert_eq!(7, sixth.unwrap());
        assert_eq!(17, fourteenth.unwrap());
    }
}
