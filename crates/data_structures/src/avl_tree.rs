use std::{
    cmp::{max, Ordering},
    iter::FromIterator,
    mem,
    ops::Not,
};


/// AVL树的节点结构体
///
/// # 字段
/// * `value` - 节点存储的值
/// * `height` - 节点的高度，用于维护AVL树的平衡性
/// * `left` - 左子树
/// * `right` - 右子树
struct AVLNode<T: Ord> {
    value: T,
    height: usize,
    left: Option<Box<AVLNode<T>>>,
    right: Option<Box<AVLNode<T>>>,
}

/// AVL树结构体，实现自平衡二叉搜索树
///
/// # 字段
/// * `root` - 树的根节点
/// * `length` - 树中元素的数量
pub struct AVLTree<T: Ord> {
    root: Option<Box<AVLNode<T>>>,
    length: usize,
}


/// 表示树的左右子树方向的枚举
#[derive(Clone, Copy)]
enum Side {
    Left,
    Right,
}

impl<T: Ord> AVLTree<T> {

    /// 创建一个新的空AVL树
    ///
    /// # 返回值
    /// 返回一个空的AVLTree实例
    pub fn new() -> Self {
        Self {
            root: None,
            length: 0,
        }
    }

    /// 检查树中是否包含指定值
    ///
    /// # 参数
    /// * `value` - 要查找的值的引用
    ///
    /// # 返回值
    /// 如果树中包含该值返回true，否则返回false
    pub fn contains(&self, value: &T) -> bool {
        let mut current = &self.root;
        while let Some(node) = current {
            current = match value.cmp(&node.value) {
                Ordering::Equal => return true,
                Ordering::Less => &node.left,
                Ordering::Greater => &node.right,
            }
        }
        false
    }

    /// 向树中插入一个值
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

    /// 从树中删除指定值
    ///
    /// # 参数
    /// * `value` - 要删除的值的引用
    ///
    /// # 返回值
    /// 如果删除成功返回true，如果值不存在返回false
    pub fn remove(&mut self, value: &T) -> bool {
        let removed = remove(&mut self.root, value);
        if removed {
            self.length -= 1;
        }
        removed
    }

    /// 获取树中元素的数量
    ///
    /// # 返回值
    /// 返回树中元素的数量
    pub fn len(&self) -> usize {
        self.length
    }

    /// 检查树是否为空
    ///
    /// # 返回值
    /// 如果树为空返回true，否则返回false
    pub fn is_empty(&self) -> bool {
        self.length == 0
    }

    /// 创建节点迭代器，用于遍历树中的节点
    ///
    /// # 返回值
    /// 返回一个NodeIter实例，按中序遍历顺序访问节点
    fn node_iter(&self) -> NodeIter<T> {
        let cap = self.root.as_ref().map_or(0, |n| n.height);
        let mut node_iter = NodeIter {
            stack: Vec::with_capacity(cap),
        };
        let mut child = &self.root;
        while let Some(node) = child {
            node_iter.stack.push(node.as_ref());
            child = &node.left;
        }
        node_iter
    }

    /// 创建值迭代器，用于遍历树中的值
    ///
    /// # 返回值
    /// 返回一个Iter实例，按中序遍历顺序访问值
    pub fn iter(&self) -> Iter<T> {
        Iter {
            node_iter: self.node_iter(),
        }
    }
}

/// 在AVL树中插入一个值的内部函数
///
/// # 参数
/// * `tree` - AVL树节点的可变引用
/// * `value` - 要插入的值
///
/// # 返回值
/// 如果插入成功返回true，如果值已存在返回false
fn insert<T: Ord>(tree: &mut Option<Box<AVLNode<T>>>, value: T) -> bool {
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
        *tree = Some(Box::new(AVLNode {
            value,
            height: 1,
            left: None,
            right: None,
        }));
        true
    }
}

