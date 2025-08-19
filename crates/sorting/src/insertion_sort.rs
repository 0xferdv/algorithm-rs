/// 插入排序函数，对给定的切片进行原地排序。
///
/// 该函数使用插入排序算法对输入的可变切片进行升序排序。插入排序是一种简单直观的排序算法，
/// 它的工作原理是通过构建有序序列，对于未排序数据，在已排序序列中从后向前扫描，找到相应位置并插入。
///
/// # 参数
///
/// * `arr` - 一个可变引用的泛型切片，元素类型必须实现 `Ord` 和 `Copy` trait。
///
/// # 泛型约束
///
/// * `T: Ord` - 元素类型必须支持比较操作。
/// * `T: Copy` - 元素类型必须支持复制操作。


#[allow(unused)]
pub fn insertion_sort<T: Ord + Copy>(arr: &mut [T]) {
    // 如果数组长度小于等于1，则无需排序，直接返回
    if arr.len() <= 1 {
        return;
    }

    // 从第二个元素开始，逐个将元素插入到已排序的部分中
    (1..arr.len()).for_each(|i| {
        let mut j = i;
        let cur = arr[i];

        // 将当前元素向前移动，直到找到合适的位置插入
        while j > 0 && cur < arr[j - 1] {
            arr[j] = arr[j - 1];
            j -= 1;
        }

        // 将当前元素插入到正确位置
        arr[j] = cur;
    })
}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::{have_same_elements, is_sorted};
    
    #[test]
    fn empty() {
        let mut arr: [u8; 0] = [];
        let cloned = arr;
        insertion_sort(&mut arr);
        assert!(is_sorted(&arr) && have_same_elements(&arr, &cloned));
    }
    
    #[test]
    fn one_element() {
        let mut arr: [char; 1] = ['a'];
        let cloned = arr;
        insertion_sort(&mut arr);
        assert!(is_sorted(&arr) && have_same_elements(&arr, &cloned));
    }
    
    #[test]
    fn already_sorted() {
        let mut arr: [&str; 3] = ["a", "d", "c"];
        let cloned = arr;
        insertion_sort(&mut arr);
        assert!(is_sorted(&arr) && have_same_elements(&arr, &cloned));
    }
    
    #[test]
    fn basic() {
        let mut arr: [&str; 4] = ["d", "a", "c", "b"];
        let cloned = arr;
        insertion_sort(&mut arr);
        assert!(is_sorted(&arr) && have_same_elements(&arr, &cloned));
    }
    
    #[test]
    fn odd_number_of_elements() {
        let mut arr: Vec<&str> = vec!["d", "a", "c", "e", "b"];
        let cloned = arr.clone();
        insertion_sort(&mut arr);
        assert!(is_sorted(&arr) && have_same_elements(&arr, &cloned));
    }
    
    #[test]
    fn repeated_elements() {
        let mut arr: Vec<usize> = vec![542, 542, 542, 542];
        let cloned = arr.clone();
        insertion_sort(&mut arr);
        assert!(is_sorted(&arr) && have_same_elements(&arr, &cloned));
    }
}
