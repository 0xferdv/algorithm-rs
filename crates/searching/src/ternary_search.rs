use std::cmp::Ordering;


/// 在一个有序数组中使用三分查找算法搜索指定元素。
///
/// 该函数支持升序和降序排列的数组。如果找到目标元素，则返回其索引；
/// 否则返回 `None`。
///
/// # 参数
///
/// - `item`: 要查找的目标元素引用。
/// - `arr`: 已排序的切片，用于搜索目标元素。
///
/// # 返回值
///
/// 如果找到目标元素，返回 `Some(索引)`；否则返回 `None`。
#[allow(unused)]
pub fn ternary_search<T: Ord>(item: &T, arr: &[T]) -> Option<usize> {
    if arr.is_empty() {
        return None;
    }
    let is_asc = is_asc_arr(arr);
    let mut left = 0;
    let mut right = arr.len() - 1;
    while left <= right {
        if match_compare(item, arr, &mut left, &mut right, is_asc) {
            return Some(left);
        }
    }
    None
}

/// 根据当前区间的左右边界进行三分比较，调整搜索区间或判断是否找到目标元素。
///
/// 该函数计算两个中间点，并根据目标值与这两个中间点值的比较结果，
/// 决定下一步搜索区间的范围。
///
/// # 参数
///
/// - `item`: 要查找的目标元素引用。
/// - `arr`: 已排序的切片。
/// - `left`: 当前搜索区间的左边界（可变引用）。
/// - `right`: 当前搜索区间的右边界（可变引用）。
/// - `is_asc`: 表示数组是否为升序排列。
///
/// # 返回值
///
/// 如果找到目标元素，返回 `true`；否则返回 `false`。
fn match_compare<T: Ord>(
    item: &T,
    arr: &[T],
    left: &mut usize,
    right: &mut usize,
    is_asc: bool,
) -> bool {
    // 计算两个三分点
    let first_mid = *left + (*right -  *left) / 3;
    let second_mid = *right - (*right - *left) / 3;

    // 特殊情况：当两个三分点重合且等于左边界时，直接比较
    if first_mid == second_mid && first_mid == *left {
        return match &arr[*left] {
            x if x == item => true,
            _ => {
                *left += 1;
                false
            }
        };
    }

    // 比较目标值与两个三分点的大小关系
    let cmp_first_mid = item.cmp(&arr[first_mid]);
    let cmp_second_mid = item.cmp(&arr[second_mid]);

    // 根据比较结果决定下一步搜索区间
    match (is_asc, cmp_first_mid, cmp_second_mid) {
        (_, Ordering::Equal, _) => {
            *left = first_mid;
            return true;
        }
        (_, _, Ordering::Equal) => {
            *left = second_mid;
            return true;
        }
        (true, Ordering::Less, _) | (false, Ordering::Greater, _) => {
            *right = first_mid.saturating_sub(1)
        }
        (true, _, Ordering::Greater) | (false, _, Ordering::Less) => *left = second_mid + 1,
        (_, _, _) => {
            *left = first_mid + 1;
            *right = second_mid - 1;
        }
    }
    false
}

/// 判断给定数组是否为升序排列。
///
/// 通过比较首尾元素来粗略判断数组的排序方向。
///
/// # 参数
///
/// - `arr`: 待判断的数组切片。
///
/// # 返回值
///
/// 如果数组长度大于1且首元素小于尾元素，返回 `true`；否则返回 `false`。
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
                    if let Some(expected_index) = expected {
                        assert_eq!(arr[expected_index], item);
                    }
                    assert_eq!(ternary_search(&item, arr), expected);
                }
            )*
        };
    }

    test_cases! {
        empty: ("a", &[] as &[&str], None),
        one_item_found: ("a", &["a"], Some(0)),
        one_item_not_found: ("b", &["a"], None),
        search_two_elements_found_at_start: (1, &[1, 2], Some(0)),
        search_two_elements_found_at_end: (2, &[1, 2], Some(1)),
        search_two_elements_not_found_start: (0, &[1, 2], None),
        search_two_elements_not_found_end: (3, &[1, 2], None),
        search_three_elements_found_start: (1, &[1, 2, 3], Some(0)),
        search_three_elements_found_middle: (2, &[1, 2, 3], Some(1)),
        search_three_elements_found_end: (3, &[1, 2, 3], Some(2)),
        search_three_elements_not_found_start: (0, &[1, 2, 3], None),
        search_three_elements_not_found_end: (4, &[1, 2, 3], None),
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
