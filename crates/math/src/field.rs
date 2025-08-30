use core::fmt;
use std::hash::{Hash, Hasher};
use std::ops::{Add, Div, Mul, Neg, Sub};

/// 定义一个数学域（Field）的 trait，表示支持基本算术运算的代数结构。
/// 该 trait 继承了多个标准库 trait，包括负号、加法、减法、乘法、除法、复制、相等比较和调试输出。
///
/// 常量：
/// - `ONE`: 表示域中的乘法单位元。
/// - `ZERO`: 表示域中的加法单位元。
/// - `CHARACTERISTIC`: 表示域的特征（通常是素数）。
///
/// 类型：
/// - `ElementsIter`: 迭代器类型，用于遍历域中所有元素。
///
/// 方法：
/// - `inverse(self) -> Self`: 计算当前元素的乘法逆元。
/// - `integer_mul(self, a: i64) -> Self`: 将当前元素与整数相乘。
/// - `from_integer(a: i64) -> Self`: 从整数构造域元素（默认实现基于 ONE 和 integer_mul）。
/// - `elements() -> Self::ElementsIter`: 返回一个迭代器，用于遍历域中所有元素。
pub trait Field:
    Neg<Output = Self>
    + Add<Output = Self>
    + Sub<Output = Self>
    + Mul<Output = Self>
    + Div<Output = Self>
    + Copy
    + Eq
    + fmt::Debug
{
    const ONE: Self;
    const ZERO: Self;
    const CHARACTERISTIC: u64;

    type ElementsIter: Iterator<Item = Self>;

    fn inverse(self) -> Self;
    fn integer_mul(self, a: i64) -> Self;
    fn from_integer(a: i64) -> Self {
        Self::ONE.integer_mul(a)
    }
    fn elements() -> Self::ElementsIter;
}

/// 表示一个有限域（素域），其模数由泛型参数 P 指定。
///
/// 字段：
/// - `a`: 存储域元素的值（在模 P 下）。
#[derive(Clone, Copy)]
pub struct PrimeField<const P: u64> {
    a: i64,
}

impl<const P: u64> PrimeField<P> {

    /// 对当前域元素进行模约简，确保其值在 [0, P) 范围内。
    ///
    /// 返回值：
    /// - 约简后的 PrimeField 实例。
    fn reduce(self) -> Self {
        let Self { a } = self;
        let p: i64 = P.try_into().expect("module not fitting into singed 64 bit.");
        let a = a.rem_euclid(p);
        assert!(a >= 0);
        Self { a }
    }

    /// 将当前域元素转换为对应的无符号 64 位整数。
    ///
    /// 返回值：
    /// - 当前域元素的整数值（已约简）。
    pub fn to_integer(&self) -> u64 {
        self.reduce().a as u64
    }
}

/// 实现从 i64 到 PrimeField 的转换。
impl<const P: u64> From<i64> for PrimeField<P> {

    /// 从给定的 i64 值创建一个新的 PrimeField 元素。
    ///
    /// 参数：
    /// - `a`: 要转换的整数值。
    ///
    /// 返回值：
    /// - 新的 PrimeField 实例。
    fn from(a: i64) -> Self {
        Self { a }
    }
}

/// 实现 PrimeField 的相等性比较。
impl<const P: u64> PartialEq for PrimeField<P> {
    /// 比较两个 PrimeField 元素是否相等（通过约简后比较）。
    ///
    /// 参数：
    /// - `other`: 另一个 PrimeField 元素。
    ///
    /// 返回值：
    /// - 如果两个元素相等则返回 true，否则返回 false。
    fn eq(&self, other: &Self) -> bool {
        self.reduce().a == other.reduce().a
    }
}

/// 实现 PrimeField 的相等性 trait。
impl<const P: u64> Eq for PrimeField<P> {}

/// 实现 PrimeField 的负号操作。
impl<const P: u64> Neg for PrimeField<P> {
    type Output = Self;

    /// 计算当前元素的负数（即加法逆元）。
    ///
    /// 返回值：
    /// - 当前元素的负数。
    fn neg(self) -> Self::Output {
        Self { a: -self.a }
    }
}

/// 实现 PrimeField 的加法操作。
impl<const P: u64> Add for PrimeField<P> {
    type Output = Self;

