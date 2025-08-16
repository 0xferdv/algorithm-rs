/// 比较并交换数组中两个位置的元素
///
/// 根据排序方向比较数组中两个位置的元素，如果不符合排序要求则进行交换
///
/// # 参数
/// * `array` - 需要操作的数组切片
/// * `left` - 左侧元素的索引
/// * `right` - 右侧元素的索引
/// * `ascending` - 排序方向，true表示升序，false表示降序
fn _cmp_to_swap<T: Ord>(array: &mut [T], left: usize, right: usize, ascending: bool) {
    if ascending && array[left] > array[right] || (!ascending && array[left] < array[right]) {
        array.swap(left, right);
    }
}

/// 双调归并函数
///
/// 对双调序列进行归并操作，使其按照指定方向排序
///
/// # 参数
/// * `array` - 需要排序的数组切片
/// * `low` - 当前处理序列的起始索引
/// * `length` - 当前处理序列的长度
/// * `ascending` - 排序方向，true表示升序，false表示降序
fn _bitonic_merge<T: Ord>(array: &mut [T], low: usize, length: usize, ascending: bool) {
    if length > 1 {
        let mid = length / 2;
        // 比较并交换序列前半部分和后半部分对应位置的元素
        (low..(low + mid)).for_each(|i| {
            _cmp_to_swap(array, i, i + mid, ascending);
        });
        // 递归处理前半部分
        _bitonic_merge(array, low, mid, ascending);
        // 递归处理后半部分
        _bitonic_merge(array, low + mid, mid, ascending);
    }
}

/// 双调排序主函数
///
/// 使用双调排序算法对数组进行排序
///
/// # 参数
/// * `array` - 需要排序的数组切片
/// * `low` - 当前处理序列的起始索引
/// * `length` - 当前处理序列的长度
/// * `ascending` - 排序方向，true表示升序，false表示降序
#[allow(unused)]
pub fn bitonic_sort<T: Ord>(array: &mut [T], low: usize, length: usize, ascending: bool) {
    if length > 1 {
        let mid = length / 2;
        // 递归构建前半部分的双调序列
        bitonic_sort(array, low, mid, ascending);
        // 递归构建后半部分的双调序列
        bitonic_sort(array, low + mid, mid, ascending);
        // 归并两个双调序列
        _bitonic_merge(array, low, length, ascending);
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::{have_same_elements, is_sorted};


    #[test]
    fn descending() {
        //descending
        let mut ve1 = vec![6, 5, 4, 3];
        let cloned = ve1.clone();
        bitonic_sort(&mut ve1, 0, 4, true);
        println!("输出排序后的数组vec1: {:?}", ve1);
        assert!(is_sorted(&ve1) && have_same_elements(&ve1, &cloned));
    }
}
