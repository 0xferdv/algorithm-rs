use std::cmp::Ordering;
use std::ops::Deref;

/// 二叉搜索树的实现
///
/// 该结构体表示一个二叉搜索树，其中每个节点包含一个可选值以及左右子树。
/// 树中的元素必须实现 `Ord` trait 以便进行比较操作。
pub struct BinarySearchTree<T>
where
    T: Ord
{
    value: Option<T>,
    left: Option<Box<BinarySearchTree<T>>>,
    right: Option<Box<BinarySearchTree<T>>>,
}

impl<T> Default for BinarySearchTree<T>
where
    T: Ord
{
    /// 创建一个新的空二叉搜索树
    ///
    /// # 返回值
    /// 返回一个初始化为空的二叉搜索树实例
    fn default() -> Self { Self::new() }
}

impl<T> BinarySearchTree<T>
where
    T: Ord
{
    /// 创建一个新的空二叉搜索树
    ///
    /// # 返回值
    /// 返回一个初始化为空的二叉搜索树实例
    pub fn new() -> BinarySearchTree<T> {
        BinarySearchTree {
            value: None,
            left: None,
            right: None,
        }
    }

    /// 在二叉搜索树中查找指定值
    ///
    /// # 参数
    /// * `value` - 要查找的值的引用
    ///
    /// # 返回值
    /// 如果找到该值则返回 true，否则返回 false
    pub fn search(&self, value: &T) -> bool {
        match &self.value {
            Some(key) => {
                match key.cmp(value) {
                    Ordering::Equal => true,
                    Ordering::Greater => {
                        match &self.left {
                            Some(node) => node.search(value),
                            None => false,
                        }
                    }
                    Ordering::Less => {
                        match &self.right {
                            Some(node) => node.search(value),
                            None => false,
                        }
                    }
                }
            }
            None => false,
        }
    }

    /// 获取二叉搜索树的迭代器
    ///
    /// # 返回值
    /// 返回一个按中序遍历顺序访问树中元素的迭代器
    pub fn iter(&self) -> impl Iterator<Item = &T>{
        BinarySearchTreeIter::new(self)
    }

    /// 向二叉搜索树中插入一个新值
    ///
    /// # 参数
    /// * `value` - 要插入的值
    pub fn insert(&mut self, value: T) {
        match &self.value {
            None => self.value = Some(value),
            Some(key) => {
                let target_node = if value < *key {
                    &mut self.left
                } else {
                    &mut self.right
                };
                match target_node {
                    Some(node) => {
                        node.insert(value);
                    }
                    None => {
                        let mut node = BinarySearchTree::new();
                        node.value = Some(value);
                        *target_node = Some(Box::new(node));
                    }
                }
            }
        }
    }

    /// 查找二叉搜索树中的最小值
    ///
    /// # 返回值
    /// 返回树中最小值的引用，如果树为空则返回 None
    pub fn minimum(&self) -> Option<&T> {
        match &self.left {
            Some(node) => node.minimum(),
            None => self.value.as_ref(),
        }
    }

    /// 查找二叉搜索树中的最大值
    ///
    /// # 返回值
    /// 返回树中最大值的引用，如果树为空则返回 None
    pub fn maximum(&self) -> Option<&T> {
        match &self.right {
            Some(node) => node.maximum(),
            None => self.value.as_ref(),
        }
    }

    /// 查找不大于给定值的最大元素（向下取整）
    ///
    /// # 参数
    /// * `value` - 用于比较的目标值的引用
    ///
    /// # 返回值
    /// 返回不大于给定值的最大元素的引用，如果不存在则返回 None
    pub fn floor(&self, value: &T) -> Option<&T> {
        match &self.value {
            Some(key) => {
                match key.cmp(value) {
                    Ordering::Greater => {
                        match &self.left {
                            Some(node) => node.floor(value),
                            None => None,
                        }
                    }
                    Ordering::Less => {
                        match &self.right {
                            Some(node) => {
                                let val = node.floor(value);
                                match val {
                                    Some(_) => val,
                                    None => Some(key),
                                }
                            }
                            None => Some(key),
                        }
                    }
                    Ordering::Equal => Some(key),
                }
            }
            None => None,
        }
    }

    /// 查找不小于给定值的最小元素（向上取整）
    ///
    /// # 参数
    /// * `value` - 用于比较的目标值的引用
    ///
    /// # 返回值
    /// 返回不小于给定值的最小元素的引用，如果不存在则返回 None
    pub fn ceil(&self, value: &T) -> Option<&T> {
        match &self.value {
            Some(key) => {
                match key.cmp(value) {
                    Ordering::Less => {
                        match &self.right {
                            Some(node) => node.ceil(value),
                            None => None,
                        }
                    }
                    Ordering::Greater => {
                        match &self.left {
                            Some(node) => {
                                let val = node.ceil(value);
                                match val {
                                    Some(_) => val,
                                    None => Some(key),
                                }
                            }
                            None => Some(key),
                        }
                    }
                    Ordering::Equal => Some(key),
                }
            }
            None => None,
        }
    }
}

/// 二叉搜索树的迭代器结构体
///
/// 用于实现二叉搜索树的中序遍历迭代器
struct BinarySearchTreeIter<'a, T>
where
    T: Ord
{
    stack: Vec<&'a BinarySearchTree<T>>,
}

