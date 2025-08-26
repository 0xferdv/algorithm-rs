use std::hash::{BuildHasher, Hash, Hasher};
use std::collections::hash_map::{DefaultHasher, RandomState};

/// 布隆过滤器 trait，定义了插入和查询操作
/// 
/// # 泛型参数
/// * `Item` - 要存储在布隆过滤器中的元素类型，必须实现 Hash trait
pub trait BloomFilter<Item: Hash> {
    /// 将元素插入到布隆过滤器中
    /// 
    /// # 参数
    /// * `item` - 要插入的元素
    fn insert(&mut self, item: Item);

    /// 检查元素是否可能存在于布隆过滤器中
    /// 
    /// # 参数
    /// * `item` - 要查询的元素引用
    /// 
    /// # 返回值
    /// 如果元素可能存在返回 true，如果确定不存在返回 false
    fn contains(&self, item: &Item) -> bool;
}


/// 基础布隆过滤器实现，使用固定大小的布尔数组
/// 
/// # 泛型参数
/// * `CAPACITY` - 布隆过滤器的容量大小
#[derive(Debug)]
struct BasicBloomFilter<const CAPACITY: usize> {
    /// 存储位信息的布尔数组
    vec: [bool; CAPACITY],
}

impl<const CAPACITY: usize> Default for BasicBloomFilter<CAPACITY> {
    /// 创建一个新的默认布隆过滤器，所有位都初始化为 false
    fn default() -> Self {
        Self {
            vec: [false; CAPACITY],
        }
    }
}


impl<Item: Hash, const CAPACITY: usize> BloomFilter<Item> for BasicBloomFilter<CAPACITY> {
    /// 插入元素到布隆过滤器中
    /// 使用单个哈希函数计算元素的索引位置并设置为 true
    /// 
    /// # 参数
    /// * `item` - 要插入的元素
    fn insert(&mut self, item: Item) {
        let mut hasher = DefaultHasher::new();
        item.hash(&mut hasher);
        let idx = (hasher.finish() % CAPACITY as u64) as usize;
        self.vec[idx] = true;
    }

    /// 检查元素是否可能存在于布隆过滤器中
    /// 使用单个哈希函数计算索引位置并检查对应位是否为 true
    /// 
    /// # 参数
    /// * `item` - 要查询的元素引用
    /// 
    /// # 返回值
    /// 如果对应位为 true 返回 true，否则返回 false
    fn contains(&self, item: &Item) -> bool {
        let mut hasher = DefaultHasher::new();
        item.hash(&mut hasher);
        let idx = (hasher.finish() % CAPACITY as u64) as usize;
        self.vec[idx]
    }
}

#[allow(dead_code)]
#[derive(Debug, Default)]
/// 单二进制位布隆过滤器，使用一个 u128 整数作为位图
struct SingleBinaryBloomFilter {
    /// 存储位信息的指纹，使用 u128 的每一位表示一个位
    fingerprint: u128,
}

/// 计算元素在 u128 位图中的掩码
/// 
/// # 参数
/// * `hasher` - 哈希器引用
/// * `item` - 要计算掩码的元素
/// 
/// # 返回值
/// 对应位设置为 1 的 u128 掩码值
fn mask_128<T: Hash>(hasher: &mut DefaultHasher, item: T) -> u128 {
    item.hash(hasher);
    let idx = (hasher.finish() % 128) as u32;
    2u128.pow(idx)
}


impl<T: Hash> BloomFilter<T> for SingleBinaryBloomFilter {
    /// 插入元素到二进制布隆过滤器中
    /// 通过或运算将对应位设置为 1
    /// 
    /// # 参数
    /// * `item` - 要插入的元素
    fn insert(&mut self, item: T) {
        self.fingerprint |= mask_128(&mut DefaultHasher::new(), &item);
    }

    /// 检查元素是否可能存在于二进制布隆过滤器中
    /// 通过取模运算检查对应位是否为 1
    /// 
    /// # 参数
    /// * `item` - 要查询的元素引用
    /// 
    /// # 返回值
    /// 如果对应位为 1 返回 true，否则返回 false
    fn contains(&self, item: &T) -> bool {
        (self.fingerprint % mask_128(&mut DefaultHasher::new(), item)) > 0
    }
}


/// 多二进制位布隆过滤器，使用多个哈希函数和字节数组实现
pub struct MultiBinaryBloomFilter {
    /// 存储位信息的字节数组
    bytes: Vec<u8>,
    /// 布隆过滤器的总位数
    filter_size: usize,
    /// 多个哈希构建器，用于生成多个哈希值
    hash_builders: Vec<RandomState>,
}


impl MultiBinaryBloomFilter {
    /// 创建指定尺寸和哈希函数数量的布隆过滤器
    /// 
    /// # 参数
    /// * `filter_size` - 过滤器的总位数
    /// * `hash_count` - 使用的哈希函数数量
    /// 
    /// # 返回值
    /// 新创建的 MultiBinaryBloomFilter 实例
    pub fn with_dimensions(filter_size: usize, hash_count: usize) -> Self {
        let bytes_count = filter_size / 8 + usize::from(filter_size % 8 > 0);
        Self {
            bytes: vec![0; bytes_count],
            filter_size,
            hash_builders: vec![RandomState::new(); hash_count],
        }
    }

