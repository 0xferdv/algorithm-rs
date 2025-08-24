use std::{
    mem,
    ops::Not,
    cmp::Ordering,
    iter::FromIterator,
    time::{SystemTime, UNIX_EPOCH},
};

/// Treap节点结构体，表示Treap中单个节点
///
/// # 字段
/// * `value` - 节点存储的值
/// * `priority` - 节点的优先级，用于维持堆性质
/// * `left` - 左子树
/// * `right` - 右子树
struct TreapNode<T: Ord> {
    value: T,
    priority: usize,
    left: Option<Box<TreapNode<T>>>,
    right: Option<Box<TreapNode<T>>>,
}


/// Treap数据结构，实现了一个同时满足二叉搜索树和堆性质的平衡二叉树
///
/// Treap结合了二叉搜索树和堆的特性：
/// - 二叉搜索树性质：对于任意节点，左子树所有节点值小于该节点值，右子树所有节点值大于该节点值
/// - 堆性质：每个节点的优先级大于等于其子节点的优先级
///
/// # 字段
/// * `root` - Treap的根节点
/// * `length` - Treap中元素的数量
pub struct Treap<T: Ord> {
    root: Option<Box<TreapNode<T>>>,
    length: usize,
}


/// 表示树的左右方向的枚举
///
/// 用于在树操作中指定方向
#[derive(Clone, Copy)]
enum Side {
    Left,
    Right,
}


impl<T: Ord> Treap<T> {

    /// 创建一个新的空Treap
    ///
    /// # 返回值
    /// 返回一个空的Treap实例
    pub fn new() -> Treap<T> {
        Treap {
            root: None,
            length: 0,
        }
    }

    /// 检查Treap中是否包含指定值
    ///
    /// # 参数
    /// * `value` - 要查找的值的引用
    ///
    /// # 返回值
    /// 如果Treap中包含该值则返回true，否则返回false
    pub fn contains(&self, value: &T) -> bool {
        let mut curr = &self.root;
        while let Some(node) = curr {
            curr = match value.cmp(&node.value) {
                Ordering::Equal => return true,
                Ordering::Less => &node.left,
                Ordering::Greater => &node.right,
            }
        }
        false
    }

    /// 向Treap中插入一个值
    ///
    /// # 参数
    /// * `value` - 要插入的值
    ///
    /// # 返回值
    /// 如果插入成功返回true，如果值已存在返回false
    pub fn insert(&mut self, value: T) -> bool {
        let inserted = insert(&mut self.root, value);
        if inserted {
            self.length += 1;
        }
        inserted
    }

    /// 从Treap中删除指定值
    ///
    /// # 参数
    /// * `value` - 要删除的值的引用
    ///
    /// # 返回值
    /// 如果删除成功返回true，如果值不存在返回false
    pub fn remove(&mut self, value: &T) -> bool {
        let removed = remove(&mut self.root, value);
        if removed{
            self.length -= 1;
        }
        removed
    }

    /// 获取Treap中元素的数量
    ///
    /// # 返回值
    /// 返回Treap中存储的元素数量
    pub fn len(&self) -> usize {
        self.length
    }

    /// 检查Treap是否为空
    ///
    /// # 返回值
    /// 如果Treap为空返回true，否则返回false
    pub fn is_empty(&self) -> bool {
        self.length == 0
    }

    /// 创建节点迭代器，用于中序遍历
    ///
    /// # 返回值
    /// 返回一个NodeIter实例，用于遍历Treap节点
    fn node_iter(&self) -> NodeIter<T> {
        let mut node_iter = NodeIter { stack: Vec::new() };
        let mut child = &self.root;
        while let Some(node) = child {
            node_iter.stack.push(node.as_ref());
            child = &node.left;
        }
        node_iter
    }

    /// 创建值迭代器，用于中序遍历Treap中的值
    ///
    /// # 返回值
    /// 返回一个Iter实例，用于遍历Treap中的值
    pub fn iter(&self) -> Iter<T> {
        Iter {
            node_iter: self.node_iter(),
        }
    }
}

/// 生成随机优先级的辅助函数
///
/// 使用系统时间的纳秒部分作为随机数种子
///
/// # 返回值
/// 返回一个usize类型的随机数
fn rand() -> usize {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .subsec_nanos() as usize
}

