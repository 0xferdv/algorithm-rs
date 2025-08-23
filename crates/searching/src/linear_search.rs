/// 线性搜索 - 基础版本
///
/// # 参数
/// * `item` - 要搜索的目标元素
/// * `arr` - 搜索的数组切片
///
/// # 返回值
/// 如果找到元素返回其索引，否则返回 None
///
/// # 泛型约束
/// T 必须实现 Ord trait 以支持排序比较
#[allow(unused)]
pub fn linear_search<T: Ord>(item: &T, arr: &[T]) -> Option<usize> {
    for (i, data) in arr.iter().enumerate() {
        if item == data {
            return Some(i);
        }
    }
    None
}

/// 线性搜索 - 使用迭代器实现
///
/// # 参数
/// * `item` - 要搜索的目标元素
/// * `arr` - 搜索的数组切片
///
/// # 返回值
/// 如果找到元素返回其索引，否则返回 None
///
/// # 泛型约束
/// T 必须实现 PartialEq trait 以支持相等比较
pub fn linear_search_iter<T: PartialEq>(item: &T, arr: &[T]) -> Option<usize> {
    arr.iter().position(|x| x == item)
}

/// 线性搜索 - 使用手动索引遍历
///
/// # 参数
/// * `item` - 要搜索的目标元素
/// * `arr` - 搜索的数组切片
///
/// # 返回值
/// 如果找到元素返回其索引，否则返回 None
pub fn linear_search_idx<T: PartialEq>(item: &T, arr: &[T]) -> Option<usize> {
    let mut i = 0;
    while i < arr.len() {
        if arr[i] == *item {
            return Some(i);
        }
        i += 1;
    }
    None
}

/// 线性搜索 - 使用指针偏移量实现
///
/// # 参数
/// * `item` - 要搜索的目标元素
/// * `arr` - 搜索的数组切片
///
/// # 返回值
/// 如果找到元素返回其索引，否则返回 None
pub fn linear_search_point<T: PartialEq>(item: &T, arr: &[T]) -> Option<usize> {
    let start = arr.as_ptr();
    let end = unsafe { start.add(arr.len()) };
    let mut current = start;

    unsafe {
        while current < end {
            if *current == *item {
                return Some(current.offset_from(start) as usize);
            }
            current = current.add(1);
        }
    }
    None
}

/// 线性搜索 - 函数式风格实现
///
/// # 参数
/// * `item` - 要搜索的目标元素
/// * `arr` - 搜索的数组切片
///
/// # 返回值
/// 如果找到元素返回其索引，否则返回 None
pub fn linear_search_fn<T: PartialEq>(item: &T, arr: &[T]) -> Option<usize> {
    arr.iter()
        .enumerate()
        .find_map(|(index, value)| if value == item { Some(index) } else { None })
}

/// 线性搜索 - 支持自定义比较函数
///
/// # 参数
/// * `item` - 要搜索的目标元素
/// * `arr` - 搜索的数组切片
/// * `predicate` - 自定义比较函数
///
/// # 返回值
/// 如果找到元素返回其索引，否则返回 None
pub fn linear_search_by<T, F>(arr: &[T], mut predicate: F) -> Option<usize>
where
    F: FnMut(&T) -> bool,
{
    arr.iter().position(|x| predicate(x))
}

/// 使用自定义比较的线性搜索包装函数
///
/// # 参数
/// * `item` - 要搜索的目标元素
/// * `arr` - 搜索的数组切片
///
/// # 返回值
/// 如果找到元素返回其索引，否则返回 None
pub fn linear_search_from_by<T: PartialEq>(item: &T, arr: &[T]) -> Option<usize> {
    linear_search_by(arr, |x| x == item)
}


#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! test_cases {
        ($($name:ident: $tc:expr,)*) => {
            $(
                #[test]
                fn $name() {
                    let (item, arr, expected) = $tc;
                    // 验证预期索引对应的元素确实等于目标元素
                    if let Some(expected_index) = expected {
                        assert_eq!(arr[expected_index], item);
                    }
                    // 验证线性搜索结果与预期一致
                    assert_eq!(linear_search(&item, arr), expected);
                }
            )*
        }
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
