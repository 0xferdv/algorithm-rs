/// Van Emde Boas Tree 实现
///
/// Van Emde Boas Tree 是一种支持快速插入、查找前驱和后继操作的数据结构，
/// 所有操作的时间复杂度均为 O(log log U)，其中 U 是 universe size。
pub struct VebTree {
    /// 树中最小元素，如果树为空则为 u32::MIN
    min: u32,
    /// 树中最大元素，如果树为空则为 u32::MAX
    max: u32,
    /// 当前树的大小（universe size），总是 2 的幂次
    size: u32,
    /// 子树的大小，等于 sqrt(size)
    child_size: u32,
    /// 子树集群，每个子树处理一个高阶部分
    cluster: Vec<VebTree>,
    /// 摘要树，用于快速定位非空的子树集群
    summary: Option<Box<VebTree>>,
}

impl VebTree {
    /// 创建一个新的 Van Emde Boas Tree
    ///
    /// # 参数
    /// * `size` - 树的 universe size
    ///
    /// # 返回值
    /// 返回一个新的 VebTree 实例
    pub fn new(size: u32) -> VebTree {
        let rounded_size = size.next_power_of_two();
        let child_size = (size as f64).sqrt().ceil() as u32;
        let mut cluster = Vec::new();
        if rounded_size > 2 {
            (0..rounded_size).for_each(|_| {
                cluster.push(VebTree::new(child_size));
            });
        }
        VebTree {
            min: u32::MIN,
            max: u32::MAX,
            size: rounded_size,
            child_size,
            cluster,
            summary: if rounded_size > 2 {
                None
            } else {
                Some(Box::new(VebTree::new(child_size)))
            },
        }
    }

    /// 计算给定值的高阶部分（cluster index）
    ///
    /// # 参数
    /// * `value` - 要计算高阶部分的值
    ///
    /// # 返回值
    /// 返回 value / child_size 的结果
    fn high(&self, value: u32) -> u32 {
        value / self.child_size
    }

    /// 计算给定值的低阶部分（offset within cluster）
    ///
    /// # 参数
    /// * `value` - 要计算低阶部分的值
    ///
    /// # 返回值
    /// 返回 value % child_size 的结果
    fn low(&self, value: u32) -> u32 {
        value % self.child_size
    }

    /// 根据 cluster index 和 offset 计算完整值
    ///
    /// # 参数
    /// * `cluster` - cluster index
    /// * `offest` - offset within cluster
    ///
    /// # 返回值
    /// 返回 cluster * child_size + offset 的结果
    fn index(&self, cluster: u32, offest: u32) -> u32 {
        cluster * self.child_size + offest
    }

    /// 获取树中的最小元素
    ///
    /// # 返回值
    /// 返回树中的最小元素
    #[allow(unused)]
    fn min(&self) -> u32 {
        self.min
    }

    /// 获取树中的最大元素
    ///
    /// # 返回值
    /// 返回树中的最大元素
    #[allow(unused)]
    fn max(&self) -> u32 {
        self.max
    }

    /// 创建一个迭代器用于遍历树中的所有元素
    ///
    /// # 返回值
    /// 返回一个新的 VebTreeIter 实例
    pub fn iter(&self) -> VebTreeIter {
        VebTreeIter::new(self)
    }

    /// 检查树是否为空
    ///
    /// # 返回值
    /// 如果树为空返回 true，否则返回 false
    pub fn empty(&self) -> bool {
        self.min > self.max
    }

    /// 在树中搜索指定值
    ///
    /// # 参数
    /// * `value` - 要搜索的值
    ///
    /// # 返回值
    /// 如果值存在于树中返回 true，否则返回 false
    pub fn search(&self, value: u32) -> bool {
        if self.empty() {
            return false;
        } else if value == self.min || value == self.max {
            return true
        } else if value < self.min || value > self.max {
            return false;
        }
        self.cluster[self.high(value) as usize].search(self.low(value))
    }

    /// 在空树中插入第一个元素
    ///
    /// # 参数
    /// * `value` - 要插入的值
    ///
    /// # 前置条件
    /// 树必须为空
    fn insert_empty(&mut self, value: u32) {
        assert!(self.empty(), "tree should be empty");
        self.min = value;
        self.max = value;
    }

    /// 向树中插入元素
    ///
    /// # 参数
    /// * `value` - 要插入的值
    ///
    /// # 前置条件
    /// value 必须小于树的大小
    pub fn insert(&mut self, mut value: u32) {
        assert!(value < self.size);
        if self.empty() {
            self.insert_empty(value);
            return
        }
        if value < self.min {
            (value, self.min) = (self.min, value);
        }
        if self.size > 2 {
            let high = self.high(value);
            let low = self.low(value);
            if self.cluster[high as usize].empty() {
                self.cluster[high as usize].insert_empty(low);
                if let Some(summary) = self.summary.as_mut() {
                    summary.insert(high);
                }
            } else {
                self.cluster[high as usize].insert(low);
            }
        }
        if value > self.max {
            self.max = value;
        }
    }

    /// 查找给定值的后继元素
    ///
    /// # 参数
    /// * `pred` - 要查找后继的值
    ///
    /// # 返回值
    /// 如果存在后继元素返回 Some(后继值)，否则返回 None
    pub fn succ(&self, pred: u32) -> Option<u32> {
        if self.empty() {
            return None
        }
        if self.size == 2 {
            return if pred == 0 && self.max == 1 {
                Some(1)
            } else {
                None
            };
        }
        if pred < self.min {
            return Some(self.min);
        }
        let low = self.low(pred);
        let high = self.high(pred);
        if !self.cluster[high as usize].empty() && low < self.cluster[high as usize].max {
            return Some(self.index(high, self.cluster[high as usize].succ(low).unwrap()))
        };
        let succ_cluster = self.summary.as_ref().unwrap().succ(high);
        succ_cluster
            .map(|succ_cluster| self.index(succ_cluster, self.cluster[succ_cluster as usize].min))
    }

