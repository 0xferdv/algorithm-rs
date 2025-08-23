use std::{cmp::Ord, slice::Iter};

/// 堆数据结构，支持自定义比较器实现最小堆或最大堆。
///
/// 泛型参数:
/// - `T`: 堆中存储的元素类型。
pub struct Heap<T> {
    items: Vec<T>,
    comparator: fn(&T, &T) -> bool,
}

impl<T> Heap<T> {

    /// 创建一个新的空堆，并指定比较器。
    ///
    /// 参数:
    /// - `comparator`: 比较函数，用于决定堆的排序方式。
    ///
    /// 返回值:
    /// - 返回一个新创建的堆实例。
    pub fn new(comparator: fn(&T, &T) -> bool) -> Self {
        Self {
            items: vec![],
            comparator,
        }
    }

    /// 从一个向量创建堆，并使用指定的比较器构建堆结构。
    ///
    /// 参数:
    /// - `items`: 初始元素集合。
    /// - `comparator`: 比较函数，用于决定堆的排序方式。
    ///
    /// 返回值:
    /// - 返回一个已构建好的堆实例。
    pub fn from_vec(items: Vec<T>, comparator: fn(&T, &T) -> bool) -> Self {
        let mut heap = Self { items, comparator };
        heap.build_heap();
        heap
    }

    /// 构建堆结构，从最后一个非叶子节点开始向上调整。
    fn build_heap(&mut self) {
        // 最后一个非叶节点索引为 (len / 2) - 1
        let last_parent_idx = (self.len() / 2).wrapping_sub(1);
        for idx in (0..=last_parent_idx).rev() {
            self.heapify_down(idx);
        }
    }

    /// 获取堆中的元素数量。
    ///
    /// 返回值:
    /// - 当前堆中元素的数量。
    pub fn len(&self) -> usize {
        self.items.len()
    }

    /// 判断堆是否为空。
    ///
    /// 返回值:
    /// - 如果堆为空返回 true，否则返回 false。
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// 向堆中添加一个元素，并维护堆性质。
    ///
    /// 参数:
    /// - `value`: 要添加到堆中的元素。
    pub fn add(&mut self, value: T) {
        self.items.push(value);
        self.heapify_up(self.len() - 1);
    }

    /// 弹出堆顶元素，并维护堆性质。
    ///
    /// 返回值:
    /// - 如果堆不为空，返回堆顶元素；否则返回 None。
    pub fn pop(&mut self) -> Option<T> {
        if self.is_empty() {
            return None;
        }
        let next = Some(self.items.swap_remove(0));
        if !self.is_empty() {
            self.heapify_down(0);
        }
        next
    }

    /// 获取堆元素的迭代器。
    ///
    /// 返回值:
    /// - 返回一个指向堆内部元素的只读迭代器。
    pub fn iter(&self) -> Iter<'_, T> {
        self.items.iter()
    }

    /// 将指定索引的元素上浮以维护堆性质。
    ///
    /// 参数:
    /// - `idx`: 需要上浮的元素在堆中的索引。
    fn heapify_up(&mut self, mut idx: usize) {
        while let Some(pdx) = self.parent_idx(idx) {
            if (self.comparator)(&self.items[idx], &self.items[pdx]) {
                self.items.swap(idx, pdx);
                idx = pdx;
            } else {
                break;
            }
        }
    }

    /// 将指定索引的元素下沉以维护堆性质。
    ///
    /// 参数:
    /// - `idx`: 需要下沉的元素在堆中的索引。
    fn heapify_down(&mut self, mut idx: usize) {
        while self.children_present(idx) {
            // 找到左右子节点中满足比较条件的那个节点
            let cdx = {
                if self.right_child_idx(idx) >= self.len() {
                    self.left_child_idx(idx)
                } else {
                    let ldx = self.left_child_idx(idx);
                    let rdx = self.right_child_idx(idx);
                    if (self.comparator)(&self.items[ldx], &self.items[rdx]) {
                        ldx
                    } else {
                        rdx
                    }
                }
            };
            if (self.comparator)(&self.items[cdx], &self.items[idx]) {
                self.items.swap(idx, cdx);
                idx = cdx;
            } else {
                break;
            }
        }
    }

    /// 获取指定索引元素的父节点索引。
    ///
    /// 参数:
    /// - `idx`: 元素索引。
    ///
    /// 返回值:
    /// - 如果存在父节点，返回其索引；否则返回 None。
    fn parent_idx(&self, idx: usize) -> Option<usize> {
        if idx > 0 {
            Some((idx - 1) / 2)
        } else {
            None
        }
    }

    /// 判断指定索引是否有子节点。
    ///
    /// 参数:
    /// - `idx`: 元素索引。
    ///
    /// 返回值:
    /// - 如果有子节点返回 true，否则返回 false。
    fn children_present(&self, idx: usize) -> bool {
        self.left_child_idx(idx) < self.len()
    }

    /// 计算左子节点索引。
    ///
    /// 参数:
    /// - `idx`: 父节点索引。
    ///
    /// 返回值:
    /// - 左子节点的索引。
    fn left_child_idx(&self, idx: usize) -> usize {
        idx * 2 + 1
    }

    /// 计算右子节点索引。
    ///
    /// 参数:
    /// - `idx`: 父节点索引。
    ///
    /// 返回值:
    /// - 右子节点的索引。
    fn right_child_idx(&self, idx: usize) -> usize {
        self.left_child_idx(idx) + 1
    }
}

