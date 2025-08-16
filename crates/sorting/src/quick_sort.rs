/// 对数组进行分区操作，使得基准元素左边的元素都小于等于它，
/// 右边的元素都大于等于它。这是快速排序算法的核心步骤。
///
/// # 参数
/// * `arr` - 待分区的可变切片引用
/// * `lo` - 分区范围的起始索引（包含）
/// * `hi` - 分区范围的结束索引（包含），同时也是基准元素的索引
///
/// # 返回值
/// 返回基准元素在分区后的最终位置索引
#[allow(unused)]
pub fn partition<T: PartialOrd>(arr: &mut [T], lo: usize, hi: usize) -> usize {
    let pivot = hi;           // 选择最后一个元素作为基准
    let mut i = lo;           // 左指针，从起始位置开始
    let mut j = hi - 1;       // 右指针，从倒数第二个元素开始

    // 双指针向中间移动，将数组分为两部分
    loop {
        // 找到第一个大于等于基准的元素
        while arr[i] < arr[pivot] {
            i += 1;
        }
        // 找到第一个小于等于基准的元素
        while j > 0 && arr[j] > arr[pivot] {
            j -= 1;
        }
        // 当两个指针相遇或交错时退出循环
        if j == 0 || i >= j {
            break;
        } else if arr[i] == arr[j] {
            // 相等元素同时移动两个指针
            i += 1;
            j -= 1;
        } else {
            // 交换不相等的元素
            arr.swap(i, j);
        }
    }

    // 将基准元素放到正确的位置
    arr.swap(i, pivot);
    i
}

/// 快速排序的递归实现函数，使用尾递归优化减少栈空间使用
///
/// # 参数
/// * `arr` - 待排序的可变切片引用
/// * `lo` - 排序范围的起始索引（包含）
/// * `hi` - 排序范围的结束索引（包含）
#[allow(unused)]
fn _quick_sort<T: Ord>(arr: &mut [T], mut lo: usize, mut hi: usize) {
    // 使用迭代方式实现快速排序，避免过深的递归调用
    while lo < hi {
        let pivot = partition(arr, lo, hi);  // 获取分区点

        // 优先处理较小的子数组，减少递归深度
        if pivot - lo < hi - pivot {
            if pivot > 0 {
                _quick_sort(arr, lo, pivot - 1);  // 递归处理左半部分
            }
            lo = pivot + 1;  // 迭代处理右半部分
        } else {
            _quick_sort(arr, pivot + 1, hi);     // 递归处理右半部分
            hi = pivot - 1;  // 迭代处理左半部分
        }
    }
}

/// 快速排序主函数，对给定数组进行原地排序
///
/// # 参数
/// * `arr` - 待排序的可变切片引用
pub fn quick_sort<T: Ord>(arr: &mut [T]) {
    let len = arr.len();
    if len > 1 {
        _quick_sort(arr, 0, len - 1);
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::{have_same_elements, is_sorted};


    #[test]
    fn basic() {
        let mut res = vec![10, 8, 4, 3, 1, 9, 2, 7, 5, 6];
        let cloned = res.clone();
        quick_sort(&mut res);
        assert!(is_sorted(&res) && have_same_elements(&res, &cloned));
    }


    #[test]
    fn basic_string() {
        let mut res = vec!["a", "bb", "d", "cc"];
        let cloned = res.clone();
        quick_sort(&mut res);
        assert!(is_sorted(&res) && have_same_elements(&res, &cloned));
    }


    #[test]
    fn empty() {
        let mut res = Vec::<u8>::new();
        let cloned = res.clone();
        quick_sort(&mut res);
        assert!(is_sorted(&res) && have_same_elements(&res, &cloned));
    }


    #[test]
    fn one_element() {
        let mut res = vec![1];
        let cloned = res.clone();
        quick_sort(&mut res);
        assert!(is_sorted(&res) && have_same_elements(&res, &cloned));
    }


    #[test]
    fn pre_sorted() {
        let mut res = vec![1, 2, 3, 4];
        let cloned = res.clone();
        quick_sort(&mut res);
        assert!(is_sorted(&res) && have_same_elements(&res, &cloned));
    }


    #[test]
    fn reverse_sorted() {
        let mut res = vec![4, 3, 2, 1];
        let cloned = res.clone();
        quick_sort(&mut res);
        assert!(is_sorted(&res) && have_same_elements(&res, &cloned));
    }
}