    /// 查找给定值的前驱元素
    ///
    /// # 参数
    /// * `succ` - 要查找前驱的值
    ///
    /// # 返回值
    /// 如果存在前驱元素返回 Some(前驱值)，否则返回 None
    pub fn pred(&self, succ: u32) -> Option<u32> {
        if self.empty() {
            return None
        }
        if self.size == 2 {
            return if succ == 1 && self.min == 0 {
                Some(0)
            } else {
                None
            };
        }
        if succ > self.max {
            return Some(self.max);
        }
        let low = self.low(succ);
        let high = self.high(succ);
        if !self.cluster[high as usize].empty() && low > self.cluster[high as usize].min {
            return Some(self.index(high, self.cluster[high as usize].pred(low).unwrap()));
        };
        let succ_cluster = self.summary.as_ref().unwrap().pred(high);
        match succ_cluster {
            Some(succ_cluster) => {
                Some(self.index(succ_cluster, self.cluster[succ_cluster as usize].max))
            }
            None => {
                if succ > self.min {
                    Some(self.min)
                } else {
                    None
                }
            }
        }
    }
}

/// VebTree 的迭代器实现
pub struct VebTreeIter<'a> {
    /// 要迭代的 VebTree 引用
    tree: &'a VebTree,
    /// 当前迭代位置的元素
    curr: Option<u32>,
}

impl<'a> VebTreeIter<'a> {
    /// 创建一个新的 VebTree 迭代器
    ///
    /// # 参数
    /// * `tree` - 要迭代的 VebTree 引用
    ///
    /// # 返回值
    /// 返回一个新的 VebTreeIter 实例
    pub fn new(tree: &'a VebTree) -> VebTreeIter<'a> {
        let curr = if tree.empty() { None } else { Some(tree.min) };
        VebTreeIter { tree, curr }
    }
}

impl Iterator for VebTreeIter<'_> {
    type Item = u32;

    /// 获取下一个元素
    ///
    /// # 返回值
    /// 返回下一个元素，如果没有更多元素则返回 None
    fn next(&mut self) -> Option<u32> {
        let curr = self.curr;
        curr?;
        self.curr = self.tree.succ(curr.unwrap());
        curr
    }
}

#[cfg(test)]
mod test {
    use super::VebTree;
    use rand::{rngs::StdRng, Rng, SeedableRng};

    /// 测试 VebTree 的通用函数
    ///
    /// # 参数
    /// * `size` - 树的大小
    /// * `elements` - 要插入的元素列表
    /// * `exclude` - 应该不存在于树中的元素列表
    fn test_veb_tree(size: u32, mut elements: Vec<u32>, exclude: Vec<u32>) {
        // Insert elements
        let mut tree = VebTree::new(size);
        for element in elements.iter() {
            tree.insert(*element);
        }

        // Test search
        for element in elements.iter() {
            assert!(tree.search(*element));
        }
        for element in exclude {
            assert!(!tree.search(element));
        }

        // Test iterator and successor, and predecessor
        elements.sort();
        elements.dedup();
        for (i, element) in tree.iter().enumerate() {
            assert!(elements[i] == element);
        }
        for i in 1..elements.len() {
            assert!(tree.succ(elements[i - 1]) == Some(elements[i]));
            assert!(tree.pred(elements[i]) == Some(elements[i - 1]));
        }
    }

    #[test]
    fn test_empty() {
        test_veb_tree(16, Vec::new(), (0..16).collect());
    }

    #[test]
    fn test_single() {
        test_veb_tree(16, Vec::from([5]), (0..16).filter(|x| *x != 5).collect());
    }

    #[test]
    fn test_two() {
        test_veb_tree(
            16,
            Vec::from([4, 9]),
            (0..16).filter(|x| *x != 4 && *x != 9).collect(),
        );
    }

    #[test]
    fn test_repeat_insert() {
        let mut tree = VebTree::new(16);
        for _ in 0..5 {
            tree.insert(10);
        }
        assert!(tree.search(10));
        let elements: Vec<u32> = (0..16).filter(|x| *x != 10).collect();
        for element in elements {
            assert!(!tree.search(element));
        }
    }

    #[test]
    fn test_linear() {
        test_veb_tree(16, (0..10).collect(), (10..16).collect());
    }

    /// 测试完整的树
    ///
    /// # 参数
    /// * `size` - 树的大小
    fn test_full(size: u32) {
        test_veb_tree(size, (0..size).collect(), Vec::new());
    }

    #[test]
    fn test_full_small() {
        test_full(8);
        test_full(10);
        test_full(16);
        test_full(20);
        test_full(32);
    }

    #[test]
    fn test_full_256() {
        test_full(256);
    }

    #[test]
    fn test_10_256() {
        let mut rng = StdRng::seed_from_u64(0);
        let elements: Vec<u32> = (0..10).map(|_| rng.random_range(0..255)).collect();
        test_veb_tree(256, elements, Vec::new());
    }

    #[test]
    fn test_100_256() {
        let mut rng = StdRng::seed_from_u64(0);
        let elements: Vec<u32> = (0..100).map(|_| rng.random_range(0..255)).collect();
        test_veb_tree(256, elements, Vec::new());
    }

    #[test]
    fn test_100_300() {
        let mut rng = StdRng::seed_from_u64(0);
        let elements: Vec<u32> = (0..100).map(|_| rng.random_range(0..255)).collect();
        test_veb_tree(300, elements, Vec::new());
    }
}
