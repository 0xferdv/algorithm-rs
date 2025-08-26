use std::collections::hash_map::RandomState;
use std::fmt::{Debug, Formatter};
use std::hash::{BuildHasher, Hash};

/// 一种概率数据结构，能够高效地（使用常量空间）保存各种项目的近似计数
///
/// 假设想要计算来自传入（无界）数据流中的项目
/// 一种方法是维护一个频率哈希映射，计算元素哈希值
/// 这种方法效果非常好，但如果数据流中传入项目的多样性很大，会需要大量内存
///
/// CountMinSketch旨在解决这个问题，用近似计数换取精确计数，
/// 但从潜在的无界空间复杂度降低到常量复杂度
/// 请参见下面的实现了解更多详情
///
/// 以下是CountMinSketch允许的不同操作的定义：
///     * 增加项目的计数
///     * 检索项目的计数
pub trait CountMinSketch {
    type Item;

    /// 增加指定项目计数1
    /// 
    /// # 参数
    /// * `item` - 要增加计数的项目
    fn increment(&mut self, item: Self::Item);

    /// 增加指定项目指定数量的计数
    /// 
    /// # 参数
    /// * `item` - 要增加计数的项目
    /// * `count` - 要增加的计数量
    fn increment_by(&mut self, item: Self::Item, count: usize);

    /// 获取指定项目的近似计数
    /// 
    /// # 参数
    /// * `item` - 要查询计数的项目
    /// 
    /// # 返回值
    /// 返回该项目的近似计数，该值不会低于实际计数（不会低估）
    fn get_count(&self, item: Self::Item) -> usize;
}

/// CountMinSketch的通用实现
/// 持有一个DEPTH x WIDTH的计数矩阵
///
/// 实现背后的思想如下：
/// 从上面的问题陈述开始。有一个计数频率映射，想要减少其空间复杂度
/// 立即想到的方法是使用固定大小的向量，设这个大小为`WIDTH`
/// 将在向量中保存每个项目`item`的计数，索引为`i = hash(item) % WIDTH`，
/// 其中`hash`是一个哈希函数：`item -> usize`
/// 现在有了常量空间。
///
/// 但问题是可能会遇到很多冲突。
/// 举个极端例子，如果`WIDTH = 1`，所有项目都会有相同的计数，即所有项目计数的总和
/// 可以通过使用更大的`WIDTH`来减少冲突，但这不会比"大"频率映射更高效
/// 如何改进解决方案，但仍保持常量空间？
///
/// 想法是不仅仅使用一个向量，而是使用多个（`DEPTH`）向量，
/// 并为每个向量附加不同的`hash`函数
/// 这将导致以下数据结构：
///             <- WIDTH = 5 ->
///  D   hash1: [0, 0, 0, 0, 0]
///  E   hash2: [0, 0, 0, 0, 0]
///  P   hash3: [0, 0, 0, 0, 0]
///  T   hash4: [0, 0, 0, 0, 0]
///  H   hash5: [0, 0, 0, 0, 0]
///  =   hash6: [0, 0, 0, 0, 0]
///  7   hash7: [0, 0, 0, 0, 0]
/// 每个哈希函数必须为同一项目返回不同的值。
/// 假设哈希"TEST"：
///     hash1("TEST") = 42 => idx = 2
///     hash2("TEST") = 26 => idx = 1
///     hash3("TEST") = 10 => idx = 0
///     hash4("TEST") = 33 => idx = 3
///     hash5("TEST") = 54 => idx = 4
///     hash6("TEST") = 11 => idx = 1
///     hash7("TEST") = 50 => idx = 0
/// 这将使结构变为：
///             <- WIDTH = 5 ->
///  D   hash1: [0, 0, 1, 0, 0]
///  E   hash2: [0, 1, 0, 0, 0]
///  P   hash3: [1, 0, 0, 0, 0]
///  T   hash4: [0, 0, 0, 1, 0]
///  H   hash5: [0, 0, 0, 0, 1]
///  =   hash6: [0, 1, 0, 0, 0]
///  7   hash7: [1, 0, 0, 0, 0]
///
/// 现在假设哈希"OTHER"：
///     hash1("OTHER") = 23 => idx = 3
///     hash2("OTHER") = 11 => idx = 1
///     hash3("OTHER") = 52 => idx = 2
///     hash4("OTHER") = 25 => idx = 0
///     hash5("OTHER") = 31 => idx = 1
///     hash6("OTHER") = 24 => idx = 4
///     hash7("OTHER") = 30 => idx = 0
/// 使数据结构变为：
///             <- WIDTH = 5 ->
///  D   hash1: [0, 0, 1, 1, 0]
///  E   hash2: [0, 2, 0, 0, 0]
///  P   hash3: [1, 0, 1, 0, 0]
///  T   hash4: [1, 0, 0, 1, 0]
///  H   hash5: [0, 1, 0, 0, 1]
///  =   hash6: [0, 1, 0, 0, 1]
///  7   hash7: [2, 0, 0, 0, 0]
///
/// 实际上可以看到一些冲突（上面某些行中无效的`2`计数）。
/// 这意味着如果必须返回"TEST"的计数，会从每一行获取计数并返回最小值
///
/// 如果有大量条目和很多冲突，这可能会被高估。
/// 但一个有趣的性质是为"TEST"返回的计数不会被低估
pub struct HashCountMinSketch<Item: Hash, const WIDTH: usize, const DEPTH: usize> {
    phantom: std::marker::PhantomData<Item>, // 仅作为Item使用的标记
    counts: [[usize; WIDTH]; DEPTH],
    hashers: [RandomState; DEPTH],
}

