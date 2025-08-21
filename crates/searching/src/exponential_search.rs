use std::cmp::Ordering;

/// 在已排序的数组中使用指数搜索（Exponential Search）查找指定元素。
///
/// 指数搜索是一种适用于无限或未知大小的已排序数组的搜索算法。它首先通过指数增长的方式
/// 找到目标元素可能存在的范围，然后在该范围内使用二分查找来定位元素。
///
/// # 参数
///
/// * `item`: 要查找的目标元素的引用。
/// * `arr`: 已排序的元素切片，用于搜索。
///
/// # 返回值
///
/// 如果找到目标元素，则返回其在数组中的索引（`Some(usize)`）；
/// 如果未找到，则返回 `None`。
///
/// # 示例
///
/// let arr = [1, 2, 3, 4, 5];
/// assert_eq!(exponential_search(&3, &arr), Some(2));
/// assert_eq!(exponential_search(&6, &arr), None);
///
#[allow(unused)]
pub fn exponential_search<T: Ord>(item: &T, arr: &[T]) -> Option<usize> {
    // 如果数组为空，直接返回 None
    if arr.is_empty() {
        return None
    }

    let len = arr.len();
    let mut upper = 1;

    // 指数增长地寻找上界，使得 arr[upper] >= item
    while (upper < len) && (&arr[upper] <= item) {
        upper *= 2;
    }

    // 防止 upper 超出数组长度
    if upper > len {
        upper = len;
    }

    let mut lower = upper / 2;

    // 在确定的范围内进行二分查找
    while lower < upper {
        let mid = lower + (upper - lower) / 2;
        match item.cmp(&arr[mid]) {
            Ordering::Less => upper = mid,
            Ordering::Greater => lower = mid + 1,
            Ordering::Equal => return Some(mid),
        }
    }

    // 未找到目标元素
    None
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty() {
        let index = exponential_search(&"a", &[]);
        assert_eq!(index, None);
    }

    #[test]
    fn one_item() {
        let index = exponential_search(&"a", &["a"]);
        assert_eq!(index, Some(0));
    }

    #[test]
    fn search_strings() {
        let index = exponential_search(&"a", &["a", "b", "c", "d", "google", "zoo"]);
        assert_eq!(index, Some(0));
    }

    #[test]
    fn search_ints() {
        let index = exponential_search(&4, &[1, 2, 3, 4]);
        assert_eq!(index, Some(3));

        let index = exponential_search(&3, &[1, 2, 3, 4]);
        assert_eq!(index, Some(2));

        let index = exponential_search(&2, &[1, 2, 3, 4]);
        assert_eq!(index, Some(1));

        let index = exponential_search(&1, &[1, 2, 3, 4]);
        assert_eq!(index, Some(0));
    }

    #[test]
    fn not_found() {
        let index = exponential_search(&5, &[1, 2, 3, 4]);
        assert_eq!(index, None);
    }
}
