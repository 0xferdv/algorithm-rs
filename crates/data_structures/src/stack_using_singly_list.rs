/// 链表节点的类型别名，用于表示栈中的每个节点
/// 使用 Option<Box<Node<T>>> 来表示可能为空的节点指针
type Link<T> = Option<Box<Node<T>>>;

/// 链表节点结构体，包含元素值和指向下一个节点的指针
///
/// # 字段
/// * `elem` - 节点存储的元素值
/// * `next` - 指向下一个节点的可选指针
struct Node<T> {
    elem: T,
    next: Link<T>,
}


/// 栈数据结构，基于链表实现
///
/// # 字段
/// * `head` - 指向栈顶节点的可选指针
pub struct Stack<T> {
    head: Link<T>,
}


impl<T> Stack<T> {

    /// 创建一个新的空栈
    ///
    /// # 返回值
    /// 返回一个初始化为空的栈实例
    pub fn new() -> Self {
        Self { head: None}
    }

    /// 向栈顶压入一个新元素
    ///
    /// # 参数
    /// * `elem` - 要压入栈的元素值
    pub fn push(&mut self, elem: T) {
        let new_node = Box::new(
            Node {
                elem,
                next: self.head.take(),
            }
        );
        self.head = Some(new_node);
    }

    /// 从栈顶弹出一个元素
    ///
    /// # 返回值
    /// * `Ok(T)` - 成功弹出的元素值
    /// * `Err(&str)` - 如果栈为空，返回错误信息"Stack is empty"
    pub fn pop(&mut self) -> Result<T, &str> {
        match self.head.take() {
            None => Err("Stack is empty"),
            Some(node) => {
                self.head = node.next;
                Ok(node.elem)
            }
        }
    }

    /// 检查栈是否为空
    ///
    /// # 返回值
    /// * `true` - 栈为空
    /// * `false` - 栈不为空
    pub fn is_empty(&self) -> bool {
        self.head.is_none()
    }

    /// 查看栈顶元素但不移除它
    ///
    /// # 返回值
    /// * `Some(&T)` - 栈顶元素的引用
    /// * `None` - 如果栈为空，返回None
    pub fn peek(&self) -> Option<&T> {
        match self.head.as_ref() {
            None => None,
            Some(node) => Some(&node.elem),
        }
    }

    /// 获取栈顶元素的可变引用但不移除它
    ///
    /// # 返回值
    /// * `Some(&mut T)` - 栈顶元素的可变引用
    /// * `None` - 如果栈为空，返回None
    pub fn peek_mut(&mut self) -> Option<&mut T> {
        match self.head.as_mut() {
            None => None,
            Some(node) => Some(&mut node.elem),
        }
    }

    /// 创建一个消耗栈所有权的迭代器
    ///
    /// # 返回值
    /// 返回一个IntoIter迭代器实例，该迭代器会逐个弹出栈中元素
    pub fn into_iter_for_stack(self) -> IntoIter<T> {
        IntoIter(self)
    }

    /// 创建一个只读引用迭代器
    ///
    /// # 返回值
    /// 返回一个Iter迭代器实例，该迭代器按从栈顶到栈底的顺序提供元素引用
    pub fn iter(&self) -> Iter<'_, T> {
        Iter {
            next: self.head.as_deref(),
        }
    }

    /// 创建一个可变引用迭代器
    ///
    /// # 返回值
    /// 返回一个IterMut迭代器实例，该迭代器按从栈顶到栈底的顺序提供元素可变引用
    pub fn iter_mut(&mut self) -> IterMut<'_, T> {
        IterMut {
            next: self.head.as_deref_mut(),
        }
    }
}

impl<T> Default for Stack<T> {

    /// 为栈提供默认实现，创建一个空栈
    ///
    /// # 返回值
    /// 返回一个初始化为空的栈实例
    fn default() -> Self {
        Self::new()
    }

}

impl<T> Drop for Stack<T> {