/// 向Treap中插入节点的递归辅助函数
///
/// # 参数
/// * `tree` - 指向Treap节点的可变引用
/// * `value` - 要插入的值
///
/// # 返回值
/// 如果插入成功返回true，如果值已存在返回false
fn insert<T: Ord>(tree: &mut Option<Box<TreapNode<T>>>, value: T) -> bool {
    if let Some(node) = tree {
        let inserted = match value.cmp(&node.value) {
            Ordering::Equal => false,
            Ordering::Less => insert(&mut node.left, value),
            Ordering::Greater => insert(&mut node.right, value),
        };
        if inserted {
            node.rebalance();
        }
        inserted
    } else {
        *tree = Some(Box::new(
            TreapNode {
                value,
                priority: rand(),
                left: None,
                right: None,
            }
        ));
        true
    }
}

/// 从Treap中删除节点的递归辅助函数
///
/// # 参数
/// * `tree` - 指向Treap节点的可变引用
/// * `value` - 要删除的值的引用
///
/// # 返回值
/// 如果删除成功返回true，如果值不存在返回false
fn remove<T: Ord>(tree: &mut Option<Box<TreapNode<T>>>, value: &T) -> bool {
    if let Some(node) = tree {
        let removed = match value.cmp(&node.value) {
            Ordering::Less => remove(&mut node.left, value),
            Ordering::Greater => remove(&mut node.right, value),
            Ordering::Equal => {
                *tree = match (node.left.take(), node.right.take()) {
                    (None, None) => None,
                    (Some(b), None) | (None, Some(b)) => Some(b),
                    (Some(left), Some(right)) => {
                        let side = match left.priority.cmp(&right.priority) {
                            Ordering::Greater => Side::Right,
                            _ => Side::Left,
                        };
                        node.left = Some(left);
                        node.right = Some(right);
                        node.rotate(side);
                        remove(node.child_mut(side), value);
                        Some(tree.take().unwrap())
                    }
                };
                return true;
            }
        };
        if removed {
            node.rebalance();
        }
        removed
    } else {
        false
    }
}

impl<T: Ord> TreapNode<T> {

    /// 获取指定方向的子节点引用
    ///
    /// # 参数
    /// * `side` - 指定方向（左或右）
    ///
    /// # 返回值
    /// 返回指定方向子节点的引用
    fn child(&self, side: Side) -> &Option<Box<TreapNode<T>>> {
        match side {
            Side::Left => &self.left,
            Side::Right => &self.right,
        }
    }

    /// 获取指定方向的子节点可变引用
    ///
    /// # 参数
    /// * `side` - 指定方向（左或右）
    ///
    /// # 返回值
    /// 返回指定方向子节点的可变引用
    fn child_mut(&mut self, side: Side) -> &mut Option<Box<TreapNode<T>>> {
        match side {
            Side::Left => &mut self.left,
            Side::Right => &mut self.right,
        }
    }

    /// 获取指定方向子节点的优先级
    ///
    /// # 参数
    /// * `side` - 指定方向（左或右）
    ///
    /// # 返回值
    /// 返回指定方向子节点的优先级，如果子节点不存在则返回0
    fn priority(&self, side: Side) -> usize {
        self.child(side).as_ref().map_or(0, |node| node.priority)
    }

    /// 对节点进行旋转操作以维持Treap性质
    ///
    /// # 参数
    /// * `side` - 旋转方向，Left表示左旋，Right表示右旋
    fn rotate(&mut self, side: Side) {
        if self.child_mut(!side).is_none() {
            return;
        }
        let mut subtree = self.child_mut(!side).take().unwrap();
        *self.child_mut(!side) = subtree.child_mut(side).take();
        mem::swap(self, subtree.as_mut());
        *self.child_mut(side) = Some(subtree);
    }

    /// 重新平衡节点以维持Treap的堆性质
    ///
    /// 检查当前节点与其子节点的优先级关系，必要时进行旋转操作
    fn rebalance(&mut self) {
        match (
            self.priority,
            self.priority(Side::Left),
            self.priority(Side::Right),
        ) {
            (v, p, q) if p >= q && p > v => self.rotate(Side::Right),
            (v, p, q) if p < q && q > v => self.rotate(Side::Left),
            _ => (),
        };
    }

