/**
 * 交换排序函数，对给定的整数数组进行升序排序
 *
 * 该函数使用交换排序算法（类似选择排序的变种）对数组进行原地排序。
 * 算法通过遍历数组，对每个位置与后续所有位置进行比较，
 * 如果发现更小的元素则进行交换，最终实现数组的升序排列。
 *
 * 参数:
 * arr: &mut [i32] - 需要排序的可变整数数组切片
 *
 * 返回值:
 * 无返回值，直接修改输入数组
 */
#[allow(unused)]
pub fn exchange_sort(arr: &mut [i32]) {
    let len = arr.len();
    // 外层循环遍历数组的每个位置
    (0..len).for_each(|idx| {
        // 内层循环遍历当前位置之后的所有元素
        ((idx + 1)..len).for_each(|next_idx| {
            // 如果后面的元素小于当前元素，则交换两者位置
            if arr[next_idx] < arr[idx] {
                arr.swap(idx, next_idx);
            }
        });
    });
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{have_same_elements, is_sorted};

    #[test]
    fn it_works() {
        let mut arr1 = [6, 5, 4, 3, 2, 1];
        let cloned = arr1;
        exchange_sort(&mut arr1);
        assert!(is_sorted(&arr1) && have_same_elements(&arr1, &cloned));

        arr1 = [12, 343, 21, 90, 3, 21];
        let cloned = arr1;
        exchange_sort(&mut arr1);
        assert!(is_sorted(&arr1) && have_same_elements(&arr1, &cloned));

        let mut arr2 = [1];
        let cloned = arr2;
        exchange_sort(&mut arr2);
        assert!(is_sorted(&arr2) && have_same_elements(&arr2, &cloned));

        let mut arr3 = [213, 542, 90, -23412, -32, 324, -34, 3324, 54];
        let cloned = arr3;
        exchange_sort(&mut arr3);
        assert!(is_sorted(&arr3) && have_same_elements(&arr3, &cloned));
    }
}
