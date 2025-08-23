use data_structures::Heap;
use std::cmp::{Ord, Ordering};

/// 使用最大堆找到数组中第 k 小的元素。
///
/// 该函数维护一个大小为 k 的最大堆。遍历数组时，堆顶始终是当前已遍历元素中第 k 小的元素。
/// 当堆未满时，直接添加元素；当堆满后，如果新元素比堆顶小，则替换堆顶以维持堆的性质。
///
/// # 参数
/// * `input`  - 输入的元素切片，用于查找第 k 小的元素。
/// * `k`      - 要查找的第 k 小元素的位置（从 1 开始计数）。
///
/// # 返回值
/// 返回第 k 小的元素，如果输入为空或 k 无效则返回 None。
#[allow(unused)]
pub fn kth_smallest_heap<T>(input: &[T], k: usize) -> Option<T>
where
    T: Ord + Copy
{
    // 处理空输入的情况
    if input.is_empty() {
        return None;
    }

    // 创建一个最大堆，用于维护当前最小的 k 个元素
    let mut heap = Heap::new_max();

    // 先将前 k 个元素加入堆中
    for &val in input.iter().take(k) {
        heap.add(val);
    }

    // 遍历剩余元素，维护堆的大小为 k
    for &val in input.iter().skip(k) {
        // 取出当前堆中的最大值（即第 k 小的候选值）
        let cur_big = heap.pop().unwrap();

        // 比较当前元素与堆顶元素
        match val.cmp(&cur_big) {
            // 如果当前元素大于堆顶，说明它不是前 k 小的元素，放回原堆顶
            Ordering::Greater => {
                heap.add(cur_big);
            }
            // 否则，用当前元素替换堆顶
            _ => {
                heap.add(val);
            }
        }
    }

    // 最终堆顶就是第 k 小的元素
    heap.pop()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty() {
        let zero: [u8; 0] = [];
        let first = kth_smallest_heap(&zero, 1);

        assert_eq!(None, first);
    }

    #[test]
    fn one_element() {
        let one = [1];
        let first = kth_smallest_heap(&one, 1);

        assert_eq!(1, first.unwrap());
    }

    #[test]
    fn many_elements() {
        // 0 1 3 4 5 7 8 9 9 10 12 13 16 17
        let many = [9, 17, 3, 16, 13, 10, 1, 5, 7, 12, 4, 8, 9, 0];

        let first = kth_smallest_heap(&many, 1);
        let third = kth_smallest_heap(&many, 3);
        let sixth = kth_smallest_heap(&many, 6);
        let fourteenth = kth_smallest_heap(&many, 14);

        assert_eq!(0, first.unwrap());
        assert_eq!(3, third.unwrap());
        assert_eq!(7, sixth.unwrap());
        assert_eq!(17, fourteenth.unwrap());
    }
}