impl<T> Heap<T>
where
    T: Ord
{
    /// 创建一个新的最小堆。
    ///
    /// 返回值:
    /// - 返回一个使用默认最小堆比较器的新堆实例。
    pub fn new_min() -> Heap<T> {
        Self::new(|a, b|  a < b)
    }

    /// 创建一个新的最大堆。
    ///
    /// 返回值:
    /// - 返回一个使用默认最大堆比较器的新堆实例。
    pub fn new_max() -> Heap<T> {
        Self::new(|a, b| a > b)
    }

    /// 从一个向量创建最小堆。
    ///
    /// 参数:
    /// - `items`: 初始化堆的元素集合。
    ///
    /// 返回值:
    /// - 返回一个已构建完成的最小堆实例。
    pub fn from_vec_min(items: Vec<T>) -> Heap<T> {
        Self::from_vec(items, |a, b| a < b)
    }

    /// 从一个向量创建最大堆。
    ///
    /// 参数:
    /// - `items`: 初始化堆的元素集合。
    ///
    /// 返回值:
    /// - 返回一个已构建完成的最大堆实例。
    pub fn from_vec_max(items: Vec<T>) -> Heap<T> {
        Self::from_vec(items, |a, b| a > b)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_heap() {
        let mut heap: Heap<i32> = Heap::new_max();
        assert_eq!(heap.pop(), None);
    }

    #[test]
    fn test_min_heap() {
        let mut heap = Heap::new_min();
        heap.add(4);
        heap.add(2);
        heap.add(9);
        heap.add(11);
        assert_eq!(heap.len(), 4);
        assert_eq!(heap.pop(), Some(2));
        assert_eq!(heap.pop(), Some(4));
        assert_eq!(heap.pop(), Some(9));
        heap.add(1);
        assert_eq!(heap.pop(), Some(1));
        assert_eq!(heap.pop(), Some(11));
        assert_eq!(heap.pop(), None);
    }

    #[test]
    fn test_max_heap() {
        let mut heap = Heap::new_max();
        heap.add(4);
        heap.add(2);
        heap.add(9);
        heap.add(11);
        assert_eq!(heap.len(), 4);
        assert_eq!(heap.pop(), Some(11));
        assert_eq!(heap.pop(), Some(9));
        assert_eq!(heap.pop(), Some(4));
        heap.add(1);
        assert_eq!(heap.pop(), Some(2));
        assert_eq!(heap.pop(), Some(1));
        assert_eq!(heap.pop(), None);
    }

    #[test]
    fn test_iter_heap() {
        let mut heap = Heap::new_min();
        heap.add(4);
        heap.add(2);
        heap.add(9);
        heap.add(11);

        let mut iter = heap.iter();
        assert_eq!(iter.next(), Some(&2));
        assert_eq!(iter.next(), Some(&4));
        assert_eq!(iter.next(), Some(&9));
        assert_eq!(iter.next(), Some(&11));
        assert_eq!(iter.next(), None);

        assert_eq!(heap.len(), 4);
        assert_eq!(heap.pop(), Some(2));
        assert_eq!(heap.pop(), Some(4));
        assert_eq!(heap.pop(), Some(9));
        assert_eq!(heap.pop(), Some(11));
        assert_eq!(heap.pop(), None);
    }

    #[test]
    fn test_from_vec_min() {
        let vec = vec![3, 1, 4, 1, 5, 9, 2, 6, 5];
        let mut heap = Heap::from_vec_min(vec);
        assert_eq!(heap.len(), 9);
        assert_eq!(heap.pop(), Some(1));
        assert_eq!(heap.pop(), Some(1));
        assert_eq!(heap.pop(), Some(2));
        heap.add(0);
        assert_eq!(heap.pop(), Some(0));
    }

    #[test]
    fn test_from_vec_max() {
        let vec = vec![3, 1, 4, 1, 5, 9, 2, 6, 5];
        let mut heap = Heap::from_vec_max(vec);
        assert_eq!(heap.len(), 9);
        assert_eq!(heap.pop(), Some(9));
        assert_eq!(heap.pop(), Some(6));
        assert_eq!(heap.pop(), Some(5));
        heap.add(10);
        assert_eq!(heap.pop(), Some(10));
    }
}
