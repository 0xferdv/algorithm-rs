/// 奇偶排序（Odd-Even Sort）函数，也称为砖排序（Brick Sort）。
/// 这是一种基于比较的排序算法，类似于冒泡排序，但通过交替地对奇数索引和偶数索引的元素进行比较交换来优化性能。
///
/// # 参数
/// * `arr` - 一个可变引用的切片，包含需要排序的元素。元素类型必须实现 `Ord` trait 以支持比较操作。
///
/// # 返回值
/// 无返回值。排序结果将直接作用于传入的数组切片中。
///
/// # 算法说明
/// 该算法通过不断交替地对偶数位置和奇数位置的相邻元素进行比较与交换，逐步将数组排序。
/// 每一轮包括两个阶段：
/// 1. 对所有偶数索引位置的元素与其下一个元素进行比较，若顺序错误则交换。
/// 2. 对所有奇数索引位置的元素与其下一个元素进行比较，若顺序错误则交换。
/// 当某一轮中没有发生任何交换时，排序完成。
pub fn odd_even_sort<T: Ord>(arr: &mut [T]) {
    let len = arr.len();
    // 处理空数组的情况
    if len == 0 {
        return;
    }
    let mut sorted = false;
    // 循环直到数组完全有序
    while !sorted {
        sorted = true;
        // 第一阶段：处理偶数索引位置的元素对
        (0..len - 1).step_by(2).for_each(|idx| {
            if arr[idx] > arr[idx + 1] {
                arr.swap(idx, idx + 1);
                sorted = false;
            }
        });
        // 第二阶段：处理奇数索引位置的元素对
        (1..len - 1).step_by(2).for_each(|idx| {
            if arr[idx] > arr[idx + 1] {
                arr.swap(idx, idx + 1);
                sorted = false;
            }
        });
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{have_same_elements, is_sorted};

    #[test]
    fn basic() {
        let mut arr = vec![3, 5, 1, 2, 4, 6];
        let cloned = arr.clone();
        odd_even_sort(&mut arr);
        assert!(is_sorted(&arr) && have_same_elements(&arr, &cloned));
    }

    #[test]
    fn empty() {
        let mut arr = Vec::<i32>::new();
        let cloned = arr.clone();
        odd_even_sort(&mut arr);
        assert!(is_sorted(&arr) && have_same_elements(&arr, &cloned));
    }

    #[test]
    fn one_element() {
        let mut arr = vec![3];
        let cloned = arr.clone();
        odd_even_sort(&mut arr);
        assert!(is_sorted(&arr) && have_same_elements(&arr, &cloned));
    }

    #[test]
    fn pre_sorted() {
        let mut arr = vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
        let cloned = arr.clone();
        odd_even_sort(&mut arr);
        assert!(is_sorted(&arr) && have_same_elements(&arr, &cloned));
    }
}
