use std::cmp::Ordering;


/// 在一个有序数组中使用二分查找算法查找指定元素。
///
/// 该函数支持升序和降序排列的数组。如果找到目标元素，则返回其索引；
/// 否则返回 `None`。
///
/// # 参数
///
/// - `item`: 要查找的目标元素的引用。
/// - `arr`: 要搜索的有序数组切片。
///
/// # 返回值
///
/// 如果找到目标元素，返回 `Some(索引)`；否则返回 `None`。
#[allow(unused)]
pub fn binary_search<T: Ord>(item: &T, arr: &[T]) -> Option<usize> {
    let is_asc = is_asc_arr(arr);
    let mut left = 0;
    let mut right = arr.len();

    // 使用二分查找不断缩小搜索范围
    while left < right {
        if match_compare(item, arr, &mut left, &mut right, is_asc) {
            return Some(left);
        }
    }
    None
}

/// 根据当前比较结果调整搜索区间，并判断是否找到目标元素。
///
/// 此函数根据数组是升序还是降序来决定如何移动左右指针。
///
/// # 参数
///
/// - `item`: 要查找的目标元素。
/// - `arr`: 当前搜索的数组。
/// - `left`: 搜索区间的左边界（可变引用）。
/// - `right`: 搜索区间的右边界（可变引用）。
/// - `is_asc`: 表示数组是否为升序。
///
/// # 返回值
///
/// 如果找到目标元素则返回 `true`，否则返回 `false`。
fn match_compare<T: Ord>(
    item: &T,
    arr: &[T],
    left: &mut usize,
    right: &mut usize,
    is_asc: bool,
) -> bool {
    let mid = *left + (*right  - *left) / 2;
    let cmp_result = item.cmp(&arr[mid]);

    match (is_asc, cmp_result) {
        // 如果是升序且目标小于中间元素，或降序且目标大于中间元素，则在左半部分查找
        (true, Ordering::Less) | (false, Ordering::Greater) => {
            *right = mid;
        }
        // 如果是升序且目标大于中间元素，或降序且目标小于中间元素，则在右半部分查找
        (true, Ordering::Greater) | (false, Ordering::Less) => {
            *left = mid + 1;
        }
        // 找到目标元素
        (_, Ordering::Equal) => {
            *left = mid;
            return true;
        }
    }
    false
}

/// 判断数组是否为升序排列。
///
/// 通过比较第一个和最后一个元素判断数组的排序方式。
///
/// # 参数
///
/// - `arr`: 要判断的数组。
///
/// # 返回值
///
/// 如果数组长度大于1且首元素小于末元素，则返回 `true`，表示升序；
/// 否则返回 `false`。
fn is_asc_arr<T: Ord>(arr: &[T]) -> bool {
    arr.len() > 1 && arr[0] < arr[arr.len() - 1]
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
                    assert_eq!(binary_search(&item, arr), expected);
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
