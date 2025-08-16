/// 合并两个已排序的子数组。
///
/// 该函数将数组 `arr` 中的两个已排序部分进行合并，这两个部分由索引 `mid` 分隔。
/// 左半部分是 `arr[0..mid]`，右半部分是 `arr[mid..]`。
///
/// # 参数
///
/// * `arr` - 需要合并的可变切片引用，包含两个已排序的部分。
/// * `mid` - 分隔左右两个已排序子数组的中间索引。
///
/// # 泛型约束
///
/// * `T` 必须实现 `Ord` 和 `Copy` trait，以便进行比较和复制操作。
#[allow(unused)]
pub fn merge_sort<T: Ord + Copy>(arr: &mut [T], mid: usize) {
    let mut left_index = 0;
    let mut right_index = 0;

    // 创建左右两个子数组的副本，避免在合并过程中修改原数组
    let left_half = arr[..mid].to_vec();
    let right_half = arr[mid..].to_vec();

    // 遍历原始数组，依次从左右两个子数组中选择较小的元素填入
    for item in arr {
        if right_index == right_half.len() || (
            left_index < left_half.len() && left_half[left_index] < right_half[right_index]) {
            *item = left_half[left_index];
            left_index += 1;
        } else {
            *item = right_half[right_index];
            right_index += 1;
        }
    }
}

/// 使用自顶向下方式实现归并排序（递归版本）。
///
/// 该函数通过递归地将数组分成两半，分别对两半进行排序，然后合并结果来实现归并排序。
///
/// # 参数
///
/// * `arr` - 需要排序的可变切片引用。
///
/// # 泛型约束
///
/// * `T` 必须实现 `Ord` 和 `Copy` trait，以便进行比较和复制操作。
#[allow(unused)]
fn top_down_merge_sort<T: Ord + Copy>(arr: &mut [T]) {
    // 基本情况：如果数组长度小于等于1，则已经有序
    if arr.len() <= 1 {
        return;
    }

    // 计算中间索引，将数组分为两半
    let mid = arr.len() / 2;

    // 递归地对左半部分进行排序
    top_down_merge_sort(&mut arr[..mid]);

    // 递归地对右半部分进行排序
    top_down_merge_sort(&mut arr[mid..]);

    // 合并两个已排序的子数组
    merge_sort(arr, mid);
}

/// 使用自底向上方式实现归并排序（迭代版本）。
///
/// 该函数通过逐步增加子数组大小的方式进行排序，从大小为1的子数组开始，
/// 逐步合并成更大的已排序子数组，直到整个数组有序。
///
/// # 参数
///
/// * `arr` - 需要排序的可变切片引用。
///
/// # 泛型约束
///
/// * `T` 必须实现 `Ord` 和 `Copy` trait，以便进行比较和复制操作。
#[allow(unused)]
fn bottom_up_merge_sort<T: Ord + Copy>(arr: &mut [T]) {
    // 基本情况：如果数组长度小于等于1，则已经有序
    if arr.len() <= 1 {
        return;
    }

    let len: usize = arr.len();
    let mut sub_arr_size: usize = 1;

    // 逐步增加子数组大小，从1开始，每次翻倍
    while sub_arr_size < len {
        let mut start_idx: usize = 0;

        // 处理当前大小的所有子数组对
        while len - start_idx > sub_arr_size {
            // 计算当前要合并的子数组的结束索引
            let end_idx: usize = if start_idx + 2 * sub_arr_size > len {
                len
            } else {
                start_idx + 2 * sub_arr_size
            };

            // 合并当前的两个子数组
            merge_sort(&mut arr[start_idx..end_idx], sub_arr_size);

            // 移动到下一个子数组对的起始位置
            start_idx = end_idx;
        }

        // 子数组大小翻倍
        sub_arr_size *= 2;
    }
}


#[cfg(test)]
mod tests {
    #[cfg(test)]
    mod top_down_merge_sort {
        use crate::{have_same_elements, is_sorted};
        use super::super::*;

        #[test]
        fn basic() {
            let mut res = vec![10, 8, 4, 3, 1, 9, 2, 7, 5, 6];
            let cloned = res.clone();
            top_down_merge_sort(&mut res);
            assert!(is_sorted(&res) && have_same_elements(&res, &cloned));
        }

        #[test]
        fn basic_string() {
            let mut res = vec!["a", "bb", "d", "cc"];
            let cloned = res.clone();
            top_down_merge_sort(&mut res);
            assert!(is_sorted(&res) && have_same_elements(&res, &cloned));
        }

        #[test]
        fn empty() {
            let mut res = Vec::<u8>::new();
            let cloned = res.clone();
            top_down_merge_sort(&mut res);
            assert!(is_sorted(&res) && have_same_elements(&res, &cloned));
        }

        #[test]
        fn one_element() {
            let mut res = vec![1];
            let cloned = res.clone();
            top_down_merge_sort(&mut res);
            assert!(is_sorted(&res) && have_same_elements(&res, &cloned));
        }

        #[test]
        fn pre_sorted() {
            let mut res = vec![1, 2, 3, 4];
            let cloned = res.clone();
            top_down_merge_sort(&mut res);
            assert!(is_sorted(&res) && have_same_elements(&res, &cloned));
        }

        #[test]
        fn reverse_sorted() {
            let mut res = vec![4, 3, 2, 1];
            let cloned = res.clone();
            top_down_merge_sort(&mut res);
            assert!(is_sorted(&res) && have_same_elements(&res, &cloned));
        }
    }

    #[cfg(test)]
    mod bottom_up_merge_sort {
        use crate::{have_same_elements, is_sorted};
        use super::super::*;

        #[test]
        fn basic() {
            let mut res = vec![10, 8, 4, 3, 1, 9, 2, 7, 5, 6];
            let cloned = res.clone();
            bottom_up_merge_sort(&mut res);
            assert!(is_sorted(&res) && have_same_elements(&res, &cloned));
        }

        #[test]
        fn basic_string() {
            let mut res = vec!["a", "bb", "d", "cc"];
            let cloned = res.clone();
            bottom_up_merge_sort(&mut res);
            assert!(is_sorted(&res) && have_same_elements(&res, &cloned));
        }

        #[test]
        fn empty() {
            let mut res = Vec::<u8>::new();
            let cloned = res.clone();
            bottom_up_merge_sort(&mut res);
            assert!(is_sorted(&res) && have_same_elements(&res, &cloned));
        }

        #[test]
        fn one_element() {
            let mut res = vec![1];
            let cloned = res.clone();
            bottom_up_merge_sort(&mut res);
            assert!(is_sorted(&res) && have_same_elements(&res, &cloned));
        }

        #[test]
        fn pre_sorted() {
            let mut res = vec![1, 2, 3, 4];
            let cloned = res.clone();
            bottom_up_merge_sort(&mut res);
            assert!(is_sorted(&res) && have_same_elements(&res, &cloned));
        }

        #[test]
        fn reverse_sorted() {
            let mut res = vec![4, 3, 2, 1];
            let cloned = res.clone();
            bottom_up_merge_sort(&mut res);
            assert!(is_sorted(&res) && have_same_elements(&res, &cloned));
        }
    }
}