    /// 将两个 PrimeField 元素相加。
    ///
    /// 参数：
    /// - `rhs`: 右侧操作数。
    ///
    /// 返回值：
    /// - 相加结果。
    fn add(self, rhs: Self) -> Self::Output {
        Self {
            a: self.a.checked_add(rhs.a).unwrap_or_else(|| {
                let x = self.reduce();
                let y = rhs.reduce();
                x.a + y.a
            }),
        }
    }
}

/// 实现 PrimeField 的减法操作。
impl<const P: u64> Sub for PrimeField<P> {
    type Output = Self;

    /// 将两个 PrimeField 元素相减。
    ///
    /// 参数：
    /// - `rhs`: 右侧操作数。
    ///
    /// 返回值：
    /// - 相减结果。
    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            a: self.a.checked_sub(rhs.a).unwrap_or_else(|| {
                let x = self.reduce();
                let y = rhs.reduce();
                x.a - y.a
            }),
        }
    }
}

/// 实现 PrimeField 的乘法操作。
impl<const P: u64> Mul for PrimeField<P> {
    type Output = Self;

    /// 将两个 PrimeField 元素相乘。
    ///
    /// 参数：
    /// - `rhs`: 右侧操作数。
    ///
    /// 返回值：
    /// - 相乘结果。
    fn mul(self, rhs: Self) -> Self::Output {
        Self {
            a: self.a.checked_mul(rhs.a).unwrap_or_else(|| {
                let x = self.reduce();
                let y = rhs.reduce();
                x.a * y.a
            }),
        }
    }
}

/// 实现 PrimeField 的除法操作。
impl<const P: u64> Div for PrimeField<P> {
    type Output = Self;

    /// 将两个 PrimeField 元素相除（通过乘以右侧元素的逆元实现）。
    ///
    /// 参数：
    /// - `rhs`: 右侧操作数。
    ///
    /// 返回值：
    /// - 相除结果。
    #[allow(clippy::suspicious_arithmetic_impl)]
    fn div(self, rhs: Self) -> Self::Output {
        self * rhs.inverse()
    }
}

/// 实现 PrimeField 的调试输出格式。
impl<const P: u64> fmt::Debug for PrimeField<P> {
    /// 格式化 PrimeField 元素以便调试输出。
    ///
    /// 参数：
    /// - `f`: 格式化器。
    ///
    /// 返回值：
    /// - 格式化结果。
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let x = self.reduce();
        write!(f, "{}", x.reduce().a)
    }
}

/// 实现 PrimeField 对 Field trait 的具体实现。
impl<const P: u64> Field for PrimeField<P> {
    const CHARACTERISTIC: u64 = P;
    const ZERO: Self = Self { a: 0 };
    const ONE: Self = Self { a: 1 };

    /// 计算当前元素的乘法逆元。
    ///
    /// 返回值：
    /// - 当前元素的乘法逆元。
    fn inverse(self) -> Self {
        assert_ne!(self.a, 0);
        Self {
            a: mod_inverse(
                self.a,
                P.try_into().expect("module not fitting into singed 64 bit.")
            )
        }
    }

    /// 将当前元素与整数相乘。
    ///
    /// 参数：
    /// - `n`: 要相乘的整数。
    ///
    /// 返回值：
    /// - 相乘结果。
    fn integer_mul(self, mut n: i64) -> Self {
        if n == 0 {
            return Self::ZERO;
        }
        let mut x = self;
        if n < 0 {
            x = -x;
            n = -n;
        }
        let mut y = Self::ZERO;
        while n > 1 {
            if n % 2 == 1 {
                y = y + x;
                n -= 1;
            }
            x = x + x;
            n /= 2;
        }
        x + y
    }

    type ElementsIter = PrimeFieldElementsIter<P>;

    /// 返回一个迭代器，用于遍历域中所有元素。
    ///
    /// 返回值：
    /// - 迭代器实例。
    fn elements() -> Self::ElementsIter {
        PrimeFieldElementsIter::default()
    }
}

/// 用于遍历 PrimeField 中所有元素的迭代器。
#[derive(Default)]
pub struct PrimeFieldElementsIter<const P: u64> {
    x: i64
}

/// 实现 PrimeFieldElementsIter 的迭代器功能。
impl<const P: u64> Iterator for PrimeFieldElementsIter<P> {
    type Item = PrimeField<P>;

    /// 获取下一个元素。
    ///
    /// 返回值：
    /// - 下一个 PrimeField 元素，如果没有更多元素则返回 None。
    fn next(&mut self) -> Option<Self::Item> {
        if self.x as u64 == P {
            None
        } else {
            let res = PrimeField::from_integer(self.x);
            self.x += 1;
            Some(res)
        }
    }
}

