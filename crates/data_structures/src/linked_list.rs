use std::ptr::NonNull;
use std::marker::PhantomData;
use std::fmt::{self, Display, Formatter};


/// 双向链表节点结构体
///
/// 用于存储链表中的单个元素，包含值和指向前一个及后一个节点的指针
pub struct Node<T> {
    /// 节点存储的值
    pub val: T,
    /// 指向下一个节点的指针
    pub next: Option<NonNull<Node<T>>>,
    /// 指向前一个节点的指针
    prev: Option<NonNull<Node<T>>>,
}

impl<T> Node<T> {

    /// 创建一个新的节点
    ///
    /// # 参数
    /// * `t`  - 要存储在节点中的值
    ///
    /// # 返回值
    /// 返回包含指定值的新节点
    fn new(t: T) -> Node<T> {
        Node {
            val: t,
            next: None,
            prev: None,
        }
    }
}

/// 双向链表结构体
///
/// 实现了一个双向链表，支持在头部、尾部和指定位置插入/删除元素
pub struct LinkedList<T> {
    /// 链表中元素的数量
    pub length: u32,
    /// 指向链表头部节点的指针
    pub head: Option<NonNull<Node<T>>>,
    /// 指向链表尾部节点的指针
    pub tail: Option<NonNull<Node<T>>>,
    /// 用于确保类型安全的标记
    marker: PhantomData<Box<Node<T>>>,
}

impl<T> Default for LinkedList<T> {
    /// 创建一个默认的链表实例
    ///
    /// # 返回值
    /// 返回一个空的链表
    fn default() -> Self {
        Self::new()
    }
}

impl<T> LinkedList<T> {

    /// 创建一个新的空链表
    ///
    /// # 返回值
    /// 返回一个初始化为空的链表
    pub fn new() -> Self {
        Self {
            length: 0,
            head: None,
            tail: None,
            marker: PhantomData,
        }
    }

    /// 在链表头部插入元素
    ///
    /// # 参数
    /// * `obj` - 要插入的元素值
    pub fn insert_at_head(&mut self, obj: T) {
        let mut node = Box::new(Node::new(obj));
        node.next = self.head;
        node.prev = None;
        let node_ptr = NonNull::new(Box::into_raw(node));
        match self.head {
            None => self.tail = node_ptr,
            Some(head_ptr) => unsafe {
                (*head_ptr.as_ptr()).prev = node_ptr
            }
        }
        self.head = node_ptr;
        self.length += 1;
    }

    /// 在链表尾部插入元素
    ///
    /// # 参数
    /// * `obj` - 要插入的元素值
    pub fn insert_at_tail(&mut self, obj: T) {
        let mut node = Box::new(Node::new(obj));
        node.next = None;
        node.prev = self.tail;
        let node_ptr = NonNull::new(Box::into_raw(node));
        match self.tail {
            None => self.head = node_ptr,
            Some(tail_ptr) => unsafe {
                (*tail_ptr.as_ptr()).next = node_ptr
            }
        }
        self.tail = node_ptr;
        self.length += 1;
    }

    /// 在链表指定位置插入元素
    ///
    /// # 参数
    /// * `index` - 要插入的位置索引
    /// * `obj` - 要插入的元素值
    ///
    /// # Panics
    /// 当索引超出链表范围时会触发panic
    pub fn insert_at_ith(&mut self, index: u32, obj: T) {
        // 检查索引是否超出范围
        if self.length < index {
            panic!("Index out of bounds");
        }
        // 如果索引为0或链表为空，则在头部插入
        if index == 0 || self.head.is_none() {
            self.insert_at_head(obj);
            return;
        }
        // 如果索引等于链表长度，则在尾部插入
        if self.length == index {
            self.insert_at_tail(obj);
            return;
        }

        // 在中间位置插入元素
        if let Some(mut ith_node) = self.head {
            // 找到指定位置的节点
            for _ in 0..index {
                unsafe {
                    match (*ith_node.as_ptr()).next {
                        None => panic!("Index out of bounds"),
                        Some(node_ptr) => ith_node = node_ptr,
                    }
                }
            }
            // 创建新节点并调整指针
            let mut node = Box::new(Node::new(obj));
            unsafe {
                node.prev = (*ith_node.as_ptr()).prev;
                node.next = Some(ith_node);
                if let Some(p) = (*ith_node.as_ptr()).prev {
                    let node_ptr = NonNull::new(Box::into_raw(node));
                    println!("{:?}", (*p.as_ptr()).next);
                    (*p.as_ptr()).next = node_ptr;
                    (*ith_node.as_ptr()).prev = node_ptr;
                    self.length += 1;
                }
            }
        }
    }