impl<T> BinarySearchTreeIter<'_, T>
where
    T: Ord
{
    /// 创建一个新的二叉搜索树迭代器
    ///
    /// # 参数
    /// * `tree` - 要遍历的二叉搜索树的引用
    ///
    /// # 返回值
    /// 返回一个新的迭代器实例
    pub fn new(tree: &BinarySearchTree<T>) -> BinarySearchTreeIter<T> {
        let mut iter = BinarySearchTreeIter {
            stack: vec![tree],
        };
        iter.stack_push_left();
        iter
    }

    /// 将左子树路径上的所有节点压入栈中
    ///
    /// 这个辅助函数用于构建中序遍历的栈结构
    fn stack_push_left(&mut self) {
        while let Some(child) = &self.stack.last().unwrap().left {
            self.stack.push(child);
        }
    }
}

impl<'a, T> Iterator for BinarySearchTreeIter<'a, T>
where
    T: Ord
{
    type Item = &'a T;

    /// 获取下一个元素
    ///
    /// # 返回值
    /// 返回下一个元素的引用，如果没有更多元素则返回 None
    fn next(&mut self) -> Option<&'a T> {
        if self.stack.is_empty() {
            None
        } else {
            let node = self.stack.pop().unwrap();
            if node.right.is_some() {
                self.stack.push(node.right.as_ref().unwrap().deref());
                self.stack_push_left();
            }
            node.value.as_ref()
        }
    }
}

#[cfg(test)]
mod test {
    use super::BinarySearchTree;

    fn prequel_memes_tree() -> BinarySearchTree<&'static str> {
        let mut tree = BinarySearchTree::new();
        tree.insert("hello there");
        tree.insert("general kenobi");
        tree.insert("you are a bold one");
        tree.insert("kill him");
        tree.insert("back away...I will deal with this jedi slime myself");
        tree.insert("your move");
        tree.insert("you fool");
        tree
    }

    #[test]
    fn test_search() {
        let tree = prequel_memes_tree();
        assert!(tree.search(&"hello there"));
        assert!(tree.search(&"you are a bold one"));
        assert!(tree.search(&"general kenobi"));
        assert!(tree.search(&"you fool"));
        assert!(tree.search(&"kill him"));
        assert!(
            !tree.search(&"but i was going to tosche station to pick up some power converters",)
        );
        assert!(!tree.search(&"only a sith deals in absolutes"));
        assert!(!tree.search(&"you underestimate my power"));
    }

    #[test]
    fn test_maximum_and_minimum() {
        let tree = prequel_memes_tree();
        assert_eq!(*tree.maximum().unwrap(), "your move");
        assert_eq!(
            *tree.minimum().unwrap(),
            "back away...I will deal with this jedi slime myself"
        );
        let mut tree2: BinarySearchTree<i32> = BinarySearchTree::new();
        assert!(tree2.maximum().is_none());
        assert!(tree2.minimum().is_none());
        tree2.insert(0);
        assert_eq!(*tree2.minimum().unwrap(), 0);
        assert_eq!(*tree2.maximum().unwrap(), 0);
        tree2.insert(-5);
        assert_eq!(*tree2.minimum().unwrap(), -5);
        assert_eq!(*tree2.maximum().unwrap(), 0);
        tree2.insert(5);
        assert_eq!(*tree2.minimum().unwrap(), -5);
        assert_eq!(*tree2.maximum().unwrap(), 5);
    }

    #[test]
    fn test_floor_and_ceil() {
        let tree = prequel_memes_tree();
        assert_eq!(*tree.floor(&"hello there").unwrap(), "hello there");
        assert_eq!(
            *tree
                .floor(&"these are not the droids you're looking for")
                .unwrap(),
            "kill him"
        );
        assert!(tree.floor(&"another death star").is_none());
        assert_eq!(*tree.floor(&"you fool").unwrap(), "you fool");
        assert_eq!(
            *tree.floor(&"but i was going to tasche station").unwrap(),
            "back away...I will deal with this jedi slime myself"
        );
        assert_eq!(
            *tree.floor(&"you underestimate my power").unwrap(),
            "you fool"
        );
        assert_eq!(*tree.floor(&"your new empire").unwrap(), "your move");
        assert_eq!(*tree.ceil(&"hello there").unwrap(), "hello there");
        assert_eq!(
            *tree
                .ceil(&"these are not the droids you're looking for")
                .unwrap(),
            "you are a bold one"
        );
        assert_eq!(
            *tree.ceil(&"another death star").unwrap(),
            "back away...I will deal with this jedi slime myself"
        );
        assert_eq!(*tree.ceil(&"you fool").unwrap(), "you fool");
        assert_eq!(
            *tree.ceil(&"but i was going to tasche station").unwrap(),
            "general kenobi"
        );
        assert_eq!(
            *tree.ceil(&"you underestimate my power").unwrap(),
            "your move"
        );
        assert!(tree.ceil(&"your new empire").is_none());
    }

    #[test]
    fn test_iterator() {
        let tree = prequel_memes_tree();
        let mut iter = tree.iter();
        assert_eq!(
            iter.next().unwrap(),
            &"back away...I will deal with this jedi slime myself"
        );
        assert_eq!(iter.next().unwrap(), &"general kenobi");
        assert_eq!(iter.next().unwrap(), &"hello there");
        assert_eq!(iter.next().unwrap(), &"kill him");
        assert_eq!(iter.next().unwrap(), &"you are a bold one");
        assert_eq!(iter.next().unwrap(), &"you fool");
        assert_eq!(iter.next().unwrap(), &"your move");
        assert_eq!(iter.next(), None);
        assert_eq!(iter.next(), None);
    }
}
