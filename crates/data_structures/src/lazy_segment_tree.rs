use std::fmt::{Debug, Display};
use std::ops::{Add, AddAssign, Range};

/// 懒惰线段树结构体，支持区间查询和区间更新操作。
///
/// 泛型参数 T 必须实现以下 trait：
/// - Debug: 用于调试输出
/// - Default: 提供默认值
/// - Ord: 支持比较操作
/// - Copy: 支持复制语义
/// - Display: 用于格式化显示
/// - AddAssign: 支持 += 操作
/// - Add<Output = T>: 支持加法操作
pub struct LazySegmentTree<T>
where
    T: Debug + Default + Ord + Copy + Display + AddAssign + Add<Output = T>
{
    /// 原始数组的长度
    len: usize,
    /// 存储线段树节点值的数组
    tree: Vec<T>,
    /// 存储懒惰标记的数组，None 表示无标记
    lazy: Vec<Option<T>>,
    /// 合并两个节点值的函数指针
    merge: fn(T, T) -> T
}

impl<T> LazySegmentTree<T>
where
    T: Debug + Default + Ord + Copy + Display + AddAssign + Add<Output = T>
{
    /// 从给定的数组创建一个新的懒惰线段树
    ///
    /// # 参数
    /// * `arr` - 用于构建线段树的原始数组切片
    /// * `merge` - 用于合并两个节点值的函数指针
    ///
    /// # 返回值
    /// 返回构建好的 LazySegmentTree 实例
    pub fn from_vec(arr: &[T], merge: fn(T, T) -> T) -> Self {
        let len = arr.len();
        let mut seg_str = LazySegmentTree {
            len,
            tree: vec![T::default(); 4 * len],
            lazy: vec![None; 4 * len],
            merge,
        };
        if len != 0 {
            seg_str.build_recursive(arr, 1, 0..len, merge);
        }
        seg_str
    }

    /// 递归构建线段树
    ///
    /// # 参数
    /// * `arr` - 原始数组切片
    /// * `idx` - 当前线段树节点的索引
    /// * `range` - 当前节点表示的区间范围
    /// * `merge` - 合并函数指针
    fn build_recursive(
        &mut self,
        arr: &[T],
        idx: usize,
        range: Range<usize>,
        merge: fn(T, T) -> T,
    ) {
        // 如果区间只有一个元素，直接赋值
        if range.end - range.start == 1 {
            self.tree[idx] = arr[range.start];
        } else {
            // 否则递归构建左右子树，然后合并结果
            let mid = range.start + (range.end - range.start) / 2;
            self.build_recursive(arr, 2 * idx, range.start..mid, merge);
            self.build_recursive(arr, 2 * idx + 1, mid..range.end, merge);
            self.tree[idx] = merge(self.tree[2 * idx], self.tree[2 * idx + 1]);
        }
    }

    /// 查询指定区间的值
    ///
    /// # 参数
    /// * `range` - 要查询的区间范围
    ///
    /// # 返回值
    /// 返回查询区间经过合并后的结果，如果区间无效则返回 None
    pub fn query(&mut self, range: Range<usize>) -> Option<T> {
        self.query_recursive(1, 0..self.len, &range)
    }

    /// 递归执行区间查询
    ///
    /// # 参数
    /// * `idx` - 当前线段树节点的索引
    /// * `element_range` - 当前节点表示的元素区间
    /// * `query_range` - 用户请求查询的区间
    ///
    /// # 返回值
    /// 返回查询结果，如果无交集则返回 None
    fn query_recursive(
        &mut self,
        idx: usize,
        element_range: Range<usize>,
        query_range: &Range<usize>,
    ) -> Option<T> {
        // 如果当前节点区间与查询区间无交集，返回 None
        if element_range.start >= query_range.end || element_range.end <= query_range.start {
            return None;
        }

        // 如果当前节点有懒惰标记，需要先下传
        if self.lazy[idx].is_some() {
            self.propagation(idx, &element_range, T::default());
        }

        // 如果当前节点区间完全包含在查询区间内，直接返回节点值
        if element_range.start >= query_range.start && element_range.end <= query_range.end {
            return Some(self.tree[idx]);
        }

        // 递归查询左右子树并合并结果
        let mid = element_range.start + (element_range.end - element_range.start) / 2;
        let left = self.query_recursive(2 * idx, element_range.start..mid, query_range);
        let right = self.query_recursive(2 * idx + 1, mid..element_range.end, query_range);
        match (left, right) {
            (None, None) => None,
            (None, Some(r)) => Some(r),
            (Some(l), None) => Some(l),
            (Some(l), Some(r)) => Some((self.merge)(l, r)),
        }
    }

    /// 更新指定区间的值
    ///
    /// # 参数
    /// * `target_range` - 需要更新的目标区间
    /// * `val` - 要增加的值
    pub fn update(&mut self, target_range: Range<usize>, val: T) {
        self.update_recursive(1, 0..self.len, &target_range, val);
    }

    /// 递归执行区间更新
    ///
    /// # 参数
    /// * `idx` - 当前线段树节点的索引
    /// * `element_range` - 当前节点表示的元素区间
    /// * `target_range` - 需要更新的目标区间
    /// * `val` - 要增加的值
    fn update_recursive(
        &mut self,
        idx: usize,
        element_range: Range<usize>,
        target_range: &Range<usize>,
        val: T,
    ) {
        // 如果当前节点区间与目标区间无交集，直接返回
        if element_range.start >= target_range.end || element_range.end <= target_range.start {
            return;
        }

        // 如果当前节点区间只有一个元素，直接更新
        if element_range.end - element_range.start == 1 {
            self.tree[idx] += val;
            return;
        }

        // 如果当前节点区间完全包含在目标区间内，设置懒惰标记
        if element_range.start >= target_range.start && element_range.end <= target_range.end {
            self.lazy[idx] = match self.lazy[idx] {
                Some(lazy) => Some(lazy + val),
                None => Some(val),
            };
            return;
        }

        // 如果当前节点有非默认的懒惰标记，需要先下传
        if self.lazy[idx].is_some() && self.lazy[idx].unwrap() != T::default() {
            self.propagation(idx, &element_range, T::default());
        }

        // 递归更新左右子树
        let mid = element_range.start + (element_range.end - element_range.start) / 2;
        self.update_recursive(idx * 2, element_range.start..mid, target_range, val);
        self.update_recursive(idx * 2 + 1, mid..element_range.end, target_range, val);
        self.tree[idx] = (self.merge)(self.tree[idx * 2], self.tree[idx * 2 + 1]);
        self.lazy[idx] = Some(T::default());
    }

    /// 传播懒惰标记到子节点
    ///
    /// # 参数
    /// * `idx` - 当前节点索引
    /// * `element_range` - 当前节点表示的区间
    /// * `parent_lazy` - 父节点传递下来的懒惰值
    fn propagation(&mut self, idx: usize, element_range: &Range<usize>, parent_lazy: T) {
        // 叶子节点直接应用懒惰标记
        if element_range.end - element_range.start == 1 {
            self.tree[idx] += parent_lazy;
            return;
        }

        // 获取当前节点的懒惰标记
        let lazy = self.lazy[idx].unwrap_or_default();
        self.lazy[idx] = None;

        // 将懒惰标记传播给子节点
        let mid = element_range.start + (element_range.end - element_range.start) / 2;
        self.propagation(idx * 2, &(element_range.start..mid), parent_lazy + lazy);
        self.propagation(idx * 2 + 1, &(mid..element_range.end), parent_lazy + lazy);
        self.tree[idx] = (self.merge)(self.tree[idx * 2], self.tree[idx * 2 + 1]);
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use quickcheck::TestResult;
    use quickcheck_macros::quickcheck;
    use std::cmp::{max, min};

    #[test]
    fn test_min_segments() {
        let vec = vec![-30, 2, -4, 7, 3, -5, 6, 11, -20, 9, 14, 15, 5, 2, -8];
        let mut min_seg_tree = LazySegmentTree::from_vec(&vec, min);
        // [-30, 2, -4, 7, (3, -5, 6), 11, -20, 9, 14, 15, 5, 2, -8]
        assert_eq!(Some(-5), min_seg_tree.query(4..7));
        // [(-30, 2, -4, 7, 3, -5, 6, 11, -20, 9, 14, 15, 5, 2, -8)]
        assert_eq!(Some(-30), min_seg_tree.query(0..vec.len()));
        // [(-30, 2), -4, 7, 3, -5, 6, 11, -20, 9, 14, 15, 5, 2, -8]
        assert_eq!(Some(-30), min_seg_tree.query(0..2));
        // [-30, (2, -4), 7, 3, -5, 6, 11, -20, 9, 14, 15, 5, 2, -8]
        assert_eq!(Some(-4), min_seg_tree.query(1..3));
        // [-30, (2, -4, 7, 3, -5, 6), 11, -20, 9, 14, 15, 5, 2, -8]
        assert_eq!(Some(-5), min_seg_tree.query(1..7));
    }

    #[test]
    fn test_max_segments() {
        let vec = vec![-30, 2, -4, 7, 3, -5, 6, 11, -20, 9, 14, 15, 5, 2, -8];
        let mut max_seg_tree = LazySegmentTree::from_vec(&vec, max);
        // [-30, 2, -4, 7, (3, -5, 6), 11, -20, 9, 14, 15, 5, 2, -8]
        assert_eq!(Some(6), max_seg_tree.query(4..7));
        // [(-30, 2, -4, 7, 3, -5, 6, 11, -20, 9, 14, 15, 5, 2, -8)]
        assert_eq!(Some(15), max_seg_tree.query(0..vec.len()));
        // [(-30, 2), -4, 7, 3, -5, 6, 11, -20, 9, 14, 15, 5, 2, -8]
        assert_eq!(Some(2), max_seg_tree.query(0..2));
        // [-30, (2, -4), 7, 3, -5, 6, 11, -20, 9, 14, 15, 5, 2, -8]
        assert_eq!(Some(2), max_seg_tree.query(1..3));
        // [-30, (2, -4, 7, 3, -5, 6), 11, -20, 9, 14, 15, 5, 2, -8]
        assert_eq!(Some(7), max_seg_tree.query(1..7));
    }

    #[test]
    fn test_sum_segments() {
        let vec = vec![-30, 2, -4, 7, 3, -5, 6, 11, -20, 9, 14, 15, 5, 2, -8];
        let mut max_seg_tree = LazySegmentTree::from_vec(&vec, |x, y| x + y);
        // [-30, 2, -4, 7, (3, -5, 6), 11, -20, 9, 14, 15, 5, 2, -8]
        assert_eq!(Some(4), max_seg_tree.query(4..7));
        // [(-30, 2, -4, 7, 3, -5, 6, 11, -20, 9, 14, 15, 5, 2, -8)]
        assert_eq!(Some(7), max_seg_tree.query(0..vec.len()));
        // [(-30, 2), -4, 7, 3, -5, 6, 11, -20, 9, 14, 15, 5, 2, -8]
        assert_eq!(Some(-28), max_seg_tree.query(0..2));
        // [-30, (2, -4), 7, 3, -5, 6, 11, -20, 9, 14, 15, 5, 2, -8]
        assert_eq!(Some(-2), max_seg_tree.query(1..3));
        // [-30, (2, -4, 7, 3, -5, 6), 11, -20, 9, 14, 15, 5, 2, -8]
        assert_eq!(Some(9), max_seg_tree.query(1..7));
    }

    #[test]
    fn test_update_segments_tiny() {
        let vec = vec![0, 0, 0, 0, 0];
        let mut update_seg_tree = LazySegmentTree::from_vec(&vec, |x, y| x + y);
        update_seg_tree.update(0..3, 3);
        update_seg_tree.update(2..5, 3);
        assert_eq!(Some(3), update_seg_tree.query(0..1));
        assert_eq!(Some(3), update_seg_tree.query(1..2));
        assert_eq!(Some(6), update_seg_tree.query(2..3));
        assert_eq!(Some(3), update_seg_tree.query(3..4));
        assert_eq!(Some(3), update_seg_tree.query(4..5));
    }

    #[test]
    fn test_update_segments() {
        let vec = vec![-30, 2, -4, 7, 3, -5, 6, 11, -20, 9, 14, 15, 5, 2, -8];
        let mut update_seg_tree = LazySegmentTree::from_vec(&vec, |x, y| x + y);
        // -> [-30, (5, -1, 10, 6), -5, 6, 11, -20, 9, 14, 15, 5, 2, -8]
        update_seg_tree.update(1..5, 3);

        // [-30, 5, -1, 10, (6 -5, 6), 11, -20, 9, 14, 15, 5, 2, -8]
        assert_eq!(Some(7), update_seg_tree.query(4..7));
        // [(-30, 5, -1, 10, 6 , -5, 6, 11, -20, 9, 14, 15, 5, 2, -8)]
        assert_eq!(Some(19), update_seg_tree.query(0..vec.len()));
        // [(-30, 5), -1, 10, 6, -5, 6, 11, -20, 9, 14, 15, 5, 2, -8]
        assert_eq!(Some(-25), update_seg_tree.query(0..2));
        // [-30, (5, -1), 10, 6, -5, 6, 11, -20, 9, 14, 15, 5, 2, -8]
        assert_eq!(Some(4), update_seg_tree.query(1..3));
        // [-30, (5, -1, 10, 6, -5, 6), 11, -20, 9, 14, 15, 5, 2, -8]
        assert_eq!(Some(21), update_seg_tree.query(1..7));
    }

    // Some properties over segment trees:
    //  When asking for the range of the overall array, return the same as iter().min() or iter().max(), etc.
    //  When asking for an interval containing a single value, return this value, no matter the merge function

    #[quickcheck]
    fn check_overall_interval_min(array: Vec<i32>) -> TestResult {
        let mut seg_tree = LazySegmentTree::from_vec(&array, min);
        TestResult::from_bool(array.iter().min().copied() == seg_tree.query(0..array.len()))
    }

    #[quickcheck]
    fn check_overall_interval_max(array: Vec<i32>) -> TestResult {
        let mut seg_tree = LazySegmentTree::from_vec(&array, max);
        TestResult::from_bool(array.iter().max().copied() == seg_tree.query(0..array.len()))
    }

    #[quickcheck]
    fn check_overall_interval_sum(array: Vec<i32>) -> TestResult {
        let mut seg_tree = LazySegmentTree::from_vec(&array, max);
        TestResult::from_bool(array.iter().max().copied() == seg_tree.query(0..array.len()))
    }

    #[quickcheck]
    fn check_single_interval_min(array: Vec<i32>) -> TestResult {
        let mut seg_tree = LazySegmentTree::from_vec(&array, min);
        for (i, value) in array.into_iter().enumerate() {
            let res = seg_tree.query(Range {
                start: i,
                end: i + 1,
            });
            if res != Some(value) {
                return TestResult::error(format!("Expected {:?}, got {:?}", Some(value), res));
            }
        }
        TestResult::passed()
    }

    #[quickcheck]
    fn check_single_interval_max(array: Vec<i32>) -> TestResult {
        let mut seg_tree = LazySegmentTree::from_vec(&array, max);
        for (i, value) in array.into_iter().enumerate() {
            let res = seg_tree.query(Range {
                start: i,
                end: i + 1,
            });
            if res != Some(value) {
                return TestResult::error(format!("Expected {:?}, got {:?}", Some(value), res));
            }
        }
        TestResult::passed()
    }

    #[quickcheck]
    fn check_single_interval_sum(array: Vec<i32>) -> TestResult {
        let mut seg_tree = LazySegmentTree::from_vec(&array, max);
        for (i, value) in array.into_iter().enumerate() {
            let res = seg_tree.query(Range {
                start: i,
                end: i + 1,
            });
            if res != Some(value) {
                return TestResult::error(format!("Expected {:?}, got {:?}", Some(value), res));
            }
        }
        TestResult::passed()
    }
}