    /// 删除链表头部元素
    ///
    /// # 返回值
    /// 返回被删除元素的值，如果链表为空则返回None
    pub fn delete_head(&mut self) -> Option<T> {
        // 如果链表为空，直接返回None
        if self.length == 0 {
            return None;
        }
        self.head.map(|head_ptr| unsafe {
            let old_head = Box::from_raw(head_ptr.as_ptr());
            match old_head.next {
                None => self.tail = None,
                Some(mut next_ptr) => next_ptr.as_mut().prev = None,
            }
            self.head = old_head.next;
            self.length = self.length.checked_add_signed(-1).unwrap();
            old_head.val
        })
    }

    /// 删除链表尾部元素
    ///
    /// # 返回值
    /// 返回被删除元素的值，如果链表为空则返回None
    pub fn delete_tail(&mut self) -> Option<T> {
        self.tail.map(|tail_ptr| unsafe {
            let old_tail = Box::from_raw(tail_ptr.as_ptr());
            match old_tail.prev {
                None => self.head = None,
                Some(mut prev) => prev.as_mut().next = None,
            }
            self.tail = old_tail.prev;
            self.length -= 1;
            old_tail.val
        })
    }

    /// 删除链表指定位置的元素
    ///
    /// # 参数
    /// * `index` - 要删除元素的位置索引
    ///
    /// # 返回值
    /// 返回被删除元素的值，如果索引无效则返回None
    ///
    /// # Panics
    /// 当索引超出链表范围时会触发panic
    pub fn delete_ith(&mut self, index: u32) -> Option<T> {
        // 检查索引是否超出范围
        if self.length <= index {
            panic!("Index out of bounds");
        }
        // 如果索引为0或链表为空，则删除头部元素
        if index == 0 || self.head.is_none() {
            return self.delete_head();
        }
        // 如果索引指向尾部元素，则删除尾部元素
        if self.length - 1 == index {
            return self.delete_tail();
        }

        // 删除中间位置的元素
        if let Some(mut ith_node) = self.head {
            // 找到指定位置的节点
            for _ in 0..index {
                unsafe {
                    match (*ith_node.as_ptr()).next {
                        None => panic!("Index out of bounds"),
                        Some(node_ptr) => ith_node = node_ptr,
                    }
                }
            }
            unsafe {
                let old_ith = Box::from_raw(ith_node.as_ptr());
                // 调整前后节点的指针
                if let Some(mut prev) = old_ith.prev {
                    prev.as_mut().next = old_ith.next;
                }
                if let Some(mut next) = old_ith.next {
                    next.as_mut().prev = old_ith.prev;
                }
                self.length -= 1;
                Some(old_ith.val)
            }
        } else {
            None
        }
    }

    /// 获取指定索引位置元素的引用
    ///
    /// # 参数
    /// * `index` - 要获取元素的索引位置
    ///
    /// # 返回值
    /// 返回指定位置元素的引用，如果索引无效则返回None
    pub fn get(&self, index: i32) -> Option<&T> {
        Self::get_ith_node(self.head, index).map(|ptr| unsafe {
            &(*ptr.as_ptr()).val
        })
    }

    /// 递归获取第i个节点
    ///
    /// # 参数
    /// * `node` - 起始节点
    /// * `index` - 目标节点的索引
    ///
    /// # 返回值
    /// 返回第i个节点的指针，如果不存在则返回None
    fn get_ith_node(node: Option<NonNull<Node<T>>>, index: i32) -> Option<NonNull<Node<T>>> {
        match node {
            None => None,
            Some(next_ptr) => match index {
                0 => Some(next_ptr),
                _ => Self::get_ith_node(unsafe { (*next_ptr.as_ptr()).next }, index - 1),
            },
        }
    }
}

impl<T> Drop for LinkedList<T> {

    /// 释放链表占用的内存
    ///
    /// 通过不断删除头部元素来清理整个链表
    fn drop(&mut self) {
        while self.delete_head().is_some() {}
    }
}

impl<T> Display for LinkedList<T>
where
    T: Display
{
    /// 格式化链表为字符串
    ///
    /// # 参数
    /// * `f` - 格式化器
    ///
    /// # 返回值
    /// 格式化结果
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self.head {
            Some(node) => write!(f, "{}", unsafe { node.as_ref() }),
            None => Ok(()),
        }
    }
}

