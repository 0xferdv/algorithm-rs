/// 梳排序（Comb Sort）实现
///
/// 梳排序是冒泡排序的一种改进版本，通过使用较大的间隔（gap）来比较和交换元素，
/// 逐步减小间隔直到为1，从而更高效地消除数组中的逆序对。
///
/// # 参数
///
/// * `arr` - 需要排序的可变切片，元素类型必须实现Ord trait
///
/// # 返回值
///
/// 无返回值，原地排序输入数组
#[allow(unused)]
pub fn comb_sort<T: Ord>(arr: &mut [T]) {
    // 初始化间隔为数组长度
    let mut gap = arr.len();
    // 收缩因子，用于逐步减小间隔
    let shrink = 1.3;
    // 排序完成标志
    let mut sorted = false;

    // 当排序未完成时继续循环
    while !sorted {
        // 按收缩因子减小间隔
        gap = (gap as f32 / shrink).floor() as usize;

        // 如果间隔小于等于1，设置为1并标记可能已完成排序
        if gap <= 1 {
            gap = 1;
            sorted = true;
        }

        // 使用当前间隔对数组进行一次扫描比较
        (0..arr.len() - gap).for_each(|i| {
            let j = i + gap;
            // 如果前面的元素大于后面的元素，则交换它们
            if arr[i] > arr[j] {
                arr.swap(i, j);
                // 发生交换说明还未完全排序，重置标志
                sorted = false;
            }
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{have_same_elements, is_sorted};

    #[test]
    fn descending() {
        let mut ve1 = vec![6, 5, 4, 3, 2, 1, 9, 8, 8, 10];
        let cloned = ve1.clone();
        comb_sort(&mut ve1);
        assert!(is_sorted(&ve1) && have_same_elements(&ve1, &cloned));
    }

    #[test]
    fn ascending() {
        let mut ve2 = vec![1, 2, 3, 4, 5, 6];
        let cloned = ve2.clone();
        comb_sort(&mut ve2);
        assert!(is_sorted(&ve2) && have_same_elements(&ve2, &cloned));
    }

    #[test]
    fn duplicates() {
        let mut ve3 = vec![2, 2, 2, 2, 2, 1];
        let cloned = ve3.clone();
        comb_sort(&mut ve3);
        assert!(is_sorted(&ve3) && have_same_elements(&ve3, &cloned));
    }
}
