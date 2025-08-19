/// 冒泡排序算法实现
///
/// 该函数使用冒泡排序算法对切片进行原地排序。算法通过重复遍历列表，
/// 比较相邻元素并在顺序错误时交换它们来工作。每轮遍历都会将最大元素
/// "冒泡"到正确位置。
///
/// # 参数
///
/// * `arr` - 需要排序的可变切片引用，元素类型必须实现Ord trait
///
/// # 返回值
///
/// 无返回值，直接修改输入切片
///
/// # 算法复杂度
///
/// * 时间复杂度：最坏情况 O(n²)，最好情况 O(n)
/// * 空间复杂度：O(1)
#[allow(unused)]
pub fn bubble_sort<T: Ord>(arr: &mut [T]) {
    // 如果数组为空，直接返回
    if arr.is_empty() {
        return;
    }

    let mut sorted = false;
    let mut n = arr.len();

    // 持续进行冒泡排序直到没有元素需要交换
    while !sorted {
        sorted = true;
        // 每轮比较相邻元素，将最大元素"冒泡"到末尾
        for i in 0..n.saturating_sub(1) {
            if arr[i] > arr[i + 1] {
                arr.swap(i, i + 1);
                sorted = false;
            }
        }
        // 每轮结束后，最大元素已就位，减少比较范围
        n = n.saturating_sub(1); // 避免潜在的下溢风险
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::{have_same_elements, is_sorted};


    #[test]
    fn descending() {
        let mut ve1 = vec![6, 5, 4, 3, 2, 1];
        let cloned = ve1.clone();
        bubble_sort(&mut ve1);
        assert!(is_sorted(&ve1) && have_same_elements(&ve1, &cloned));
    }

    #[test]
    fn ascending() {
        let mut ve2 = vec![1, 2, 3, 4, 5, 6];
        let cloned = ve2.clone();
        bubble_sort(&mut ve2);
        assert!(is_sorted(&ve2) && have_same_elements(&ve2, &cloned));
    }

    #[test]
    fn empty() {
        let mut ve3: Vec<usize> = vec![];
        let cloned = ve3.clone();
        bubble_sort(&mut ve3);
        assert!(is_sorted(&ve3) && have_same_elements(&ve3, &cloned));
    }
}