impl<Item: Hash, const WIDTH: usize, const DEPTH: usize> Debug
    for HashCountMinSketch<Item, WIDTH, DEPTH>
{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Item").field("vecs", &self.counts).finish()
    }
}

impl<T: Hash, const WIDTH: usize, const DEPTH: usize> Default
    for HashCountMinSketch<T, WIDTH, DEPTH>
{
    fn default() -> Self {
        let hashers = std::array::from_fn(|_| RandomState::new());

        Self {
            phantom: std::marker::PhantomData,
            counts: [[0; WIDTH]; DEPTH],
            hashers,
        }
    }
}

impl<Item: Hash, const WIDTH: usize, const DEPTH: usize> CountMinSketch
    for HashCountMinSketch<Item, WIDTH, DEPTH>
{
    type Item = Item;

    /// 增加指定项目计数1
    /// 
    /// # 参数
    /// * `item` - 要增加计数的项目
    fn increment(&mut self, item: Self::Item) {
        self.increment_by(item, 1)
    }

    /// 增加指定项目指定数量的计数
    /// 
    /// # 参数
    /// * `item` - 要增加计数的项目
    /// * `count` - 要增加的计数量
    fn increment_by(&mut self, item: Self::Item, count: usize) {
        // 对每个哈希函数计算项目哈希值，并在对应位置增加计数
        for (row, r) in self.hashers.iter_mut().enumerate() {
            let mut h = r.build_hasher();
            item.hash(&mut h);
            let hashed = r.hash_one(&item);
            let col = (hashed % WIDTH as u64) as usize;
            self.counts[row][col] += count;
        }
    }

    /// 获取指定项目的近似计数
    /// 
    /// # 参数
    /// * `item` - 要查询计数的项目
    /// 
    /// # 返回值
    /// 返回该项目的近似计数，该值不会低于实际计数（不会低估）
    fn get_count(&self, item: Self::Item) -> usize {
        // 计算项目在每个哈希函数下的哈希值，获取对应位置的计数，并返回最小值
        self.hashers
            .iter()
            .enumerate()
            .map(|(row, r)| {
                let mut h = r.build_hasher();
                item.hash(&mut h);
                let hashed = r.hash_one(&item);
                let col = (hashed % WIDTH as u64) as usize;
                self.counts[row][col]
            })
            .min()
            .unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use quickcheck::{Arbitrary, Gen};
    use std::collections::HashSet;

    #[test]
    fn hash_functions_should_hash_differently() {
        let mut sketch: HashCountMinSketch<&str, 50, 50> = HashCountMinSketch::default();
        sketch.increment("something");
        let mut indices_of_ones: HashSet<usize> = HashSet::default();
        for counts in sketch.counts {
            let ones = counts
                .into_iter()
                .enumerate()
                .filter_map(|(idx, count)| (count == 1).then_some(idx))
                .collect::<Vec<_>>();
            assert_eq!(1, ones.len());
            indices_of_ones.insert(ones[0]);
        }
        assert!(indices_of_ones.len() > 1); 
    }

    #[test]
    fn inspect_counts() {
        let mut sketch: HashCountMinSketch<&str, 5, 7> = HashCountMinSketch::default();
        sketch.increment("test");
        for counts in sketch.counts {
            let zeroes = counts.iter().filter(|count| **count == 0).count();
            assert_eq!(4, zeroes);
            let ones = counts.iter().filter(|count| **count == 1).count();
            assert_eq!(1, ones);
        }
        sketch.increment("test");
        for counts in sketch.counts {
            let zeroes = counts.iter().filter(|count| **count == 0).count();
            assert_eq!(4, zeroes);
            let twos = counts.iter().filter(|count| **count == 2).count();
            assert_eq!(1, twos);
        }
        
        assert_eq!(2, sketch.get_count("test"));
    }

    #[derive(Debug, Clone, Eq, PartialEq, Hash)]
    struct TestItem {
        item: String,
        count: usize,
    }

    const MAX_STR_LEN: u8 = 30;
    const MAX_COUNT: usize = 20;

    impl Arbitrary for TestItem {
        fn arbitrary(g: &mut Gen) -> Self {
            let str_len = u8::arbitrary(g) % MAX_STR_LEN;
            let mut str = String::with_capacity(str_len as usize);
            for _ in 0..str_len {
                str.push(char::arbitrary(g));
            }
            let count = usize::arbitrary(g) % MAX_COUNT;
            TestItem { item: str, count }
        }
    }

    #[quickcheck_macros::quickcheck]
    fn must_not_understimate_count(test_items: Vec<TestItem>) {
        let test_items = test_items.into_iter().collect::<HashSet<_>>(); 
        let n = test_items.len();
        let mut sketch: HashCountMinSketch<String, 50, 10> = HashCountMinSketch::default();
        let mut exact_count = 0;
        for TestItem { item, count } in &test_items {
            sketch.increment_by(item.clone(), *count);
        }
        for TestItem { item, count } in test_items {
            let stored_count = sketch.get_count(item);
            assert!(stored_count >= count);
            if count == stored_count {
                exact_count += 1;
            }
        }
        if n > 20 {
            let exact_ratio = exact_count as f64 / n as f64;
            assert!(exact_ratio > 0.7); 
        }
    }
}
