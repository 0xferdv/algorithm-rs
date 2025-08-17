/// 对一个 u32 类型的数组进行计数排序。
///
/// # 参数
///
/// * `arr` - 需要排序的数组，元素类型为 u32。
/// * `max_val` - 数组中元素的最大值，用于确定计数数组的大小。
///
/// # 说明
///
/// 该函数使用计数排序算法对数组进行原地排序。计数排序是一种非比较排序算法，
/// 适用于已知数据范围的情况。时间复杂度为 O(n + k)，其中 n 是数组长度，k 是数据范围。
#[allow(unused)]
pub fn counting_sort(arr: &mut [u32], max_val: usize) {
    let len = arr.len();
    // 创建计数数组，初始化为 0
    let mut curs: Vec<usize> = vec![0; max_val + 1];
    // 统计每个元素出现的次数
    (0..len).for_each(|i| curs[arr[i] as usize] += 1);
    let mut i = 0;
    // 根据计数数组重构排序后的数组
    curs.iter().enumerate().for_each(|(data, &number)| {
        (0..number).for_each(|_| {
            arr[i] = data as u32;
            i += 1;
        });
    });
}

use std::ops::AddAssign;

/// 泛型版本的计数排序，支持实现了特定 trait 的类型。
///
/// # 参数
///
/// * `arr` - 需要排序的数组，元素类型为泛型 T。
/// * `max_val` - 数组中元素的最大值，用于确定计数数组的大小。
///
/// # 泛型约束
///
/// * `T` 必须可以转换为 u64（`Into<u64>`）。
/// * `T` 必须可以从 u8 转换而来（`From<u8>`）。
/// * `T` 必须支持 += 运算（`AddAssign`）。
/// * `T` 必须支持复制（Copy）。
///
/// # 说明
///
/// 该函数是计数排序的泛型实现，能够处理多种整数类型。通过类型转换和泛型约束，
/// 实现了对不同整数类型的统一排序逻辑。
#[allow(unused)]
pub fn generic_counting_sort<T: Into<u64> + From<u8> + AddAssign + Copy> (arr: &mut [T], max_val: usize) {
    let len = arr.len();
    // 创建计数数组，初始化为 0
    let mut curs: Vec<usize> = vec![0; max_val + 1];
    // 统计每个元素出现的次数
    (0..len).for_each(|i| curs[arr[i].into() as usize] += 1);
    let mut i = 0;
    let mut data = T::from(0);
    // 根据计数数组重构排序后的数组
    curs.iter().for_each(|&number| {
        (0..number).for_each(|_| {
            arr[i] = data;
            i += 1;
        });
        data += T::from(1);
    });
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::{have_same_elements, is_sorted};

    #[test]
    fn counting_sort_descending() {
        let mut ve1 = vec![6, 5, 4, 3, 2, 1];
        let cloned = ve1.clone();
        counting_sort(&mut ve1, 6);
        assert!(is_sorted(&ve1) && have_same_elements(&ve1, &cloned));
    }

    #[test]
    fn counting_sort_pre_sorted() {
        let mut ve2 = vec![1, 2, 3, 4, 5, 6];
        let cloned = ve2.clone();
        counting_sort(&mut ve2, 6);
        assert!(is_sorted(&ve2) && have_same_elements(&ve2, &cloned));
    }

    #[test]
    fn generic_counting_sort() {
        let mut ve1: Vec<u8> = vec![100, 30, 60, 10, 20, 120, 1];
        let cloned = ve1.clone();
        super::generic_counting_sort(&mut ve1, 120);
        assert!(is_sorted(&ve1) && have_same_elements(&ve1, &cloned));
    }

    #[test]
    fn presorted_u64_counting_sort() {
        let mut ve2: Vec<u64> = vec![1, 2, 3, 4, 5, 6];
        let cloned = ve2.clone();
        super::generic_counting_sort(&mut ve2, 6);
        assert!(is_sorted(&ve2) && have_same_elements(&ve2, &cloned));
    }
}

