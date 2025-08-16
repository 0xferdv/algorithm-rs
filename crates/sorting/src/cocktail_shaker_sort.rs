/// 鸡尾酒搅拌排序（Cocktail Shaker Sort）实现。
///
/// 这是一种冒泡排序的变种，也称为双向冒泡排序。它通过在数组中交替地
/// 正向和反向遍历来排序元素，每次遍历将当前未排序部分的最大值或最小值
/// 移动到正确的位置。
///
/// # 参数
///
/// * `arr` - 一个可变引用的切片，包含需要排序的元素。元素类型必须实现 `Ord` trait。
///
/// # 返回值
///
/// 无返回值。排序结果直接作用于传入的数组。
#[allow(unused)]
pub fn cocktail_shaker_sort<T: Ord>(arr: &mut [T]) {
    let len = arr.len();
    // 如果数组为空，则无需排序，直接返回
    if len == 0 {
        return;
    }

    loop {
        // 第一阶段：正向遍历，将最大元素“冒泡”到右端
        let mut swapped = false;
        (0..(len - 1).clamp(0, len)).for_each(|i| {
            if arr[i] > arr[i + 1] {
                arr.swap(i, i + 1);
                swapped = true;
            }
        });

        // 如果没有发生交换，说明数组已经有序，可以提前结束
        if !swapped {
            break;
        }

        // 第二阶段：反向遍历，将最小元素“冒泡”到左端
        swapped = false;
        (1..(len - 1).clamp(0, len)).rev().for_each(|i| {
            if arr[i] < arr[i - 1] {
                arr.swap(i, i - 1);
                swapped = true;
            }
        });

        // 如果没有发生交换，说明数组已经有序，可以提前结束
        if !swapped {
            break;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{have_same_elements, is_sorted};


    #[test]
    fn basic() {
        let mut arr = vec![5, 2, 1, 3, 4, 6];
        let cloned = arr.clone();
        cocktail_shaker_sort(&mut arr);
        assert!(is_sorted(&arr) && have_same_elements(&arr, &cloned));
    }


    #[test]
    fn empty() {
        let mut arr = Vec::<i32>::new();
        let cloned = arr.clone();
        cocktail_shaker_sort(&mut arr);
        assert!(is_sorted(&arr) && have_same_elements(&arr, &cloned));
    }


    #[test]
    fn one_element() {
        let mut arr = vec![1];
        let cloned = arr.clone();
        cocktail_shaker_sort(&mut arr);
        assert!(is_sorted(&arr) && have_same_elements(&arr, &cloned));
    }


    #[test]
    fn pre_sorted() {
        let mut arr = vec![1, 2, 3, 4, 5, 6];
        let cloned = arr.clone();
        cocktail_shaker_sort(&mut arr);
        assert!(is_sorted(&arr) && have_same_elements(&arr, &cloned));
    }
}
