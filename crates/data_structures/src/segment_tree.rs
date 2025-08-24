use std::fmt::Debug;
use std::ops::Range;


#[derive(Debug, PartialEq, Eq)]
pub enum SegmentTreeError {
    InvalidRange,
    IndexOutOfBounds,
}


/// 线段树结构体，用于高效处理区间查询和单点更新操作。
///
/// 泛型参数:
/// - `T`: 节点存储的数据类型，必须实现 Debug、Default、Ord 和 Copy trait。
/// - `F`: 合并函数的类型，是一个接受两个 T 类型参数并返回一个 T 类型结果的闭包。
pub struct SegmentTree<T, F>
where
    T: Debug + Default + Ord + Copy,
    F: Fn(T, T) -> T
{
    size: usize,
    nodes: Vec<T>,
    merge_fn: F,
}

impl<T, F> SegmentTree<T, F>
where
    T: Debug + Default + Ord + Copy,
    F: Fn(T, T) -> T,
{
    /// 使用给定的数组和合并函数构造一个新的线段树。
    ///
    /// 参数:
    /// - `arr`: 用于构建线段树的原始数组切片。
    /// - `merge`: 用于合并子节点值的函数。
    ///
    /// 返回值:
    /// 构造完成的 SegmentTree 实例。
    pub fn from_vec(arr: &[T], merge: F) -> Self {
        let size = arr.len();
        let mut buffer: Vec<T> = vec![T::default(); 2 * size];
        buffer[size..(2 * size)].clone_from_slice(arr);

        // 自底向上构建线段树
        for idx in (1..size).rev() {
            buffer[idx] = merge(buffer[2 * idx], buffer[2 * idx + 1]);
        }

        Self {
            size,
            nodes: buffer,
            merge_fn: merge,
        }
    }

    /// 查询指定区间的值。
    ///
    /// 参数:
    /// - `range`: 需要查询的区间范围（左闭右开）。
    ///
    /// 返回值:
    /// - 成功时返回查询结果，如果区间为空则返回 None；
    /// - 如果区间越界，则返回 InvalidRange 错误。
    pub fn query(&self, range: Range<usize>) -> Result<Option<T>, SegmentTreeError> {
        if range.start >= self.size || range.end > self.size {
            return Err(SegmentTreeError::InvalidRange);
        }

        let mut left = range.start + self.size;
        let mut right = range.end + self.size;
        let mut result = None;

        // 在线段树中遍历查询区间
        while left < right {
            if left % 2 == 1 {
                result = Some(match result {
                    None => self.nodes[left],
                    Some(old) => (self.merge_fn)(old, self.nodes[left]),
                });
                left += 1;
            }
            if right % 2 == 1 {
                right -= 1;
                result = Some(match result {
                    None => self.nodes[right],
                    Some(old) => (self.merge_fn)(old, self.nodes[right]),
                });
            }
            left /= 2;
            right /= 2;
        }

        Ok(result)
    }

    /// 更新指定索引位置的值。
    ///
    /// 参数:
    /// - `idx`: 需要更新的元素在原数组中的索引。
    /// - `val`: 新的值。
    ///
    /// 返回值:
    /// - 成功时返回 Ok(())；
    /// - 如果索引越界，则返回 IndexOutOfBounds 错误。
    pub fn update(&mut self, idx: usize, val: T) -> Result<(), SegmentTreeError> {
        if idx >= self.size {
            return Err(SegmentTreeError::IndexOutOfBounds);
        }

        let mut index = idx + self.size;

        // 如果新旧值相同则无需更新
        if self.nodes[index] == val {
            return Ok(());
        }

        self.nodes[index] = val;

        // 向上更新受影响的节点
        while index > 1 {
            index /= 2;
            self.nodes[index] = (self.merge_fn)(self.nodes[2 * index], self.nodes[2 * index + 1]);
        }

        Ok(())
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use std::cmp::{max, min};

    #[test]
    fn test_min_segments() {
        let vec = vec![-30, 2, -4, 7, 3, -5, 6, 11, -20, 9, 14, 15, 5, 2, -8];
        let mut min_seg_tree = SegmentTree::from_vec(&vec, min);
        assert_eq!(min_seg_tree.query(4..7), Ok(Some(-5)));
        assert_eq!(min_seg_tree.query(0..vec.len()), Ok(Some(-30)));
        assert_eq!(min_seg_tree.query(0..2), Ok(Some(-30)));
        assert_eq!(min_seg_tree.query(1..3), Ok(Some(-4)));
        assert_eq!(min_seg_tree.query(1..7), Ok(Some(-5)));
        assert_eq!(min_seg_tree.update(5, 10), Ok(()));
        assert_eq!(min_seg_tree.update(14, -8), Ok(()));
        assert_eq!(min_seg_tree.query(4..7), Ok(Some(3)));
        assert_eq!(
            min_seg_tree.update(15, 100),
            Err(SegmentTreeError::IndexOutOfBounds)
        );
        assert_eq!(min_seg_tree.query(5..5), Ok(None));
        assert_eq!(
            min_seg_tree.query(10..16),
            Err(SegmentTreeError::InvalidRange)
        );
        assert_eq!(
            min_seg_tree.query(15..20),
            Err(SegmentTreeError::InvalidRange)
        );
    }

    #[test]
    fn test_max_segments() {
        let vec = vec![1, 2, -4, 7, 3, -5, 6, 11, -20, 9, 14, 15, 5, 2, -8];
        let mut max_seg_tree = SegmentTree::from_vec(&vec, max);
        assert_eq!(max_seg_tree.query(0..vec.len()), Ok(Some(15)));
        assert_eq!(max_seg_tree.query(3..5), Ok(Some(7)));
        assert_eq!(max_seg_tree.query(4..8), Ok(Some(11)));
        assert_eq!(max_seg_tree.query(8..10), Ok(Some(9)));
        assert_eq!(max_seg_tree.query(9..12), Ok(Some(15)));
        assert_eq!(max_seg_tree.update(4, 10), Ok(()));
        assert_eq!(max_seg_tree.update(14, -8), Ok(()));
        assert_eq!(max_seg_tree.query(3..5), Ok(Some(10)));
        assert_eq!(
            max_seg_tree.update(15, 100),
            Err(SegmentTreeError::IndexOutOfBounds)
        );
        assert_eq!(max_seg_tree.query(5..5), Ok(None));
        assert_eq!(
            max_seg_tree.query(10..16),
            Err(SegmentTreeError::InvalidRange)
        );
        assert_eq!(
            max_seg_tree.query(15..20),
            Err(SegmentTreeError::InvalidRange)
        );
    }

    #[test]
    fn test_sum_segments() {
        let vec = vec![1, 2, -4, 7, 3, -5, 6, 11, -20, 9, 14, 15, 5, 2, -8];
        let mut sum_seg_tree = SegmentTree::from_vec(&vec, |a, b| a + b);
        assert_eq!(sum_seg_tree.query(0..vec.len()), Ok(Some(38)));
        assert_eq!(sum_seg_tree.query(1..4), Ok(Some(5)));
        assert_eq!(sum_seg_tree.query(4..7), Ok(Some(4)));
        assert_eq!(sum_seg_tree.query(6..9), Ok(Some(-3)));
        assert_eq!(sum_seg_tree.query(9..vec.len()), Ok(Some(37)));
        assert_eq!(sum_seg_tree.update(5, 10), Ok(()));
        assert_eq!(sum_seg_tree.update(14, -8), Ok(()));
        assert_eq!(sum_seg_tree.query(4..7), Ok(Some(19)));
        assert_eq!(
            sum_seg_tree.update(15, 100),
            Err(SegmentTreeError::IndexOutOfBounds)
        );
        assert_eq!(sum_seg_tree.query(5..5), Ok(None));
        assert_eq!(
            sum_seg_tree.query(10..16),
            Err(SegmentTreeError::InvalidRange)
        );
        assert_eq!(
            sum_seg_tree.query(15..20),
            Err(SegmentTreeError::InvalidRange)
        );
    }
}
