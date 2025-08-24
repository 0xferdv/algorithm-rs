use std::collections::LinkedList;


/// 队列数据结构的实现
/// 使用LinkedList作为底层存储结构，提供FIFO（先进先出）的操作方式
#[derive(Debug)]
pub struct Queue<T> {
    elements: LinkedList<T>,
}


impl<T> Queue<T> {

    /// 创建一个新的空队列
    ///
    /// # 返回值
    /// 返回一个包含空LinkedList的Queue实例
    pub fn new() -> Queue<T> {
        Queue {
            elements: LinkedList::new(),
        }
    }

    /// 将元素添加到队列的尾部
    ///
    /// # 参数
    /// * `value` - 要添加到队列中的元素值
    pub fn enqueue(&mut self, value: T) {
        self.elements.push_back(value);
    }

    /// 从队列头部移除并返回元素
    ///
    /// # 返回值
    /// 如果队列不为空，返回Some(元素)；如果队列为空，返回None
    pub fn dequeue(&mut self) -> Option<T> {
        self.elements.pop_front()
    }

    /// 检查队列是否为空
    ///
    /// # 返回值
    /// 如果队列为空返回true，否则返回false
    pub fn is_empty(&self) -> bool {
        self.elements.is_empty()
    }

    /// 获取队列中元素的数量
    ///
    /// # 返回值
    /// 返回队列中当前元素的个数
    pub fn len(&self) -> usize {
        self.elements.len()
    }

    /// 查看队列头部的元素但不移除它
    ///
    /// # 返回值
    /// 如果队列不为空，返回Some(对头部元素的引用)；如果队列为空，返回None
    pub fn peek_front(&self) -> Option<&T> {
        self.elements.front()
    }

    /// 查看队列尾部的元素但不移除它
    ///
    /// # 返回值
    /// 如果队列不为空，返回Some(对尾部元素的引用)；如果队列为空，返回None
    pub fn peek_back(&self) -> Option<&T> {
        self.elements.back()
    }

    /// 清空队列中的所有元素
    pub fn drain(&mut self) {
        self.elements.clear()
    }
}

/// 为Queue实现Default trait，提供默认构造方式
impl<T> Default for Queue<T> {

    /// 创建一个默认的队列实例
    ///
    /// # 返回值
    /// 返回一个新的空队列实例
    fn default() -> Queue<T> {
        Queue::new()
    }
}

/// 队列功能的测试模块
#[cfg(test)]
mod tests {
    use super::Queue;

    #[test]
    fn test_queue_functionality() {
        let mut queue: Queue<usize> = Queue::default();

        assert!(queue.is_empty());
        queue.enqueue(8);
        queue.enqueue(16);
        assert!(!queue.is_empty());
        assert_eq!(queue.len(), 2);

        assert_eq!(queue.peek_front(), Some(&8));
        assert_eq!(queue.peek_back(), Some(&16));

        assert_eq!(queue.dequeue(), Some(8));
        assert_eq!(queue.len(), 1);
        assert_eq!(queue.peek_front(), Some(&16));
        assert_eq!(queue.peek_back(), Some(&16));

        queue.drain();
        assert!(queue.is_empty());
        assert_eq!(queue.len(), 0);
        assert_eq!(queue.dequeue(), None);
    }
}
