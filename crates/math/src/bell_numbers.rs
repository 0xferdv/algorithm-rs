use num_bigint::BigUint;
use num_traits::{Zero, One};

use std::sync::RwLock;


/// 计算组合数 C(n, r) = n! / (r! * (n-r)!)
///
/// # 参数
/// * `n` - 总数
/// * `r` - 选取数
///
/// # 返回值
/// 返回组合数 C(n, r) 的大整数表示
#[allow(unused)]
fn n_choose_r(n: u32, r: u32) -> BigUint {
    if r == n || r == 0 {
        return One::one();
    }
    if r > n {
        return Zero::zero();
    }
    // 使用递推公式 C(n,r) = C(n,r-1) * (n-r+1) / r 来避免大数阶乘计算
    let product: BigUint = (0..r).fold(BigUint::one(), |acc, x| {
        (acc * BigUint::from(n - x)) / BigUint::from(x + 1)
    });
    product
}

/// 内存查找表结构体，用于缓存贝尔数计算结果
struct MemTable {
    buffer: Vec<BigUint>
}

impl MemTable {

    /// 创建一个新的空查找表
    const fn new() -> Self {
        Self {
            buffer: Vec::new()
        }
    }

    /// 从查找表中获取指定索引的值
    ///
    /// # 参数
    /// * `n` - 要获取值的索引
    ///
    /// # 返回值
    /// 如果存在有效值则返回Some(值)，否则返回None
    fn get(&self, n: usize) -> Option<BigUint> {
        if n == 0 || n == 1 {
            Some(BigUint::one())
        } else if let Some(entry) = self.buffer.get(n) {
            if *entry == BigUint::zero() {
                None
            } else {
                Some(entry.clone())
            }
        } else {
            None
        }
    }

    /// 设置查找表中指定索引的值
    ///
    /// # 参数
    /// * `n` - 要设置值的索引
    /// * `b` - 要设置的大整数值
    fn set(&mut self, n: usize, b: BigUint) {
        self.buffer[n] = b;
    }

    /// 获取查找表的容量
    ///
    /// # 返回值
    /// 返回查找表的容量大小
    #[inline]
    fn capacity(&self) -> usize {
        self.buffer.capacity()
    }

    /// 调整查找表大小
    ///
    /// # 参数
    /// * `new_size` - 新的大小
    #[inline]
    fn resize(&mut self, new_size: usize) {
        if new_size > self.buffer.len() {
            self.buffer.resize(new_size, Zero::zero());
        }
    }
}

/// 全局静态查找表锁，用于线程安全地访问贝尔数缓存
static LOOKUP_TABLE_LOCK: RwLock<MemTable> = RwLock::new(MemTable::new());

/// 计算第n个贝尔数
/// 贝尔数表示将n个元素划分成若干非空子集的方法数
/// 使用递归公式: B(n+1) = Σ(k=0 to n) C(n,k) * B(k)
///
/// # 参数
/// * `n` - 要计算的贝尔数的序号
///
/// # 返回值
/// 返回第n个贝尔数的大整数表示
#[allow(unused)]
pub fn bell_number(n: u32) -> BigUint {
    let needs_resize;
    {
        // 先尝试从缓存中读取结果
        let lookup_table = LOOKUP_TABLE_LOCK.read().unwrap();
        if let Some(entry) = lookup_table.get(n as usize) {
            return entry;
        }
        needs_resize = (n + 1) as usize > lookup_table.capacity();
    }
    // 如果缓存空间不足，则扩展缓存
    if needs_resize {
        let mut lookup_table = LOOKUP_TABLE_LOCK.write().unwrap();
        lookup_table.resize((n + 1) as usize);
    }
    // 使用递归公式计算新的贝尔数
    let new_bell_number: BigUint = (0..n)
        .map(|x| bell_number(x) * n_choose_r(n - 1, x)).sum();
    {
        // 将计算结果存入缓存
        let mut lookup_table = LOOKUP_TABLE_LOCK.write().unwrap();
        lookup_table.set(n as usize, new_bell_number.clone());
    }
    new_bell_number
}

#[cfg(test)]
pub mod tests {
    use super::*;
    use std::str::FromStr;

    #[test]
    fn test_choose_zero() {
        for i in 1..100 {
            assert_eq!(n_choose_r(i, 0), One::one());
        }
    }

    #[test]
    fn test_combination() {
        let five_choose_1 = BigUint::from(5u32);
        assert_eq!(n_choose_r(5, 1), five_choose_1);
        assert_eq!(n_choose_r(5, 4), five_choose_1);

        let ten_choose_3 = BigUint::from(120u32);
        assert_eq!(n_choose_r(10, 3), ten_choose_3);
        assert_eq!(n_choose_r(10, 7), ten_choose_3);

        let fourty_two_choose_thirty = BigUint::from_str("11058116888").unwrap();
        assert_eq!(n_choose_r(42, 30), fourty_two_choose_thirty);
        assert_eq!(n_choose_r(42, 12), fourty_two_choose_thirty);
    }

    #[test]
    fn test_bell_numbers() {
        let bell_one = BigUint::from(1u32);
        assert_eq!(bell_number(1), bell_one);

        let bell_three = BigUint::from(5u32);
        assert_eq!(bell_number(3), bell_three);

        let bell_eight = BigUint::from(4140u32);
        assert_eq!(bell_number(8), bell_eight);

        let bell_six = BigUint::from(203u32);
        assert_eq!(bell_number(6), bell_six);

        let bell_twenty_six = BigUint::from_str("49631246523618756274").unwrap();
        assert_eq!(bell_number(26), bell_twenty_six);
    }
}