    /// 自定义析构函数，手动清理栈中所有节点以避免栈溢出
    /// 通过迭代方式逐个释放节点内存，而不是递归方式
    fn drop(&mut self) {
        let mut cur_link = self.head.take();
        while let Some(mut boxed_node) = cur_link {
            cur_link = boxed_node.next.take();
        }
    }
}

/// 消耗所有权的迭代器结构体
/// 包装一个栈实例，用于逐个弹出元素
pub struct IntoIter<T>(Stack<T>);

impl<T> Iterator for IntoIter<T> {

    type Item = T;

    /// 获取下一个元素
    ///
    /// # 返回值
    /// * `Some(T)` - 下一个元素值
    /// * `None` - 如果没有更多元素
    fn next(&mut self) -> Option<Self::Item> {
        self.0.pop().ok()
    }
}


/// 只读引用迭代器结构体
///
/// # 字段
/// * `next` - 指向当前节点的引用
pub struct Iter<'a, T> {
    next: Option<&'a Node<T>>,
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;

    /// 获取下一个元素的引用
    ///
    /// # 返回值
    /// * `Some(&T)` - 下一个元素的引用
    /// * `None` - 如果没有更多元素
    fn next(&mut self) -> Option<Self::Item> {
        self.next.map(|node| {
            self.next = node.next.as_deref();
            &node.elem
        })
    }

}

/// 可变引用迭代器结构体
///
/// # 字段
/// * `next` - 指向当前节点的可变引用
pub struct IterMut<'a, T> {
    next: Option<&'a mut Node<T>>,
}

impl<'a, T> Iterator for IterMut<'a, T> {
    type Item = &'a mut T;

    /// 获取下一个元素的可变引用
    ///
    /// # 返回值
    /// * `Some(&mut T)` - 下一个元素的可变引用
    /// * `None` - 如果没有更多元素
    fn next(&mut self) -> Option<Self::Item> {
        self.next.take().map(|node| {
            self.next = node.next.as_deref_mut();
            &mut node.elem
        })
    }
}


#[cfg(test)]
mod test_stack {

    use super::*;

    #[test]
    fn basics() {
        let mut list = Stack::new();
        assert_eq!(list.pop(), Err("Stack is empty"));

        list.push(1);
        list.push(2);
        list.push(3);

        assert_eq!(list.pop(), Ok(3));
        assert_eq!(list.pop(), Ok(2));

        list.push(4);
        list.push(5);

        assert!(!list.is_empty());

        assert_eq!(list.pop(), Ok(5));
        assert_eq!(list.pop(), Ok(4));

        assert_eq!(list.pop(), Ok(1));
        assert_eq!(list.pop(), Err("Stack is empty"));

        assert!(list.is_empty());
    }

    #[test]
    fn peek() {
        let mut list = Stack::new();
        assert_eq!(list.peek(), None);
        list.push(1);
        list.push(2);
        list.push(3);

        assert_eq!(list.peek(), Some(&3));
        assert_eq!(list.peek_mut(), Some(&mut 3));

        match list.peek_mut() {
            None => (),
            Some(value) => *value = 42,
        };

        assert_eq!(list.peek(), Some(&42));
        assert_eq!(list.pop(), Ok(42));
    }

    #[test]
    fn into_iter() {
        let mut list = Stack::new();
        list.push(1);
        list.push(2);
        list.push(3);

        let mut iter = list.into_iter_for_stack();
        assert_eq!(iter.next(), Some(3));
        assert_eq!(iter.next(), Some(2));
        assert_eq!(iter.next(), Some(1));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn iter() {
        let mut list = Stack::new();
        list.push(1);
        list.push(2);
        list.push(3);

        let mut iter = list.iter();
        assert_eq!(iter.next(), Some(&3));
        assert_eq!(iter.next(), Some(&2));
        assert_eq!(iter.next(), Some(&1));
    }

    #[test]
    fn iter_mut() {
        let mut list = Stack::new();
        list.push(1);
        list.push(2);
        list.push(3);

        let mut iter = list.iter_mut();
        assert_eq!(iter.next(), Some(&mut 3));
        assert_eq!(iter.next(), Some(&mut 2));
        assert_eq!(iter.next(), Some(&mut 1));
    }
}
