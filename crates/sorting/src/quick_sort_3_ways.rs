use rand::Rng;
use std::cmp::Ordering;

/// 三路快速排序的递归实现函数
///
/// 使用三路划分的方式对数组进行原地排序，将数组划分为小于、等于和大于基准值的三个部分。
/// 适用于有大量重复元素的数组，可以有效减少递归深度。
///
/// # 参数
/// * `arr` - 待排序的数组切片，元素类型需要实现Ord trait
/// * `l`   - 排序范围的左边界（包含）
/// * `r`   - 排序范围的右边界（包含）
fn _quick_sort_3_ways<T: Ord>(arr: &mut [T], l: usize, r: usize) {
    // 递归终止条件：当左边界大于等于右边界时停止排序
    if l >= r {
        return;
    }

    // 随机选择基准元素并与第一个元素交换，避免最坏情况
    let mut rng = rand::rng();
    arr.swap(l, rng.random_range(l..=r));

    // 三路划分：lt指向小于区域的右边界，gt指向大于区域的左边界，idx为当前处理元素
    let mut lt = l;
    let mut gt = r + 1;
    let mut idx = l + 1;

    // 三路划分主循环：将数组划分为小于、等于、大于基准值的三部分
    while idx < gt {
        match arr[idx].cmp(&arr[l]) {
            Ordering::Less => {
                // 当前元素小于基准值，将其交换到小于区域
                arr.swap(idx, l + 1);
                idx += 1;
                lt += 1;
            }
            Ordering::Greater => {
                // 当前元素大于基准值，将其交换到大于区域
                arr.swap(idx, gt - 1);
                gt -= 1;
            }
            Ordering::Equal => {
                // 当前元素等于基准值，直接移动指针
                idx += 1;
            }
        }
    }

    // 将基准元素放到正确位置
    arr.swap(l, lt);

    // 递归排序小于基准值的部分
    if lt > 1 {
        _quick_sort_3_ways(arr, l, lt - 1);
    }

    // 递归排序大于基准值的部分
    _quick_sort_3_ways(arr, gt, r);
}

/// 三路快速排序的公共接口函数
///
/// 对整个数组进行三路快速排序，处理边界条件并调用递归实现。
///
/// # 参数
/// * `arr` - 待排序的数组切片，元素类型需要实现Ord trait
pub fn quick_sort_3_ways<T: Ord>(arr: &mut [T]) {
    let len = arr.len();
    // 只有当数组长度大于1时才需要排序
    if len > 1 {
        _quick_sort_3_ways(arr, 0, len - 1);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{have_same_elements, is_sorted};

    #[test]
    fn test_quick_sort_3_ways() {
        let mut arr = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
        let cloned = arr.clone();
        quick_sort_3_ways(&mut arr);
        assert!(is_sorted(&arr) && have_same_elements(&arr, &cloned));
    }
}
