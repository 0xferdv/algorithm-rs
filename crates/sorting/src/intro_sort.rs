use crate::{heap_sort, insertion_sort};

/// 对给定的数组进行内省排序（Intro sort）。
///
/// 内省排序是一种混合排序算法，结合了快速排序、堆排序和插入排序的优点。
/// 当子数组长度小于等于16时使用插入排序；
/// 当递归深度超过阈值时切换为堆排序以避免快速排序最坏情况；
/// 否则使用快速排序的分区策略递归处理。
///
/// # 参数
///
/// * `arr` - 待排序的可变切片，元素类型需实现 `Ord` 和 `Copy` trait
///
#[allow(unused)]
pub fn intro_sort<T: Ord + Copy>(arr: &mut [T]) {
    let len = arr.len();
    // 计算最大递归深度，防止快速排序退化到 O(n^2)
    let max_depth = (2.0 * len as f64).log2() as usize + 1;

    /// 递归实现内省排序的核心逻辑
    ///
    /// # 参数
    ///
    /// * `arr` - 当前处理的可变切片
    /// * `max_depth` - 剩余允许的最大递归深度
    ///
    fn intro_sort_recur<T: Ord + Copy>(arr: &mut [T], max_depth: usize) {
        let len = arr.len();
        // 小数组使用插入排序优化性能
        if len <= 16 {
            insertion_sort(arr);
        // 递归深度耗尽时使用堆排序保证 O(n log n) 时间复杂度
        } else if max_depth == 0 {
            heap_sort(arr, false);
        // 大数组且递归深度充足时使用快速排序
        } else {
            let pivot = partition(arr);
            intro_sort_recur(&mut arr[0..pivot], max_depth - 1);
            intro_sort_recur(&mut arr[pivot + 1..], max_depth - 1);
        }
    }

    /// 使用 L o m u t o 分区方案对数组进行分区操作
    ///
    /// 选择中间元素作为基准，将其移到末尾，然后进行分区，
    /// 最后将基准元素放到正确位置并返回该位置索引。
    ///
    /// # 参数
    ///
    /// * `arr` - 待分区的可变切片
    ///
    /// # 返回值
    ///
    /// 基准元素在分区后的正确位置索引
    ///
    fn partition<T: Ord>(arr: &mut [T]) -> usize {
        let len = arr.len();
        let pivot_idx = len / 2;
        // 将基准元素交换到末尾
        arr.swap(pivot_idx, len - 1);
        let mut idx = 0;
        // 遍历除末尾外的所有元素，将小于等于基准的元素移到左侧
        (0..len - 1).for_each(|next_idx| {
            if arr[next_idx] <= arr[len - 1] {
                arr.swap(idx, next_idx);
                idx += 1;
            }
        });
        // 将基准元素放到正确位置
        arr.swap(idx, len - 1);
        idx
    }
    intro_sort_recur(arr, max_depth);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_intro_sort() {
        let mut arr1 = vec![67, 34, 29, 15, 21, 9, 99];
        intro_sort(&mut arr1);
        assert_eq!(arr1, vec![9, 15, 21, 29, 34, 67, 99]);

        let mut arr2 = vec!["sydney", "london", "tokyo", "beijing", "mumbai"];
        intro_sort(&mut arr2);
        assert_eq!(arr2, vec!["beijing", "london", "mumbai", "sydney", "tokyo"]);

        let mut arr3: Vec<i32> = vec![];
        intro_sort(&mut arr3);
        assert_eq!(arr3, vec![]);
    }
}
