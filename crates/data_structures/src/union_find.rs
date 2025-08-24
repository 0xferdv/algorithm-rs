use std::{
    fmt::Debug,
    hash::Hash,
    cmp::Ordering,
    collections::HashMap,
};

/// 并查集（Union-Find）数据结构，用于处理不相交集合的合并与查询问题。
/// 支持动态插入元素、查找元素所属集合、合并两个集合以及判断两个元素是否属于同一集合。
///
/// 泛型参数 T 必须实现 Debug、Eq 和 Hash trait，以便支持调试输出、相等比较和哈希映射。
#[derive(Debug)]
pub struct UnionFind<T: Debug + Eq + Hash> {
    /// 当前并查集中独立集合的数量。
    count: usize,
    /// 每个节点对应的集合大小，用于优化合并操作（按秩合并）。
    sizes: Vec<usize>,
    /// 每个节点的父节点索引，用于路径压缩优化。
    parent_links: Vec<usize>,
    /// 将泛型值映射到内部索引的哈希表。
    payloads: HashMap<T, usize>,
}

impl<T: Debug + Eq + Hash> UnionFind<T> {

    /// 创建一个具有指定容量的并查集实例。
    ///
    /// # 参数
    /// * `capacity`: 预分配的容量大小，用于减少内存重新分配次数。
    ///
    /// # 返回值
    /// 返回一个新的并查集实例。
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            count: 0,
            sizes: Vec::with_capacity(capacity),
            parent_links: Vec::with_capacity(capacity),
            payloads: HashMap::with_capacity(capacity),
        }
    }

    /// 向并查集中插入一个新的元素。
    ///
    /// # 参数
    /// * `item`: 要插入的元素。
    pub fn insert(&mut self, item: T) {
        let key = self.payloads.len();
        self.parent_links.push(key);
        self.sizes.push(1);
        self.payloads.insert(item, key);
        self.count += 1;
    }

    /// 查找给定元素所属集合的根节点索引。
    ///
    /// # 参数
    /// * `value`: 要查找的元素引用。
    ///
    /// # 返回值
    /// 如果元素存在，则返回其根节点索引；否则返回 None。
    pub fn find(&mut self, value: &T) -> Option<usize> {
        self.payloads
            .get(value)
            .copied()
            .map(|key| self.find_by_key(key))
    }

    /// 合并两个元素所属的集合。
    ///
    /// # 参数
    /// * `first_item`: 第一个元素的引用。
    /// * `sec_item`: 第二个元素的引用。
    ///
    /// # 返回值
    /// 如果两个元素都存在且成功合并则返回 Some(true)；
    /// 如果两个元素已在同一集合中则返回 Some(false)；
    /// 如果任一元素不存在则返回 None。
    pub fn union(&mut self, first_item: &T, sec_item: &T) -> Option<bool> {
        let (first_root, sec_root) = (
            self.find(first_item), self.find(sec_item)
            );
        match (first_root, sec_root) {
            (Some(first_root), Some(sec_root)) => Some(self.union_by_key(first_root, sec_root)),
            _ => None,
        }
    }

    /// 根据内部索引查找对应元素的根节点索引，并执行路径压缩优化。
    ///
    /// # 参数
    /// * `key`: 元素在内部数组中的索引。
    ///
    /// # 返回值
    /// 返回该元素所属集合的根节点索引。
    fn find_by_key(&mut self, key: usize) -> usize {
        // 路径压缩：将查找路径上的所有节点直接连接到根节点
        if self.parent_links[key] != key {
            self.parent_links[key] = self.find_by_key(self.parent_links[key]);
        }
        self.parent_links[key]
    }

    /// 根据内部索引合并两个集合，并执行按秩合并优化。
    ///
    /// # 参数
    /// * `first_key`: 第一个集合根节点的索引。
    /// * `sec_key`: 第二个集合根节点的索引。
    ///
    /// # 返回值
    /// 如果两个集合不同且成功合并则返回 true；如果已在同一集合中则返回 false。
    fn union_by_key(&mut self, first_key: usize, sec_key: usize) -> bool {
        let (first_root, sec_root) = (self.find_by_key(first_key), self.find_by_key(sec_key));

        // 如果已在同一集合中，无需合并
        if first_root == sec_root {
            return false;
        }

        // 按秩合并：将较小的树合并到较大的树下以保持较低的高度
        match self.sizes[first_root].cmp(&self.sizes[sec_root]) {
            Ordering::Less => {
                self.parent_links[first_root] = sec_root;
                self.sizes[sec_root] += self.sizes[first_root];
            }
            _ => {
                self.parent_links[sec_root] = first_root;
                self.sizes[first_root] += self.sizes[sec_root];
            }
        }

        // 合并后独立集合数量减一
        self.count -= 1;
        true
    }

    /// 判断两个元素是否属于同一个集合。
    ///
    /// # 参数
    /// * `first_item`: 第一个元素的引用。
    /// * `sec_item`: 第二个元素的引用。
    ///
    /// # 返回值
    /// 如果两个元素都存在且属于同一集合则返回 true；否则返回 false。
    pub fn is_same_set(&mut self, first_item: &T, sec_item: &T) -> bool {
        matches!(
            (self.find(first_item), self.find(sec_item)),
            (Some(first_root), Some(sec_root))
            if first_root == sec_root
        )
    }

    /// 获取当前并查集中独立集合的数量。
    ///
    /// # 返回值
    /// 返回当前独立集合的数量。
    pub fn count(&self) -> usize {
        self.count
    }
}

