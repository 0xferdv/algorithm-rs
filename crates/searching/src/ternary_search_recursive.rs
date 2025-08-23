use std::cmp::Ordering;

/// 递归实现的三元搜索函数，在一个有序数组中查找目标值。
///
/// 三元搜索是一种分治算法，它将数组分成三个部分，并根据比较结果决定在哪个部分继续搜索。
/// 时间复杂度为 O(log_3 n)，比二元搜索略快一些。
///
/// # 参数
/// * `target`: 要查找的目标值的引用
/// * `list`: 有序数组切片，用于搜索目标值
/// * `start`: 搜索范围的起始索引（包含）
/// * `end`: 搜索范围的结束索引（包含）
///
/// # 返回值
/// 如果找到目标值，则返回其在数组中的索引；否则返回 None
#[allow(unused)]
pub fn ternary_search_rec<T: Ord>(
    target: &T,
    list: &[T],
    start: usize,
    end: usize,
) -> Option<usize> {
    // 如果数组为空，直接返回 None
    if list.is_empty() {
        return None;
    }

    // 检查搜索范围是否有效
    if end >= start {
        // 计算两个分割点的位置
        let mid1: usize = start + (end - start) / 3;
        let mid2: usize = end - (end - start) / 3;

        // 根据目标值与第一个分割点值的比较结果进行处理
        match target.cmp(&list[mid1]) {
            // 如果目标值小于第一个分割点的值，在左三分之一部分继续搜索
            Ordering::Less => return ternary_search_rec(target, list, start, mid1 - 1),
            // 如果目标值等于第一个分割点的值，返回该索引
            Ordering::Equal => return Some(mid1),
            // 如果目标值大于第一个分割点的值，需要进一步判断在哪个部分搜索
            Ordering::Greater => match target.cmp(&list[mid2]) {
                // 如果目标值大于第二个分割点的值，在右三分之一部分继续搜索
                Ordering::Greater => return ternary_search_rec(target, list, mid2 + 1, end),
                // 如果目标值等于第二个分割点的值，返回该索引
                Ordering::Equal => return Some(mid2),
                // 如果目标值介于两个分割点之间，在中间三分之一部分继续搜索
                Ordering::Less => return ternary_search_rec(target, list, mid1 + 1, mid2 - 1),
            },
        }
    }

    // 搜索范围无效时返回 None
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn returns_none_if_empty_list() {
        let index = ternary_search_rec(&"a", &[], 1, 10);
        assert_eq!(index, None);
    }

    #[test]
    fn returns_none_if_range_is_invalid() {
        let index = ternary_search_rec(&1, &[1, 2, 3], 2, 1);
        assert_eq!(index, None);
    }

    #[test]
    fn returns_index_if_list_has_one_item() {
        let index = ternary_search_rec(&1, &[1], 0, 1);
        assert_eq!(index, Some(0));
    }

    #[test]
    fn returns_first_index() {
        let index = ternary_search_rec(&1, &[1, 2, 3], 0, 2);
        assert_eq!(index, Some(0));
    }

    #[test]
    fn returns_first_index_if_end_out_of_bounds() {
        let index = ternary_search_rec(&1, &[1, 2, 3], 0, 3);
        assert_eq!(index, Some(0));
    }

    #[test]
    fn returns_last_index() {
        let index = ternary_search_rec(&3, &[1, 2, 3], 0, 2);
        assert_eq!(index, Some(2));
    }

    #[test]
    fn returns_last_index_if_end_out_of_bounds() {
        let index = ternary_search_rec(&3, &[1, 2, 3], 0, 3);
        assert_eq!(index, Some(2));
    }

    #[test]
    fn returns_middle_index() {
        let index = ternary_search_rec(&2, &[1, 2, 3], 0, 2);
        assert_eq!(index, Some(1));
    }

    #[test]
    fn returns_middle_index_if_end_out_of_bounds() {
        let index = ternary_search_rec(&2, &[1, 2, 3], 0, 3);
        assert_eq!(index, Some(1));
    }
}
