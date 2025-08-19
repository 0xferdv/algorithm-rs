/// Stooge排序实现（内部递归函数）
/// 
/// 该函数实现了Stooge排序算法的核心逻辑。
/// 它通过递归地排序前2/3部分，然后排序后2/3部分，
/// 最后再次排序前2/3部分来确保整个数组有序。
/// 
/// 算法工作原理如下：
/// 1. 如果第一个元素大于最后一个元素，则交换它们。
/// 2. 如果元素个数超过两个：
///    a. 递归排序前2/3的元素。
///    b. 递归排序后2/3的元素。
///    c. 再次递归排序前2/3以确保正确性。
/// 
/// # 参数
/// * `arr` - 需要排序的可变元素切片。元素必须实现`Ord` trait。
/// * `start` - 要排序的子数组的起始索引。
/// * `end` - 要排序的子数组的结束索引。
/// 
/// # 时间复杂度
/// O(n^(log 3 / log 1.5)) ≈ O(n^2.7095)
/// 
/// # 空间复杂度
/// O(n) 由于递归调用栈
fn _stooge_sort<T: Ord>(arr: &mut [T], start: usize, end: usize) {
    // 如果第一个元素大于最后一个元素，则交换它们
    if arr[start] > arr[end] {
        arr.swap(start, end);
    }
    
    // 基本情况：如果子数组只有2个或更少元素，则已经有序
    if start + 1 >= end {
        return;
    }
    
    // 计算子数组长度的三分之一
    let k = (end - start + 1) / 3;
    
    // 递归排序前2/3的子数组
    _stooge_sort(arr, start, end - k);
    // 递归排序后2/3的子数组
    _stooge_sort(arr, start + k, end);
    // 再次递归排序前2/3以确保正确性
    _stooge_sort(arr, start, end - k);
}

/// Stooge排序算法的公共接口。
/// 
/// Stooge排序是一种递归排序算法，时间复杂度为
/// O(n^(log 3 / log 1.5)) ≈ O(n^2.7095)。它通过以特定模式递归排序
/// 数组的三分之二部分来工作。
/// 
/// # 参数
/// * `arr` - 需要排序的可变元素切片。元素必须实现`Ord` trait。
/// 
/// # 示例
///
/// let mut arr = vec![3, 5, 6, 3, 1, 4];
/// stooge_sort(&mut arr);
/// assert_eq!(arr, vec![1, 3, 3, 4, 5, 6]);

///
/// # 时间复杂度
/// O(n^(log 3 / log 1.5)) ≈ O(n^2.7095)
///
/// # 空间复杂度
/// O(n) 由于递归调用栈
#[allow(unused)]
pub fn stooge_sort<T: Ord>(arr: &mut [T]) {
    // 处理空数组的情况
    if arr.is_empty() {
        return;
    }

    let len = arr.len();
    // 使用整个数组范围调用内部排序函数
    _stooge_sort(arr, 0, len - 1);
}


#[cfg(test)]
mod test {
    use super::*;
    use crate::{have_same_elements, is_sorted};

    #[test]
    fn basic() {
        let mut vec = vec![3, 5, 6, 3, 1, 4];
        let cloned = vec.clone();
        stooge_sort(&mut vec);
        assert!(is_sorted(&vec) && have_same_elements(&vec, &cloned));
    }

    #[test]
    fn empty() {
        let mut vec: Vec<i32> = vec![];
        let cloned = vec.clone();
        stooge_sort(&mut vec);
        assert!(is_sorted(&vec) && have_same_elements(&vec, &cloned));
    }

    #[test]
    fn reverse() {
        let mut vec = vec![6, 5, 4, 3, 2, 1];
        let cloned = vec.clone();
        stooge_sort(&mut vec);
        assert!(is_sorted(&vec) && have_same_elements(&vec, &cloned));
    }

    #[test]
    fn already_sorted() {
        let mut vec = vec![1, 2, 3, 4, 5, 6];
        let cloned = vec.clone();
        stooge_sort(&mut vec);
        assert!(is_sorted(&vec) && have_same_elements(&vec, &cloned));
    }
}
