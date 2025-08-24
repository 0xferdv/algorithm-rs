use std::cmp::PartialEq;
use std::collections::LinkedList;

/// 哈希表结构体，使用链表法解决哈希冲突
///
/// # 泛型参数
/// * `K` - 键的类型，必须实现 Hashable 和 PartialEq 特性
/// * `V` - 值的类型
pub struct HashTable<K, V> {
    elements: Vec<LinkedList<(K, V)>>,
    count: usize,
}

impl<K: Hashable + PartialEq, V> Default for HashTable<K, V> {
    /// 创建默认的哈希表实例
    fn default() -> Self {
        Self::new()
    }
}

/// 可哈希特性的定义，用于计算键的哈希值
pub trait Hashable {
    /// 计算当前对象的哈希值
    ///
    /// # 返回值
    /// 返回一个 usize 类型的哈希值
    fn hash(&self) -> usize;
}

impl<K: Hashable + PartialEq, V> HashTable<K, V> {

    /// 创建一个新的哈希表实例
    ///
    /// # 返回值
    /// 返回初始化后的 HashTable 实例，包含 3000 个桶
    pub fn new() -> HashTable<K, V> {
        let initial_capacity = 3000;
        let mut elements = Vec::with_capacity(initial_capacity);
        (0..initial_capacity).for_each(|_| elements.push(LinkedList::new()));
        HashTable { elements, count: 0 }
    }

    /// 向哈希表中插入键值对
    ///
    /// # 参数
    /// * `key` - 要插入的键
    /// * `value` - 要插入的值
    ///
    /// 当元素数量超过容量的 3/4 时会自动扩容
    pub fn insert(&mut self, key: K, value: V) {
        // 检查是否需要扩容
        if self.count >= self.elements.len() * 3 / 4 {
            self.resize();
        }
        let index = key.hash() % self.elements.len();
        self.elements[index].push_back((key, value));
        self.count += 1;
    }

    /// 在哈希表中搜索指定键对应的值
    ///
    /// # 参数
    /// * `key` - 要搜索的键
    ///
    /// # 返回值
    /// 如果找到对应的键，返回值的引用；否则返回 None
    pub fn search(&self, key: K) -> Option<&V> {
        let index = key.hash() % self.elements.len();
        self.elements[index]
            .iter()
            .find(|(k, _)| *k == key)
            .map(|(_, v)| v)
    }

    /// 扩容哈希表，将容量翻倍并重新分布所有元素
    ///
    /// 创建一个新的更大的存储空间，并将原有元素重新计算哈希后放入新位置
    fn resize(&mut self) {
        let new_size = self.elements.len() * 2;
        let mut new_elements = Vec::with_capacity(new_size);
        (0..new_size).for_each(|_| new_elements.push(LinkedList::new()));
        // 将旧元素移动到新存储空间中
        (self.elements.drain(..)).for_each(|old_list| {
            for (ket, value) in old_list {
                let new_index = ket.hash() % new_size;
                new_elements[new_index].push_back((ket, value));
            }
        });
        self.elements = new_elements;
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug, PartialEq, Eq)]
    struct TestKey(usize);

    impl Hashable for TestKey {
        fn hash(&self) -> usize {
            self.0
        }
    }

    #[test]
    fn test_insert_and_search() {
        let mut hash_table = HashTable::new();
        let key = TestKey(1);
        let value = TestKey(10);

        hash_table.insert(key, value);
        let result = hash_table.search(TestKey(1));

        assert_eq!(result, Some(&TestKey(10)));
    }

    #[test]
    fn test_resize() {
        let mut hash_table = HashTable::new();
        let initial_capacity = hash_table.elements.capacity();

        for i in 0..=initial_capacity * 3 / 4 {
            hash_table.insert(TestKey(i), TestKey(i + 10));
        }

        assert!(hash_table.elements.capacity() > initial_capacity);
    }

    #[test]
    fn test_search_nonexistent() {
        let mut hash_table = HashTable::new();
        let key = TestKey(1);
        let value = TestKey(10);

        hash_table.insert(key, value);
        let result = hash_table.search(TestKey(2));

        assert_eq!(result, None);
    }

    #[test]
    fn test_multiple_inserts_and_searches() {
        let mut hash_table = HashTable::new();
        for i in 0..10 {
            hash_table.insert(TestKey(i), TestKey(i + 100));
        }

        for i in 0..10 {
            let result = hash_table.search(TestKey(i));
            assert_eq!(result, Some(&TestKey(i + 100)));
        }
    }

    #[test]
    fn test_not_overwrite_existing_key() {
        let mut hash_table = HashTable::new();
        hash_table.insert(TestKey(1), TestKey(100));
        hash_table.insert(TestKey(1), TestKey(200));

        let result = hash_table.search(TestKey(1));
        assert_eq!(result, Some(&TestKey(100)));
    }

    #[test]
    fn test_empty_search() {
        let hash_table: HashTable<TestKey, TestKey> = HashTable::new();
        let result = hash_table.search(TestKey(1));

        assert_eq!(result, None);
    }
}
