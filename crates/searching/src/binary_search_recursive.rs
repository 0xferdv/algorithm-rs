use std::cmp::Ordering;


/// 使用递归方式在有序数组中查找指定元素。
///
/// 该函数支持升序和降序排列的数组，通过比较首尾元素判断排序方向，
/// 并使用二分法递归查找目标元素。
///
/// # 参数
///
/// - `item`: 要查找的目标元素引用。
/// - `arr`: 要搜索的切片，可以是升序或降序排列。
/// - `left`: 搜索范围的左边界（包含）。
/// - `right`: 搜索范围的右边界（不包含）。
///
/// # 返回值
///
/// 如果找到目标元素，则返回其索引；否则返回 `None`。
#[allow(unused)]
pub fn binary_search_recursive<T: Ord>(item: &T, arr: &[T], left: usize, right: usize) -> Option<usize> {
    // 基本情况：搜索区间无效时直接返回 None
    if left >= right {
        return None;
    }

    // 判断数组是否为升序排列
    let is_asc = arr.len() > 1 && arr[0] < arr[arr.len() - 1];

    // 计算中间位置，避免溢出
    let mid = left + (right - left) / 2;

    // 比较目标元素与中间元素的关系
    let cmp_result = item.cmp(&arr[mid]);

    // 根据排序方向和比较结果决定下一步搜索区间
    match (is_asc, cmp_result) {
        // 升序且目标小于中间元素，或降序且目标大于中间元素，则在左半部分继续查找
        (true, Ordering::Less) | (false, Ordering::Greater) => {
            binary_search_recursive(item, arr, left, mid)
        }
        // 升序且目标大于中间元素，或降序且目标小于中间元素，则在右半部分继续查找
        (true, Ordering::Greater) | (false, Ordering::Less) => {
            binary_search_recursive(item, arr, mid + 1, right)
        }
        // 目标等于中间元素，返回索引
        (_, Ordering::Equal) => Some(mid),
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! test_cases {
        ($($name:ident: $test_case:expr,)*) => {
            $(
                #[test]
                fn $name() {
                    let (item, arr, expected) = $test_case;
                    assert_eq!(binary_search_recursive(&item, arr, 0, arr.len()), expected);
                }
            )*
        };
    }

    test_cases! {
        empty: ("a", &[] as &[&str], None),
        one_item_found: ("a", &["a"], Some(0)),
        one_item_not_found: ("b", &["a"], None),
        search_strings_asc_start: ("a", &["a", "b", "c", "d", "google", "zoo"], Some(0)),
        search_strings_asc_middle: ("google", &["a", "b", "c", "d", "google", "zoo"], Some(4)),
        search_strings_asc_last: ("zoo", &["a", "b", "c", "d", "google", "zoo"], Some(5)),
        search_strings_asc_not_found: ("x", &["a", "b", "c", "d", "google", "zoo"], None),
        search_strings_desc_start: ("zoo", &["zoo", "google", "d", "c", "b", "a"], Some(0)),
        search_strings_desc_middle: ("google", &["zoo", "google", "d", "c", "b", "a"], Some(1)),
        search_strings_desc_last: ("a", &["zoo", "google", "d", "c", "b", "a"], Some(5)),
        search_strings_desc_not_found: ("x", &["zoo", "google", "d", "c", "b", "a"], None),
        search_ints_asc_start: (1, &[1, 2, 3, 4], Some(0)),
        search_ints_asc_middle: (3, &[1, 2, 3, 4], Some(2)),
        search_ints_asc_end: (4, &[1, 2, 3, 4], Some(3)),
        search_ints_asc_not_found: (5, &[1, 2, 3, 4], None),
        search_ints_desc_start: (4, &[4, 3, 2, 1], Some(0)),
        search_ints_desc_middle: (3, &[4, 3, 2, 1], Some(1)),
        search_ints_desc_end: (1, &[4, 3, 2, 1], Some(3)),
        search_ints_desc_not_found: (5, &[4, 3, 2, 1], None),
        with_gaps_0: (0, &[1, 3, 8, 11], None),
        with_gaps_1: (1, &[1, 3, 8, 11], Some(0)),
        with_gaps_2: (2, &[1, 3, 8, 11], None),
        with_gaps_3: (3, &[1, 3, 8, 11], Some(1)),
        with_gaps_4: (4, &[1, 3, 8, 10], None),
        with_gaps_5: (5, &[1, 3, 8, 10], None),
        with_gaps_6: (6, &[1, 3, 8, 10], None),
        with_gaps_7: (7, &[1, 3, 8, 11], None),
        with_gaps_8: (8, &[1, 3, 8, 11], Some(2)),
        with_gaps_9: (9, &[1, 3, 8, 11], None),
        with_gaps_10: (10, &[1, 3, 8, 11], None),
        with_gaps_11: (11, &[1, 3, 8, 11], Some(3)),
        with_gaps_12: (12, &[1, 3, 8, 11], None),
        with_gaps_13: (13, &[1, 3, 8, 11], None),
    }
}