    #[cfg(test)]
    fn is_valid(&self) -> bool {
        self.priority >= self.priority(Side::Left) && self.priority >= self.priority(Side::Right)
    }
}


impl<T: Ord> Default for Treap<T> {

    /// 创建Treap的默认实例
    ///
    /// # 返回值
    /// 返回一个空的Treap实例
    fn default() -> Self {
        Self::new()
    }
}

impl Not for Side {

    type Output = Side;

    /// 对Side枚举进行取反操作
    ///
    /// # 参数
    /// * `self` - 当前的Side值
    ///
    /// # 返回值
    /// 返回相反方向的Side值
    fn not(self) -> Self::Output {
        match self {
            Side::Left => Side::Right,
            Side::Right => Side::Left,
        }
    }
}


impl<T: Ord> FromIterator<T> for Treap<T> {

    /// 从迭代器创建Treap
    ///
    /// # 参数
    /// * `iter` - 实现IntoIterator trait的迭代器
    ///
    /// # 返回值
    /// 返回包含迭代器中所有元素的Treap实例
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        let mut tree = Treap::new();
        for value in iter {
            tree.insert(value);
        }
        tree
    }
}

/// Treap节点迭代器，用于中序遍历
///
/// 使用栈来实现非递归的中序遍历
///
/// # 字段
/// * `stack` - 用于遍历的栈
struct NodeIter<'a, T: Ord> {
    stack: Vec<&'a TreapNode<T>>,
}

impl<'a, T: Ord> Iterator for NodeIter<'a, T> {
    type Item = &'a TreapNode<T>;

    /// 获取下一个节点
    ///
    /// # 返回值
    /// 返回下一个节点的引用，如果没有更多节点则返回None
    fn next(&mut self) -> Option<Self::Item> {
        if let Some(node) = self.stack.pop() {
            // Push left path of right subtree to stack
            let mut child = &node.right;
            while let Some(subtree) = child {
                self.stack.push(subtree.as_ref());
                child = &subtree.left;
            }
            Some(node)
        } else {
            None
        }
    }
}

/// Treap值迭代器，用于遍历Treap中的值
///
/// 包装NodeIter以提供对值的访问
///
/// # 字段
/// * `node_iter` - 底层的节点迭代器
pub struct Iter<'a, T: Ord> {
    node_iter: NodeIter<'a, T>,
}

impl<'a, T: Ord> Iterator for Iter<'a, T> {
    type Item = &'a T;

    /// 获取下一个值
    ///
    /// # 返回值
    /// 返回下一个值的引用，如果没有更多值则返回None
    fn next(&mut self) -> Option<&'a T> {
        match self.node_iter.next() {
            Some(node) => Some(&node.value),
            None => None,
        }
    }
}


#[cfg(test)]
mod tests {
    use super::Treap;

    /// Returns `true` if all nodes in the tree are valid.
    fn is_valid<T: Ord>(tree: &Treap<T>) -> bool {
        tree.node_iter().all(|n| n.is_valid())
    }

    #[test]
    fn len() {
        let tree: Treap<_> = (1..4).collect();
        assert_eq!(tree.len(), 3);
    }

    #[test]
    fn contains() {
        let tree: Treap<_> = (1..4).collect();
        assert!(tree.contains(&1));
        assert!(!tree.contains(&4));
    }

    #[test]
    fn insert() {
        let mut tree = Treap::new();
        // First insert succeeds
        assert!(tree.insert(1));
        // Second insert fails
        assert!(!tree.insert(1));
    }

    #[test]
    fn remove() {
        let mut tree: Treap<_> = (1..8).collect();
        // First remove succeeds
        assert!(tree.remove(&4));
        // Second remove fails
        assert!(!tree.remove(&4));
    }

    #[test]
    fn sorted() {
        let tree: Treap<_> = (1..8).rev().collect();
        assert!((1..8).eq(tree.iter().copied()));
    }

    #[test]
    fn valid() {
        let mut tree: Treap<_> = (1..8).collect();
        assert!(is_valid(&tree));
        for x in 1..8 {
            tree.remove(&x);
            assert!(is_valid(&tree));
        }
    }
}