    /// 根据预期元素数量和最大误报率计算最优参数并创建布隆过滤器
    /// 
    /// # 参数
    /// * `estimated_count_of_items` - 预期插入的元素数量
    /// * `max_false_positive_probability` - 最大允许的误报率
    /// 
    /// # 返回值
    /// 根据最优参数创建的 MultiBinaryBloomFilter 实例
    pub fn from_estimate(
        estimated_count_of_items: usize,
        max_false_positive_probability: f64,
    ) -> Self {
        let optimal_filter_size = (-(estimated_count_of_items as f64)
            * max_false_positive_probability.ln()
            / (2.0f64.ln().powi(2))).ceil() as usize;
        let optimal_hash_count = ((optimal_filter_size as f64 / estimated_count_of_items as f64)
            * 2.0_f64.ln()
        ).ceil() as usize;
        Self::with_dimensions(optimal_filter_size, optimal_hash_count)
    }
}

impl<Item: Hash> BloomFilter<Item> for MultiBinaryBloomFilter {
    /// 插入元素到多哈希布隆过滤器中
    /// 使用多个哈希函数计算多个索引位置并设置对应位为 1
    /// 
    /// # 参数
    /// * `item` - 要插入的元素
    fn insert(&mut self, item: Item) {
        for builder in &self.hash_builders {
            let mut hasher = builder.build_hasher();
            item.hash(&mut hasher);
            let hash = builder.hash_one(&item);
            let index = hash % self.filter_size as u64;
            let byte_index = index as usize / 8;
            let bit_index = (index % 8) as u8;
            self.bytes[byte_index] |= 1 << bit_index;
        }
    }

    /// 检查元素是否可能存在于多哈希布隆过滤器中
    /// 使用多个哈希函数计算索引位置并检查所有对应位是否都为 1
    /// 
    /// # 参数
    /// * `item` - 要查询的元素引用
    /// 
    /// # 返回值
    /// 如果所有对应位都为 1 返回 true，否则返回 false
    fn contains(&self, item: &Item) -> bool {
        for builder in &self.hash_builders {
            let mut hasher = builder.build_hasher();
            item.hash(&mut hasher);
            let hash = builder.hash_one(item);
            let index = hash % self.filter_size as u64;
            let byte_index = index as usize / 8;
            let bit_index = (index % 8) as u8;
            if self.bytes[byte_index] & (1 << bit_index) == 0 {
                return false;
            }
        }
        true
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use quickcheck::{Arbitrary, Gen};
    use quickcheck_macros::quickcheck;
    use std::collections::HashSet;


    #[derive(Debug, Clone)]
    struct TestSet {
        to_insert: HashSet<i32>,
        to_test: Vec<i32>,
    }

    impl Arbitrary for TestSet {

        fn arbitrary(g: &mut Gen) -> Self {
            let mut qty = usize::arbitrary(g) % 5_000;
            if qty < 50 {
                qty += 50; 
            }
            let mut to_insert = HashSet::with_capacity(qty);
            let mut to_test = Vec::with_capacity(qty);
            for _ in 0..(qty) {
                to_insert.insert(i32::arbitrary(g));
                to_test.push(i32::arbitrary(g));
            }
            TestSet { to_insert, to_test }
        }
    }
    
    #[quickcheck]
    fn basic_filter_must_not_return_false_negative(TestSet { to_insert, to_test }: TestSet) {
        let mut basic_filter = BasicBloomFilter::<10_000>::default();
        for item in &to_insert {
            basic_filter.insert(*item);
        }
        for other in to_test {
            if !basic_filter.contains(&other) {
                assert!(!to_insert.contains(&other))
            }
        }
    }
    
    #[quickcheck]
    fn binary_filter_must_not_return_false_negative(TestSet { to_insert, to_test }: TestSet) {
        let mut binary_filter = SingleBinaryBloomFilter::default();
        for item in &to_insert {
            binary_filter.insert(*item);
        }
        for other in to_test {
            if !binary_filter.contains(&other) {
                assert!(!to_insert.contains(&other))
            }
        }
    }
    
    #[quickcheck]
    fn a_basic_filter_of_capacity_128_is_the_same_as_a_binary_filter(
        TestSet { to_insert, to_test }: TestSet,
    ) {
        let mut basic_filter = BasicBloomFilter::<128>::default(); 
        let mut binary_filter = SingleBinaryBloomFilter::default();
        for item in &to_insert {
            basic_filter.insert(*item);
            binary_filter.insert(*item);
        }
        for other in to_test {
            assert_eq!(
                basic_filter.contains(&other),
                binary_filter.contains(&other)
            );
        }
    }

    /// 最大允许的误报率常量
    const FALSE_POSITIVE_MAX: f64 = 0.05;

    /// 测试多二进制布隆过滤器不会返回假阴性结果
    #[quickcheck]
    fn a_multi_binary_bloom_filter_must_not_return_false_negatives(
        TestSet { to_insert, to_test }: TestSet,
    ) {
        let n = to_insert.len();
        if n == 0 {
            // avoid dividing by 0 when adjusting the size
            return;
        }
        // See Wikipedia for those formula
        let mut binary_filter = MultiBinaryBloomFilter::from_estimate(n, FALSE_POSITIVE_MAX);
        for item in &to_insert {
            binary_filter.insert(*item);
        }
        let tests = to_test.len();
        let mut false_positives = 0;
        for other in to_test {
            if !binary_filter.contains(&other) {
                assert!(!to_insert.contains(&other))
            } else if !to_insert.contains(&other) {
                // false positive
                false_positives += 1;
            }
        }
        let fp_rate = false_positives as f64 / tests as f64;
        assert!(fp_rate < 1.0); // This isn't really a test, but so that you have the `fp_rate` variable to print out, or evaluate
    }
}
