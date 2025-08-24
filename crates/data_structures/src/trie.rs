use std::{
    hash::Hash,
    collections::HashMap
};

/// Trie树中的节点结构。
///
/// 每个节点包含：
/// - 一个哈希映射，用于存储子节点，键为Key类型，值为Node节点
/// - 一个可选的值，类型为Type，表示该节点是否存储了一个完整的键对应的值
#[derive(Debug, Default)]
struct Node<Key: Default, Type: Default> {
    children: HashMap<Key, Node<Key, Type>>,
    value: Option<Type>
}

/// 泛型Trie树（前缀树）数据结构。
///
/// Trie是一种树形数据结构，用于高效地存储和检索键值对集合，
/// 其中键通常是由字符或数字组成的序列。
///
/// 泛型参数：
/// - Key: 键的元素类型，必须实现Default、Hash和Eq trait
/// - Type: 值的类型，必须实现Default trait
#[derive(Debug, Default)]
pub struct Trie<Key: Default + Hash + Eq, Type: Default> {
    root: Node<Key, Type>
}

impl<Key, Type> Trie<Key, Type>
where
    Key: Default + Eq + Hash,
    Type: Default
{
    /// 创建一个新的空Trie树实例。
    ///
    /// 返回值：
    /// - 新创建的Trie实例
    pub fn new() -> Self {
        Self {
            root: Node::default(),
        }
    }

    /// 向Trie树中插入一个键值对。
    ///
    /// 参数：
    /// - key: 要插入的键，可以是任何能转换为Key类型迭代器的对象
    /// - value: 与键关联的值
    pub fn insert(&mut self, key: impl IntoIterator<Item = Key>, value: Type)
    where
        Key: Eq + Hash
    {
        // 从根节点开始遍历
        let mut node = &mut self.root;
        // 遍历键的每个元素，逐层向下创建或获取子节点
        for c in key {
            node = node.children.entry(c).or_default();
        }
        // 在最后一个节点上设置值
        node.value = Some(value);
    }

    /// 从Trie树中获取指定键对应的值。
    ///
    /// 参数：
    /// - key: 要查找的键，可以是任何能转换为Key类型迭代器的对象
    ///
    /// 返回值：
    /// - 如果找到键，则返回指向对应值的引用；否则返回None
    pub fn get(&self, key: impl IntoIterator<Item = Key>) -> Option<&Type>
    where
        Key: Eq + Hash
    {
        // 从根节点开始查找
        let mut node = &self.root;
        // 遍历键的每个元素，在Trie树中沿着路径向下查找
        for c in key {
            node = node.children.get(&c)?;
        }
        // 返回最终节点上的值引用（如果存在）
        node.value.as_ref()
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_insertion_and_retrieval_with_strings() {
        let mut trie = Trie::new();

        trie.insert("foo".chars(), 1);
        assert_eq!(trie.get("foo".chars()), Some(&1));
        trie.insert("foobar".chars(), 2);
        assert_eq!(trie.get("foobar".chars()), Some(&2));
        assert_eq!(trie.get("foo".chars()), Some(&1));
        trie.insert("bar".chars(), 3);
        assert_eq!(trie.get("bar".chars()), Some(&3));
        assert_eq!(trie.get("baz".chars()), None);
        assert_eq!(trie.get("foobarbaz".chars()), None);
    }

    #[test]
    fn test_insertion_and_retrieval_with_integers() {
        let mut trie = Trie::new();

        trie.insert(vec![1, 2, 3], 1);
        assert_eq!(trie.get(vec![1, 2, 3]), Some(&1));
        trie.insert(vec![1, 2, 3, 4, 5], 2);
        assert_eq!(trie.get(vec![1, 2, 3, 4, 5]), Some(&2));
        assert_eq!(trie.get(vec![1, 2, 3]), Some(&1));
        trie.insert(vec![10, 20, 30], 3);
        assert_eq!(trie.get(vec![10, 20, 30]), Some(&3));
        assert_eq!(trie.get(vec![4, 5, 6]), None);
        assert_eq!(trie.get(vec![1, 2, 3, 4, 5, 6]), None);
    }

    #[test]
    fn test_empty_trie() {
        let trie: Trie<char, i32> = Trie::new();

        assert_eq!(trie.get("foo".chars()), None);
        assert_eq!(trie.get("".chars()), None);
    }

    #[test]
    fn test_insert_empty_key() {
        let mut trie: Trie<char, i32> = Trie::new();

        trie.insert("".chars(), 42);
        assert_eq!(trie.get("".chars()), Some(&42));
        assert_eq!(trie.get("foo".chars()), None);
    }

    #[test]
    fn test_overlapping_keys() {
        let mut trie = Trie::new();

        trie.insert("car".chars(), 1);
        trie.insert("cart".chars(), 2);
        trie.insert("carter".chars(), 3);
        assert_eq!(trie.get("car".chars()), Some(&1));
        assert_eq!(trie.get("cart".chars()), Some(&2));
        assert_eq!(trie.get("carter".chars()), Some(&3));
        assert_eq!(trie.get("care".chars()), None);
    }

    #[test]
    fn test_partial_match() {
        let mut trie = Trie::new();

        trie.insert("apple".chars(), 10);
        assert_eq!(trie.get("app".chars()), None);
        assert_eq!(trie.get("appl".chars()), None);
        assert_eq!(trie.get("apple".chars()), Some(&10));
        assert_eq!(trie.get("applepie".chars()), None);
    }
}