impl<T> Display for Node<T>
where
    T: Display
{
    /// 格式化节点为字符串
    ///
    /// # 参数
    /// * `f` - 格式化器
    ///
    /// # 返回值
    /// 格式化结果
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self.next {
            Some(node) => write!(f, "{}, {}", self.val, unsafe { node.as_ref() }),
            None => write!(f, "{}", self.val),
        }
    }
}


#[cfg(test)]
mod tests {
    use std::convert::TryInto;

    use super::LinkedList;

    #[test]
    fn insert_at_tail_works() {
        let mut list = LinkedList::<i32>::new();
        let second_value = 2;
        list.insert_at_tail(1);
        list.insert_at_tail(second_value);
        println!("Linked List is {list}");
        match list.get(1) {
            Some(val) => assert_eq!(*val, second_value),
            None => panic!("Expected to find {second_value} at index 1"),
        }
    }
    #[test]
    fn insert_at_head_works() {
        let mut list = LinkedList::<i32>::new();
        let second_value = 2;
        list.insert_at_head(1);
        list.insert_at_head(second_value);
        println!("Linked List is {list}");
        match list.get(0) {
            Some(val) => assert_eq!(*val, second_value),
            None => panic!("Expected to find {second_value} at index 0"),
        }
    }

    #[test]
    fn insert_at_ith_can_add_to_tail() {
        let mut list = LinkedList::<i32>::new();
        let second_value = 2;
        list.insert_at_ith(0, 0);
        list.insert_at_ith(1, second_value);
        println!("Linked List is {list}");
        match list.get(1) {
            Some(val) => assert_eq!(*val, second_value),
            None => panic!("Expected to find {second_value} at index 1"),
        }
    }

    #[test]
    fn insert_at_ith_can_add_to_head() {
        let mut list = LinkedList::<i32>::new();
        let second_value = 2;
        list.insert_at_ith(0, 1);
        list.insert_at_ith(0, second_value);
        println!("Linked List is {list}");
        match list.get(0) {
            Some(val) => assert_eq!(*val, second_value),
            None => panic!("Expected to find {second_value} at index 0"),
        }
    }

    #[test]
    fn insert_at_ith_can_add_to_middle() {
        let mut list = LinkedList::<i32>::new();
        let second_value = 2;
        let third_value = 3;
        list.insert_at_ith(0, 1);
        list.insert_at_ith(1, second_value);
        list.insert_at_ith(1, third_value);
        println!("Linked List is {list}");
        match list.get(1) {
            Some(val) => assert_eq!(*val, third_value),
            None => panic!("Expected to find {third_value} at index 1"),
        }

        match list.get(2) {
            Some(val) => assert_eq!(*val, second_value),
            None => panic!("Expected to find {second_value} at index 1"),
        }
    }

    #[test]
    fn insert_at_ith_and_delete_at_ith_in_the_middle() {
        let mut list = LinkedList::<i32>::new();
        let first_value = 0;
        let second_value = 1;
        let third_value = 2;
        let fourth_value = 3;

        list.insert_at_ith(0, first_value);
        list.insert_at_ith(1, fourth_value);
        list.insert_at_ith(1, third_value);
        list.insert_at_ith(1, second_value);

        list.delete_ith(2);
        list.insert_at_ith(2, third_value);

        for (i, expected) in [
            (0, first_value),
            (1, second_value),
            (2, third_value),
            (3, fourth_value),
        ] {
            match list.get(i) {
                Some(val) => assert_eq!(*val, expected),
                None => panic!("Expected to find {expected} at index {i}"),
            }
        }
    }

    #[test]
    fn insert_at_ith_and_delete_ith_work_over_many_iterations() {
        let mut list = LinkedList::<i32>::new();
        for i in 0..100 {
            list.insert_at_ith(i, i.try_into().unwrap());
        }

        for i in 0..50 {
            println!("list.length {}", list.length);
            if i % 2 == 0 {
                list.delete_ith(i);
            }
        }

        assert_eq!(list.length, 75);

        for i in 0..50 {
            if i % 2 == 0 {
                list.insert_at_ith(i, i.try_into().unwrap());
            }
        }

        assert_eq!(list.length, 100);

        if let Some(val) = list.get(78) {
            assert_eq!(*val, 78);
        } else {
            panic!("Expected to find 78 at index 78");
        }
    }

