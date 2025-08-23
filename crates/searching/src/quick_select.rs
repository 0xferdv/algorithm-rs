use sorting::partition;

/// 快速选择算法实现，用于在未排序的数组中找到第k小的元素
///
/// # 参数
/// * `list` - 待处理的可变整数切片
/// * `left` - 搜索范围的左边界（包含）
/// * `right` - 搜索范围的右边界（包含）
/// * `index` - 要查找的元素索引（第k小的元素，从0开始计数）
///
/// # 返回值
/// 返回数组中第index小的元素值
///
/// # 算法原理
/// 使用分治思想，基于快速排序的分区操作，只递归处理包含目标元素的分区，
/// 从而避免完全排序整个数组，平均时间复杂度为O(n)
#[allow(unused)]
pub fn quick_select(list: &mut [i32], left: usize, right: usize, index: usize) -> i32 {
    // 基础情况：当搜索范围只有一个元素时，直接返回该元素
    if left == right {
        return list[left];
    }

    // 选择中间位置作为初始基准点，然后进行分区操作
    let mut pivot_idx = left + (right - left) / 2;
    pivot_idx = partition(list, left, right);

    // 根据目标索引与基准点位置的关系，决定下一步搜索方向
    match index {
        x if x == pivot_idx => list[x],  // 找到目标元素，直接返回
        x if x < pivot_idx => quick_select(list, left, pivot_idx - 1, index),  // 在左半部分继续搜索
        _  => quick_select(list, pivot_idx + 1, right, index),  // 在右半部分继续搜索
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let mut arr1 = [2, 3, 4, 5];
        assert_eq!(quick_select(&mut arr1, 0, 3, 1), 3);
        let mut arr2 = [2, 5, 9, 12, 16];
        assert_eq!(quick_select(&mut arr2, 1, 3, 2), 9);
        let mut arr2 = [0, 3, 8];
        assert_eq!(quick_select(&mut arr2, 0, 0, 0), 0);
    }
}