impl<T: Debug + Eq + Hash> Default for UnionFind<T> {

    /// 创建一个默认的并查集实例。
    ///
    /// # 返回值
    /// 返回一个新的空并查集实例。
    fn default() -> Self {
        Self {
            count: 0,
            sizes: Vec::default(),
            parent_links: Vec::default(),
            payloads: HashMap::default(),
        }
    }
}

impl<T: Debug + Eq + Hash> FromIterator<T> for UnionFind<T> {

    /// 从迭代器创建并查集实例。
    ///
    /// # 参数
    /// * `iter`: 提供元素的迭代器。
    ///
    /// # 返回值
    /// 返回包含所有迭代器元素的并查集实例。
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        let mut uf = UnionFind::default();
        for item in iter {
            uf.insert(item);
        }
        uf
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_union_find() {
        let mut uf = (0..10).collect::<UnionFind<_>>();
        assert_eq!(uf.find(&0), Some(0));
        assert_eq!(uf.find(&1), Some(1));
        assert_eq!(uf.find(&2), Some(2));
        assert_eq!(uf.find(&3), Some(3));
        assert_eq!(uf.find(&4), Some(4));
        assert_eq!(uf.find(&5), Some(5));
        assert_eq!(uf.find(&6), Some(6));
        assert_eq!(uf.find(&7), Some(7));
        assert_eq!(uf.find(&8), Some(8));
        assert_eq!(uf.find(&9), Some(9));

        assert!(!uf.is_same_set(&0, &1));
        assert!(!uf.is_same_set(&2, &9));
        assert_eq!(uf.count(), 10);

        assert_eq!(uf.union(&0, &1), Some(true));
        assert_eq!(uf.union(&1, &2), Some(true));
        assert_eq!(uf.union(&2, &3), Some(true));
        assert_eq!(uf.union(&0, &2), Some(false));
        assert_eq!(uf.union(&4, &5), Some(true));
        assert_eq!(uf.union(&5, &6), Some(true));
        assert_eq!(uf.union(&6, &7), Some(true));
        assert_eq!(uf.union(&7, &8), Some(true));
        assert_eq!(uf.union(&8, &9), Some(true));
        assert_eq!(uf.union(&7, &9), Some(false));

        assert_ne!(uf.find(&0), uf.find(&9));
        assert_eq!(uf.find(&0), uf.find(&3));
        assert_eq!(uf.find(&4), uf.find(&9));
        assert!(uf.is_same_set(&0, &3));
        assert!(uf.is_same_set(&4, &9));
        assert!(!uf.is_same_set(&0, &9));
        assert_eq!(uf.count(), 2);

        assert_eq!(Some(true), uf.union(&3, &4));
        assert_eq!(uf.find(&0), uf.find(&9));
        assert_eq!(uf.count(), 1);
        assert!(uf.is_same_set(&0, &9));

        assert_eq!(None, uf.union(&0, &11));
    }

    #[test]
    fn test_spanning_tree() {
        let mut uf = UnionFind::from_iter(["A", "B", "C", "D", "E", "F", "G"]);
        uf.union(&"A", &"B");
        uf.union(&"B", &"C");
        uf.union(&"A", &"D");
        uf.union(&"F", &"G");

        assert_eq!(None, uf.union(&"A", &"W"));

        assert_eq!(uf.find(&"A"), uf.find(&"B"));
        assert_eq!(uf.find(&"A"), uf.find(&"C"));
        assert_eq!(uf.find(&"B"), uf.find(&"D"));
        assert_ne!(uf.find(&"A"), uf.find(&"E"));
        assert_ne!(uf.find(&"A"), uf.find(&"F"));
        assert_eq!(uf.find(&"G"), uf.find(&"F"));
        assert_ne!(uf.find(&"G"), uf.find(&"E"));

        assert!(uf.is_same_set(&"A", &"B"));
        assert!(uf.is_same_set(&"A", &"C"));
        assert!(uf.is_same_set(&"B", &"D"));
        assert!(!uf.is_same_set(&"B", &"F"));
        assert!(!uf.is_same_set(&"E", &"A"));
        assert!(!uf.is_same_set(&"E", &"G"));
        assert_eq!(uf.count(), 3);
    }

    #[test]
    fn test_with_capacity() {
        let mut uf: UnionFind<i32> = UnionFind::with_capacity(5);
        uf.insert(0);
        uf.insert(1);
        uf.insert(2);
        uf.insert(3);
        uf.insert(4);

        assert_eq!(uf.count(), 5);

        assert_eq!(uf.union(&0, &1), Some(true));
        assert!(uf.is_same_set(&0, &1));
        assert_eq!(uf.count(), 4);

        assert_eq!(uf.union(&2, &3), Some(true));
        assert!(uf.is_same_set(&2, &3));
        assert_eq!(uf.count(), 3);

        assert_eq!(uf.union(&0, &2), Some(true));
        assert!(uf.is_same_set(&0, &1));
        assert!(uf.is_same_set(&2, &3));
        assert!(uf.is_same_set(&0, &3));
        assert_eq!(uf.count(), 2);

        assert_eq!(None, uf.union(&0, &10));
    }
}
