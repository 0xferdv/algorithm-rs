use std::fmt::Debug;
use std::ops::Range;


#[derive(Debug, PartialEq, Eq)]
#[allow(unused)]
pub enum SegmentTreeError {
    InvalidRange,
    IndexOutOfBounds,
}

/// 线段树结构体，用于高效处理区间查询和单点更新操作。
///
/// # 类型参数
/// * `T` - 线段树中存储的数据类型，必须实现 Debug、Default、Ord 和 Copy trait
/// * `F` - 合并函数的类型，用于定义如何合并两个节点的值
pub struct SegmentTree<T, F>
where
    T: Debug + Default + Ord + Copy,
    F: Fn(T, T) -> T,
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
    /// 从给定的数组创建一个新的线段树
    ///
    /// # 参数
    /// * `arr` - 用于构建线段树的源数组
    /// * `merge_fn` - 定义如何合并两个节点值的函数
    ///
    /// # 返回值
    /// 返回构建好的线段树实例
    #[allow(unused)]
    pub fn from_vec(arr: &[T], merge_fn: F) -> Self {
        let size = arr.len();
        let mut seg_tree = SegmentTree {
            size,
            nodes: vec![T::default(); 4 * size],
            merge_fn,
        };
        if size != 0 {
            seg_tree.build_recursive(arr, 1, 0..size);
        }
        seg_tree
    }

    /// 递归构建线段树
    ///
    /// # 参数
    /// * `arr` - 源数组
    /// * `node_idx` - 当前节点在线段树数组中的索引
    /// * `node_range` - 当前节点所代表的区间范围
    fn build_recursive(&mut self, arr: &[T], node_idx: usize, node_range: Range<usize>) {
        // 如果区间只有一个元素，则直接赋值
        if node_range.end - node_range.start == 1 {
            self.nodes[node_idx] = arr[node_range.start];
        } else {
            // 否则将区间分为两半，递归构建左右子树，然后合并结果
            let mid = node_range.start + (node_range.end - node_range.start) / 2;
            self.build_recursive(arr, node_idx * 2, node_range.start..mid);
            self.build_recursive(arr, node_idx * 2 + 1, mid..node_range.end);
            self.nodes[node_idx] = (self.merge_fn)(self.nodes[node_idx * 2], self.nodes[node_idx * 2 + 1]);
        }
    }

    /// 查询指定区间的值
    ///
    /// # 参数
    /// * `target_range` - 要查询的区间范围
    ///
    /// # 返回值
    /// 成功时返回查询结果的Option包装（None表示空区间），失败时返回错误
    #[allow(unused)]
    pub fn query(&self, target_range: Range<usize>) -> Result<Option<T>, SegmentTreeError> {
        if target_range.start >= self.size || target_range.end > self.size {
            return Err(SegmentTreeError::InvalidRange);
        }
        Ok(self.query_recursive(1, 0..self.size, &target_range))
    }

    /// 递归执行区间查询
    ///
    /// # 参数
    /// * `node_idx` - 当前节点在线段树数组中的索引
    /// * `tree_range` - 当前节点所代表的区间范围
    /// * `target_range` - 要查询的区间范围
    ///
    /// # 返回值
    /// 返回查询结果的Option包装
    fn query_recursive(&self, node_idx: usize, tree_range: Range<usize>, target_range: &Range<usize>) -> Option<T> {
        // 如果当前节点区间与目标区间无交集，返回None
        if tree_range.start >= target_range.end || tree_range.end <= target_range.start {
            return None;
        }
        // 如果当前节点区间完全包含在目标区间内，直接返回当前节点值
        if tree_range.start >= target_range.start && tree_range.end <= target_range.end {
            return Some(self.nodes[node_idx]);
        }
        // 否则递归查询左右子树并合并结果
        let mid = tree_range.start + (tree_range.end - tree_range.start) / 2;
        let left_res = self.query_recursive(node_idx * 2, tree_range.start..mid, target_range);
        let right_res = self.query_recursive(node_idx * 2 + 1, mid..tree_range.end, target_range);
        match (left_res, right_res) {
            (None, None) => None,
            (None, Some(r)) => Some(r),
            (Some(l), None) => Some(l),
            (Some(l), Some(r)) => Some((self.merge_fn)(l, r)),
        }
    }

    /// 更新指定位置的值
    ///
    /// # 参数
    /// * `target_idx` - 要更新的位置索引
    /// * `val` - 新的值
    ///
    /// # 返回值
    /// 成功时返回Ok(()), 失败时返回相应的错误
    pub fn update(&mut self, target_idx: usize, val: T) -> Result<(), SegmentTreeError> {
        if target_idx >= self.size {
            return Err(SegmentTreeError::IndexOutOfBounds);
        }
        self.update_recursive(1, 0..self.size, target_idx, val);
        Ok(())
    }

    /// 递归执行更新操作
    ///
    /// # 参数
    /// * `node_idx` - 当前节点在线段树数组中的索引
    /// * `tree_range` - 当前节点所代表的区间范围
    /// * `target_idx` - 要更新的位置索引
    /// * `val` - 新的值
    fn update_recursive(&mut self, node_idx: usize, tree_range: Range<usize>, target_idx: usize, val: T) {
        // 如果目标索引不在当前节点区间范围内，直接返回
        if tree_range.start > target_idx || tree_range.end <= target_idx {
            return;
        }
        // 如果到达叶子节点且为目标索引，更新值
        if tree_range.end - tree_range.start <= 1 && tree_range.start == target_idx {
            self.nodes[node_idx] = val;
            return;
        }
        // 否则递归更新左右子树，然后重新合并当前节点值
        let mid = tree_range.start + (tree_range.end - tree_range.start) / 2;
        self.update_recursive(node_idx * 2, tree_range.start..mid, target_idx, val);
        self.update_recursive(node_idx * 2 + 1, mid..tree_range.end, target_idx, val);
        self.nodes[node_idx] =
            (self.merge_fn)(self.nodes[node_idx * 2], self.nodes[node_idx * 2 + 1]);
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