/// 从AVL树中删除指定值的内部函数
///
/// # 参数
/// * `tree` - AVL树节点的可变引用
/// * `value` - 要删除的值的引用
///
/// # 返回值
/// 如果删除成功返回true，如果值不存在返回false
fn remove<T: Ord>(tree: &mut Option<Box<AVLNode<T>>>, value: &T) -> bool {
    if let Some(node) = tree {
        let removed = match value.cmp(&node.value) {
            Ordering::Less => remove(&mut node.left, value),
            Ordering::Greater => remove(&mut node.right, value),
            Ordering::Equal => {
                *tree = match (node.left.take(), node.right.take()) {
                    (None, None) => None,
                    (Some(b), None) | (None, Some(b)) => Some(b),
                    (Some(left), Some(right)) => Some(merge(left, right)),
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

/// 合并两个AVL子树的函数
///
/// # 参数
/// * `left` - 左子树
/// * `right` - 右子树
///
/// # 返回值
/// 返回合并后的AVL树节点
fn merge<T: Ord>(left: Box<AVLNode<T>>, right: Box<AVLNode<T>>) -> Box<AVLNode<T>> {
    let mut op_right = Some(right);
    let mut root = take_min(&mut op_right).unwrap();
    root.left = Some(left);
    root.right = op_right;
    root.rebalance();
    root
}

/// 从AVL树中取出最小值节点的函数
///
/// # 参数
/// * `tree` - AVL树节点的可变引用
///
/// # 返回值
/// 返回最小值节点，如果树为空返回None
fn take_min<T: Ord>(tree: &mut Option<Box<AVLNode<T>>>) -> Option<Box<AVLNode<T>>> {
    if let Some(mut node) = tree.take() {
        if let Some(small) = take_min(&mut node.left) {
            node.rebalance();
            *tree = Some(node);
            Some(small)
        } else {
            *tree = node.right.take();
            Some(node)
        }
    } else {
        None
    }
}

impl<T: Ord> AVLNode<T> {

    /// 获取指定方向的子树
    ///
    /// # 参数
    /// * `side` - 子树方向（左或右）
    ///
    /// # 返回值
    /// 返回指定方向子树的引用
    fn child(&self, side: Side) -> &Option<Box<AVLNode<T>>> {
        match side {
            Side::Left => &self.left,
            Side::Right => &self.right,
        }
    }

    /// 获取指定方向的子树的可变引用
    ///
    /// # 参数
    /// * `side` - 子树方向（左或右）
    ///
    /// # 返回值
    /// 返回指定方向子树的可变引用
    fn child_mut(&mut self, side: Side) -> &mut Option<Box<AVLNode<T>>> {
        match side {
            Side::Left => &mut self.left,
            Side::Right => &mut self.right,
        }
    }

    /// 获取指定方向子树的高度
    ///
    /// # 参数
    /// * `side` - 子树方向（左或右）
    ///
    /// # 返回值
    /// 返回指定方向子树的高度，如果子树为空返回0
    fn height(&self, side: Side) -> usize {
        self.child(side).as_ref().map_or(0, |n| n.height)
    }

    /// 计算节点的平衡因子
    ///
    /// # 返回值
    /// 返回平衡因子，右子树高度减去左子树高度
    fn balance_factor(&self) -> i8 {
        let (left, right) = (self.height(Side::Left), self.height(Side::Right));
        if left < right {
            (right - left) as i8
        } else {
            -((left - right) as i8)
        }
    }

    /// 更新节点的高度
    fn update_height(&mut self) {
        self.height = 1 + max(self.height(Side::Left), self.height(Side::Right));
    }

    /// 对节点进行旋转操作以维护AVL树的平衡
    ///
    /// # 参数
    /// * `side` - 旋转方向
    fn rotate(&mut self, side: Side) {
        let mut subtree = self.child_mut(!side).take().unwrap();
        *self.child_mut(!side) = subtree.child_mut(side).take();
        self.update_height();
        mem::swap(self, subtree.as_mut());
        *self.child_mut(side) = Some(subtree);
        self.update_height();
    }

    /// 重新平衡节点以维护AVL树的性质
    fn rebalance(&mut self) {
        self.update_height();
        let side = match self.balance_factor() {
            -2 => Side::Left,
            2 => Side::Right,
            _ => return,
        };
        let subtree = self.child_mut(side).as_mut().unwrap();
        if let (Side::Left, 1) | (Side::Right, -1) = (side, subtree.balance_factor()) {
            subtree.rotate(side);
        }
        self.rotate(!side);
    }
}

impl<T: Ord> Default for AVLTree<T> {
    /// 创建AVL树的默认实例
    ///
    /// # 返回值
    /// 返回一个空的AVLTree实例
    fn default() -> Self { Self::new() }
}

impl Not for Side {
    type Output = Side;

    /// 对Side枚举取反操作
    ///
    /// # 参数
    /// * `self` - 当前的Side值
    ///
    /// # 返回值
    /// 返回相反方向的Side值
    fn not(self) -> Self::Output {
        match self {
            Self::Left => Side::Right,
            Self::Right => Side::Left,
        }
    }
}

impl<T: Ord> FromIterator<T> for AVLTree<T> {
    /// 从迭代器创建AVL树
    ///
    /// # 参数
    /// * `iter` - 实现IntoIterator trait的迭代器
    ///
    /// # 返回值
    /// 返回包含迭代器中所有元素的AVLTree实例
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        let mut tree = AVLTree::new();
        for value in iter {
            tree.insert(value);
        }
        tree
    }
}

/// AVL树节点的迭代器结构体
///
/// # 字段
/// * `stack` - 用于迭代的栈，存储节点引用
struct NodeIter<'a, T: Ord> {
    stack: Vec<&'a AVLNode<T>>,
}

impl<'a, T: Ord> Iterator for NodeIter<'a, T> {
    type Item = &'a AVLNode<T>;

    /// 获取下一个节点
    ///
    /// # 返回值
    /// 返回下一个节点的引用，如果没有更多节点返回None
    fn next(&mut self) -> Option<Self::Item> {
        if let Some(node) = self.stack.pop() {
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

/// AVL树值的迭代器结构体
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
    /// 返回下一个值的引用，如果没有更多值返回None
    fn next(&mut self) -> Option<&'a T> {
        match self.node_iter.next() {
            Some(node ) => Some(&node.value),
            None => None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::AVLTree;

    /// 检查AVL树是否平衡的辅助函数
    ///
    /// # 参数
    /// * `tree` - 要检查的AVL树
    ///
    /// # 返回值
    /// 如果树平衡返回true，否则返回false
    fn is_balanced<T: Ord>(tree: &AVLTree<T>) -> bool {
        tree.node_iter()
            .all(|n| (-1..=1).contains(&n.balance_factor()))
    }

    #[test]
    fn len() {
        let tree: AVLTree<_> = (1..4).collect();
        assert_eq!(tree.len(), 3);
    }

    #[test]
    fn contains() {
        let tree: AVLTree<_> = (1..4).collect();
        assert!(tree.contains(&1));
        assert!(!tree.contains(&4));
    }

    #[test]
    fn insert() {
        let mut tree = AVLTree::new();
        assert!(tree.insert(1));
        assert!(!tree.insert(1));
    }

    #[test]
    fn remove() {
        let mut tree: AVLTree<_> = (1..8).collect();
        assert!(tree.remove(&4));
        assert!(!tree.remove(&4));
    }

    #[test]
    fn sorted() {
        let tree: AVLTree<_> = (1..8).rev().collect();
        assert!((1..8).eq(tree.iter().copied()));
    }

    #[test]
    fn balanced() {
        let mut tree: AVLTree<_> = (1..8).collect();
        assert!(is_balanced(&tree));
        for x in 1..8 {
            tree.remove(&x);
            assert!(is_balanced(&tree));
        }
    }
}
