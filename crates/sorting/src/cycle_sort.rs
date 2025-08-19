/// 使用循环排序算法对整数数组进行原地排序。
///
/// 循环排序是一种基于元素应该在的位置来排序的算法，它通过找到每个元素的正确位置并进行交换来实现排序。
/// 该算法尽量减少写操作的次数，适用于写操作成本较高的场景。
///
/// # 参数
///
/// * `arr` - 需要排序的整数数组的可变引用
///
/// # 返回值
///
/// 无返回值，直接修改输入数组
#[allow(unused)]
pub fn cycle_sort(arr: &mut [i32]) {
    let len = arr.len();
    // 如果数组长度小于等于1，则无需排序
    if len <= 1 {
        return;
    }

    // 遍历数组的每个位置作为循环起点
    for cycle_start in 0..len {
        let mut pos = cycle_start;
        let mut item = arr[cycle_start];

        // 计算当前元素应该放置的位置
        for next_item in arr.iter().skip(cycle_start + 1) {
            if *next_item < item {
                pos += 1;
            }
        }

        // 如果当前位置就是正确位置，则跳过
        if pos == cycle_start {
            continue;
        }

        // 跳过重复元素，找到第一个不等于当前元素的位置
        while item == arr[pos] {
            pos += 1;
        }

        // 将元素放到正确位置
        std::mem::swap(&mut arr[pos], &mut item);

        // 继续处理循环中的其他元素，直到回到起始位置
        while pos != cycle_start {
            pos = cycle_start;

            // 重新计算当前元素应该放置的位置
            for next_item in arr.iter().skip(cycle_start + 1) {
                if *next_item < item {
                    pos += 1;
                }
            }

            // 跳过重复元素
            while item == arr[pos] {
                pos += 1;
            }

            // 将元素放到正确位置
            std::mem::swap(&mut arr[pos], &mut item);
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{have_same_elements, is_sorted};
    use super::*;

    #[test]
    fn it_works() {
        let mut arr1 = [6, 5, 4, 3, 2, 1];
        let cloned = arr1;
        cycle_sort(&mut arr1);
        assert!(is_sorted(&arr1) && have_same_elements(&arr1, &cloned));

        arr1 = [12, 343, 21, 90, 3, 21];
        let cloned = arr1;
        cycle_sort(&mut arr1);
        assert!(is_sorted(&arr1) && have_same_elements(&arr1, &cloned));

        let mut arr2 = [1];
        let cloned = arr2;
        cycle_sort(&mut arr2);
        assert!(is_sorted(&arr2) && have_same_elements(&arr2, &cloned));

        let mut arr3 = [213, 542, 90, -23412, -32, 324, -34, 3324, 54];
        let cloned = arr3;
        cycle_sort(&mut arr3);
        assert!(is_sorted(&arr3) && have_same_elements(&arr3, &cloned));
    }
}
