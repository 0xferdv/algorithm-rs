/// PCG32 是一个基于 PCG (Permuted Congruential Generator) 算法的伪随机数生成器。
/// 它使用 64 位状态和线性同余生成器（LCG）来生成高质量的伪随机数。
pub struct PCG32 {
    /// 当前生成器的状态值，用于生成下一个随机数。
    state: u64,
    /// LCG 的乘数参数。
    multiplier: u64,
    /// LCG 的增量参数，必须为奇数以保证周期长度。
    increment: u64,
}

/// PCG32 算法默认使用的乘数常量。
pub const PCG32_MULTIPLIER: u64 = 6364136223846793005_u64;

/// PCG32 算法默认使用的增量常量。
pub const PCG32_INCREMENT: u64 = 1442695040888963407_u64;

/// 提供对 PCG32 实例的可变迭代器封装。
pub struct IterMut<'a> {
    /// 指向被迭代的 PCG32 实例的可变引用。
    pcg: &'a mut PCG32,
}

impl PCG32 {
    /// 创建一个新的 PCG32 实例。
    ///
    /// # 参数
    /// * `seed` - 用于初始化状态的种子值。
    /// * `multiplier` - LCG 使用的乘数。
    /// * `stream` - 流选择参数，将被左移一位并加一以确保为奇数。
    ///
    /// # 返回值
    /// 返回初始化后的 PCG32 实例。
    pub fn new(seed: u64, multiplier: u64, stream: u64) -> Self {
        let increment = (stream << 1) | 1;
        let mut pcg = PCG32 {
            state: seed.wrapping_add(increment),
            multiplier,
            increment,
        };
        pcg.next();
        pcg
    }

    /// 使用默认参数创建一个新的 PCG32 实例。
    ///
    /// # 参数
    /// * `seed` - 用于初始化状态的种子值。
    ///
    /// # 返回值
    /// 返回使用默认乘数和增量初始化后的 PCG32 实例。
    pub fn new_default(seed: u64) -> Self {
        let increment = PCG32_INCREMENT;
        let multiplier = PCG32_MULTIPLIER;
        let mut pcg = PCG32 {
            state: seed.wrapping_add(increment),
            multiplier,
            increment,
        };
        pcg.next();
        pcg
    }

    /// 执行一次线性同余生成器步骤，更新内部状态。
    #[inline]
    pub fn next(&mut self) {
        self.state = self
            .state
            .wrapping_mul(self.multiplier)
            .wrapping_add(self.increment);
    }

    /// 快进或后退指定步数的状态。
    ///
    /// # 参数
    /// * `delta` - 要前进或后退的步数（无符号整数）。
    #[inline]
    pub fn advance(&mut self, mut delta: u64) {
        // 使用快速幂算法计算等效变换
        let mut acc_mult = 1u64;
        let mut acc_incr = 0u64;
        let mut curr_mlt = self.multiplier;
        let mut curr_inc = self.increment;
        while delta > 0 {
            if delta & 1 != 0 {
                acc_mult = acc_mult.wrapping_mul(curr_mlt);
                acc_incr = acc_incr.wrapping_mul(curr_mlt).wrapping_add(curr_inc);
            }
            curr_inc = curr_mlt.wrapping_add(1).wrapping_mul(curr_inc);
            curr_mlt = curr_mlt.wrapping_mul(curr_mlt);
            delta >>= 1;
        }
        self.state = acc_mult.wrapping_mul(self.state).wrapping_add(acc_incr);
    }

    /// 生成下一个 32 位无符号整数随机数。
    ///
    /// # 返回值
    /// 返回一个 32 位无符号整数随机数。
    #[inline]
    pub fn get_u32(&mut self) -> u32 {
        let mut x = self.state;
        let count = (x >> 59) as u32;
        self.next();
        x ^= x >> 18;
        ((x >> 27) as u32).rotate_right(count)
    }

    /// 生成下一个 64 位无符号整数随机数。
    ///
    /// # 返回值
    /// 返回一个 64 位无符号整数随机数。
    #[inline]
    pub fn get_u64(&mut self) -> u64 {
        self.get_u32() as u64 ^ ((self.get_u32() as u64) << 32)
    }

    /// 生成两个 16 位无符号整数随机数。
    ///
    /// # 返回值
    /// 返回包含两个 16 位无符号整数的元组。
    #[inline]
    pub fn get_u16(&mut self) -> (u16, u16) {
        let res = self.get_u32();
        (res as u16, (res >> 16) as u16)
    }

    /// 生成四个 8 位无符号整数随机数。
    ///
    /// # 返回值
    /// 返回包含四个 8 位无符号整数的元组。
    #[inline]
    pub fn get_u8(&mut self) -> (u8, u8, u8, u8) {
        let res = self.get_u32();
        (
            res as u8,
            (res >> 8) as u8,
            (res >> 16) as u8,
            (res >> 24) as u8,
        )
    }

    /// 获取当前生成器的状态值。
    ///
    /// # 返回值
    /// 返回当前的状态值。
    #[inline]
    pub fn get_state(&self) -> u64 {
        self.state
    }

    /// 创建一个可变迭代器，用于连续获取随机数。
    ///
    /// # 返回值
    /// 返回一个指向自身可变引用的 IterMut 结构体。
    pub fn iter_mut(&mut self) -> IterMut<'_> {
        IterMut { pcg: self }
    }
}

impl Iterator for IterMut<'_> {
    type Item = u32;

    /// 获取下一个 32 位随机数。
    ///
    /// # 返回值
    /// 返回 Some(u32) 类型的随机数。
    fn next(&mut self) -> Option<Self::Item> {
        Some(self.pcg.get_u32())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn no_birthday() {
        let numbers = 1e5 as usize;
        let mut pcg = PCG32::new_default(314159);
        let mut pcg2 = PCG32::new_default(314159);
        assert_eq!(pcg.get_u32(), pcg2.get_u32());
        let mut randoms: Vec<u32> = pcg.iter_mut().take(numbers).collect::<Vec<u32>>();
        pcg2.advance(1000);
        assert_eq!(pcg2.get_u32(), randoms[1000]);
        pcg2.advance((-1001_i64) as u64);
        assert_eq!(pcg2.get_u32(), randoms[0]);
        randoms.sort_unstable();
        randoms.dedup();
        assert_eq!(randoms.len(), numbers);
    }
}
