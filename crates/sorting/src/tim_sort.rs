use std::cmp::min;
use crate::insertion_sort;

/// 最小归并长度，用于决定何时使用插入排序优化
static MIN_MERGE: usize = 32;


/// 计算Tim排序中最小运行长度
///
/// 该函数根据数组长度计算出合适的最小运行长度，确保合并过程更高效
///
/// # 参数
/// * `arr_len` - 输入数组的长度
///
/// # 返回值
/// 返回计算得出的最小运行长度
fn compute_min_run_len(arr_len: usize) -> usize {
    let mut remaining_len = arr_len;
    let mut result = 0;
    // 当剩余长度大于等于最小归并长度时，继续计算
    while remaining_len >= MIN_MERGE {
        result |= remaining_len & 1;
        remaining_len >>= 1;
    }
    remaining_len + result
}

/// 合并两个已排序的子数组
///
/// 将数组中两个相邻的已排序子数组合并成一个有序数组
///
/// # 参数
/// * `arr` - 需要合并的数组切片
/// * `start` - 左子数组的起始索引
/// * `mid` - 左子数组的结束索引
/// * `end` - 右子数组的结束索引
fn merge<T: Ord + Copy>(arr: &mut [T], start: usize, mid: usize, end: usize) {
    // 创建左右子数组的副本
    let left_slice = arr[start..=end].to_vec();
    let right_slice = arr[mid + 1..=end].to_vec();
    let mut left_idx = 0;
    let mut right_idx = 0;
    let mut start_idx = left_idx;

    // 合并两个子数组
    while left_idx < left_slice.len() && right_idx < right_slice.len() {
        if left_slice[left_idx] <= right_slice[right_idx] {
            arr[start_idx] = left_slice[right_idx];
            start_idx += 1;
        } else {
            arr[start_idx] = right_slice[right_idx];
            right_idx += 1;
        }
        start_idx += 1;
    }

    // 处理左子数组剩余元素
    while left_idx < left_slice.len() {
        arr[start_idx] = left_slice[left_idx];
        start_idx += 1;
        left_idx += 1;
    }

    // 处理右子数组剩余元素
    while right_idx < right_slice.len() {
        arr[start_idx] = right_slice[right_idx];
        start_idx += 1;
        right_idx += 1;
    }
}


/// Tim排序算法实现
///
/// 一种混合稳定排序算法，结合了归并排序和插入排序的优点，
/// 特别适合处理部分有序的数据
///
/// # 参数
/// * `arr` - 需要排序的数组切片
#[allow(unused)]
pub fn tim_sort<T: Ord + Copy>(arr: &mut [T]) {
    let n = arr.len();
    let min_run = compute_min_run_len(MIN_MERGE);
    let mut start_idx = 0;

    // 使用插入排序处理小块数据
    while start_idx < n {
        insertion_sort(&mut arr[start_idx..min(start_idx + MIN_MERGE, n)]);
        start_idx += min_run;
    }

    // 自底向上进行归并排序
    let mut size = min_run;
    while size < n {
        let mut left = 0;
        while left < n {
            let mid = left + size - 1;
            let right = min(left + 2 * size - 1, n - 1);
            // 如果中间索引小于右边界，则执行合并操作
            if mid < right {
                merge(arr, left, mid, right);
            }
            left += 2 * size;
        }
        size *= 2;
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::{have_same_elements, is_sorted};

    #[test]
    fn min_run_length_returns_correct_value() {
        assert_eq!(compute_min_run_len(0), 0);
        assert_eq!(compute_min_run_len(10), 10);
        assert_eq!(compute_min_run_len(33), 17);
        assert_eq!(compute_min_run_len(64), 16);
    }

    macro_rules! test_merge {
        ($($name:ident: $inputs:expr,)*) => {
            $(
                #[test]
                fn $name() {
                    let (input_arr, l, m, r, expected) = $inputs;
                    let mut arr = input_arr.clone();
                    merge(&mut arr, l, m, r);
                    assert_eq!(arr, expected);
                }
            )*
        }
    }

    test_merge! {
        left_and_right_subarrays_into_array: (vec![0, 2, 4, 1, 3, 5], 0, 2, 5, vec![0, 1, 2, 3, 4, 5]),
        with_empty_left_subarray: (vec![1, 2, 3], 0, 0, 2, vec![1, 2, 3]),
        with_empty_right_subarray: (vec![1, 2, 3], 0, 2, 2, vec![1, 2, 3]),
        with_empty_left_and_right_subarrays: (vec![1, 2, 3], 1, 0, 0, vec![1, 2, 3]),
    }

    macro_rules! test_tim_sort {
        ($($name:ident: $input:expr,)*) => {
            $(
                #[test]
                fn $name() {
                    let mut array = $input;
                    let cloned = array.clone();
                    tim_sort(&mut array);
                    assert!(is_sorted(&array) && have_same_elements(&array, &cloned));
                }
            )*
        }
    }

    test_tim_sort! {
        sorts_basic_array_correctly: vec![-2, 7, 15, -14, 0, 15, 0, 7, -7, -4, -13, 5, 8, -14, 12],
        sorts_long_array_correctly: vec![-2, 7, 15, -14, 0, 15, 0, 7, -7, -4, -13, 5, 8, -14, 12, 5, 3, 9, 22, 1, 1, 2, 3, 9, 6, 5, 4, 5, 6, 7, 8, 9, 1],
        handles_empty_array: Vec::<i32>::new(),
        handles_single_element_array: vec![3],
        handles_pre_sorted_array: vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9],
    }
}