/// 实现 PrimeField 的哈希功能。
impl<const P: u64> Hash for PrimeField<P> {
    /// 将当前元素加入到哈希器中。
    ///
    /// 参数：
    /// - `state`: 哈希器状态。
    fn hash<H: Hasher>(&self, state: &mut H) {
        let Self { a } = self.reduce();
        state.write_i64(a)
    }
}

/// 计算模逆元（扩展欧几里得算法）。
///
/// 参数：
/// - `a`: 要求逆元的数。
/// - `b`: 模数。
///
/// 返回值：
/// - `a` 在模下的逆元。
fn mod_inverse(mut a: i64, mut b: i64) -> i64 {
    let mut s = 1;
    let mut t = 0;
    let step = |x, y, q| (y, x - q * y);
    while b != 0 {
        let q = a / b;
        (a, b) = step(a, b, q);
        (s, t) = step(s, t, q);
    }
    assert!(a == 1 || a == -1);
    a * s
}

#[cfg(test)]
mod tests {

    use super::*;

    // #[test]
    // fn test_field_elements() {
    //     fn test<const P: u64>() {
    //         let expected: HashSet<PrimeField<P>> = (0..P as i64).map(Into::into).collect();
    //         for gen in 1..P - 1 {
    //             // every field element != 0 generates the whole field additively
    //             let gen = PrimeField::from(gen as i64);
    //             let mut generated: HashSet<PrimeField<P>> = std::iter::once(gen).collect();
    //             let mut x = gen;
    //             for _ in 0..P {
    //                 x = x + gen;
    //                 generated.insert(x);
    //             }
    //             assert_eq!(generated, expected);
    //         }
    //     }
    //     test::<5>();
    //     test::<7>();
    //     test::<11>();
    //     test::<13>();
    //     test::<17>();
    //     test::<19>();
    //     test::<23>();
    //     test::<71>();
    //     test::<101>();
    // }

    #[test]
    fn large_prime_field() {
        const P: u64 = 2_u64.pow(63) - 25;
        type F = PrimeField<P>;
        let x = F::from(P as i64 - 1);
        let y = x.inverse();
        assert_eq!(x * y, F::ONE);
    }

    #[test]
    fn inverse() {
        fn test<const P: u64>() {
            for x in -7..7 {
                let x = PrimeField::<P>::from(x);
                if x != PrimeField::ZERO {
                    assert_eq!(x.inverse() * x, PrimeField::ONE);
                    assert_eq!(x * x.inverse(), PrimeField::ONE);
                    assert_eq!((x.inverse().a * x.a).rem_euclid(P as i64), 1);
                    assert_eq!(x / x, PrimeField::ONE);
                }
                assert_eq!(x + (-x), PrimeField::ZERO);
                assert_eq!((-x) + x, PrimeField::ZERO);
                assert_eq!(x - x, PrimeField::ZERO);
            }
        }
        test::<5>();
        test::<7>();
        test::<11>();
        test::<13>();
        test::<17>();
        test::<19>();
        test::<23>();
        test::<71>();
        test::<101>();
    }

    #[test]
    fn test_mod_inverse() {
        assert_eq!(mod_inverse(-6, 7), 1);
        assert_eq!(mod_inverse(-5, 7), -3);
        assert_eq!(mod_inverse(-4, 7), -2);
        assert_eq!(mod_inverse(-3, 7), 2);
        assert_eq!(mod_inverse(-2, 7), 3);
        assert_eq!(mod_inverse(-1, 7), -1);
        assert_eq!(mod_inverse(1, 7), 1);
        assert_eq!(mod_inverse(2, 7), -3);
        assert_eq!(mod_inverse(3, 7), -2);
        assert_eq!(mod_inverse(4, 7), 2);
        assert_eq!(mod_inverse(5, 7), 3);
        assert_eq!(mod_inverse(6, 7), -1);
    }

    #[test]
    fn integer_mul() {
        type F = PrimeField<23>;
        for x in 0..23 {
            let x = F { a: x };
            for n in -7..7 {
                assert_eq!(x.integer_mul(n), F { a: n * x.a });
            }
        }
    }

    #[test]
    fn from_integer() {
        type F = PrimeField<23>;
        for x in -100..100 {
            assert_eq!(F::from_integer(x), F { a: x });
        }
        assert_eq!(F::from(0), F::ZERO);
        assert_eq!(F::from(1), F::ONE);
    }
}