    #[test]
    fn delete_tail_works() {
        let mut list = LinkedList::<i32>::new();
        let first_value = 1;
        let second_value = 2;
        list.insert_at_tail(first_value);
        list.insert_at_tail(second_value);
        match list.delete_tail() {
            Some(val) => assert_eq!(val, 2),
            None => panic!("Expected to remove {second_value} at tail"),
        }

        println!("Linked List is {list}");
        match list.get(0) {
            Some(val) => assert_eq!(*val, first_value),
            None => panic!("Expected to find {first_value} at index 0"),
        }
    }

    #[test]
    fn delete_head_works() {
        let mut list = LinkedList::<i32>::new();
        let first_value = 1;
        let second_value = 2;
        list.insert_at_tail(first_value);
        list.insert_at_tail(second_value);
        match list.delete_head() {
            Some(val) => assert_eq!(val, 1),
            None => panic!("Expected to remove {first_value} at head"),
        }

        println!("Linked List is {list}");
        match list.get(0) {
            Some(val) => assert_eq!(*val, second_value),
            None => panic!("Expected to find {second_value} at index 0"),
        }
    }

    #[test]
    fn delete_ith_can_delete_at_tail() {
        let mut list = LinkedList::<i32>::new();
        let first_value = 1;
        let second_value = 2;
        list.insert_at_tail(first_value);
        list.insert_at_tail(second_value);
        match list.delete_ith(1) {
            Some(val) => assert_eq!(val, 2),
            None => panic!("Expected to remove {second_value} at tail"),
        }

        assert_eq!(list.length, 1);
    }

    #[test]
    fn delete_ith_can_delete_at_head() {
        let mut list = LinkedList::<i32>::new();
        let first_value = 1;
        let second_value = 2;
        list.insert_at_tail(first_value);
        list.insert_at_tail(second_value);
        match list.delete_ith(0) {
            Some(val) => assert_eq!(val, 1),
            None => panic!("Expected to remove {first_value} at tail"),
        }

        assert_eq!(list.length, 1);
    }

    #[test]
    fn delete_ith_can_delete_in_middle() {
        let mut list = LinkedList::<i32>::new();
        let first_value = 1;
        let second_value = 2;
        let third_value = 3;
        list.insert_at_tail(first_value);
        list.insert_at_tail(second_value);
        list.insert_at_tail(third_value);
        match list.delete_ith(1) {
            Some(val) => assert_eq!(val, 2),
            None => panic!("Expected to remove {second_value} at tail"),
        }

        match list.get(1) {
            Some(val) => assert_eq!(*val, third_value),
            None => panic!("Expected to find {third_value} at index 1"),
        }
    }

    #[test]
    fn create_numeric_list() {
        let mut list = LinkedList::<i32>::new();
        list.insert_at_tail(1);
        list.insert_at_tail(2);
        list.insert_at_tail(3);
        println!("Linked List is {list}");
        assert_eq!(3, list.length);
    }

    #[test]
    fn create_string_list() {
        let mut list_str = LinkedList::<String>::new();
        list_str.insert_at_tail("A".to_string());
        list_str.insert_at_tail("B".to_string());
        list_str.insert_at_tail("C".to_string());
        println!("Linked List is {list_str}");
        assert_eq!(3, list_str.length);
    }

    #[test]
    fn get_by_index_in_numeric_list() {
        let mut list = LinkedList::<i32>::new();
        list.insert_at_tail(1);
        list.insert_at_tail(2);
        println!("Linked List is {list}");
        let retrived_item = list.get(1);
        assert!(retrived_item.is_some());
        assert_eq!(2, *retrived_item.unwrap());
    }

    #[test]
    fn get_by_index_in_string_list() {
        let mut list_str = LinkedList::<String>::new();
        list_str.insert_at_tail("A".to_string());
        list_str.insert_at_tail("B".to_string());
        println!("Linked List is {list_str}");
        let retrived_item = list_str.get(1);
        assert!(retrived_item.is_some());
        assert_eq!("B", *retrived_item.unwrap());
    }

    #[test]
    #[should_panic(expected = "Index out of bounds")]
    fn delete_ith_panics_if_index_equals_length() {
        let mut list = LinkedList::<i32>::new();
        list.insert_at_tail(1);
        list.insert_at_tail(2);
        list.delete_ith(2);
    }
}
