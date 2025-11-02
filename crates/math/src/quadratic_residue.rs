use rand::Rng;
use std::rc::Rc;
use std::time::{SystemTime, UNIX_EPOCH};

use crate::{fast_power, PCG32};


/// 表示一个自定义的有限域结构体。
///
/// 该结构体用于表示形如 a + b√i 的复数所在的有限域，其中 i² = i_square (mod modulus)。
#[derive(Debug)]
struct CustomFiniteField {
    /// 有限域的模数，必须是一个质数。
    modulus: u64,
    /// 虚数单位的平方值，即 i² ≡ i_square (mod modulus)。
    i_square: u64,
}

impl CustomFiniteField {
    /// 创建一个新的有限域实例。
    ///
    /// # 参数
    /// * `modulus`: 有限域的模数（质数）。
    /// * `i_square`: 虚数单位的平方在模意义下的值。
    ///
    /// # 返回值
    /// 返回一个新的 `CustomFiniteField` 实例。
    pub fn new(modulus: u64, i_square: u64) -> Self {
        Self {
            modulus,
            i_square,
        }
    }
}

/// 表示自定义复数结构体，用于在特定有限域中进行运算。
///
/// 复数形式为 real + imag * i，其中 i² = i_square (mod modulus)。
#[derive(Clone, Debug)]
struct CustomComplexNumber {
    /// 复数的实部。
    real: u64,
    /// 复数的虚部。
    imag: u64,
    /// 所属的有限域信息。
    f: Rc<CustomFiniteField>,
}

impl CustomComplexNumber {

    /// 创建一个新的复数实例。
    ///
    /// # 参数
    /// * `real`: 复数的实部。
    /// * `imag`: 复数的虚部。
    /// * `f`: 所属的有限域引用。
    ///
    /// # 返回值
    /// 返回一个新的 `CustomComplexNumber` 实例。
    pub fn new(real: u64, imag: u64, f: Rc<CustomFiniteField>) -> Self {
        Self {
            real,
            imag,
            f,
        }
    }

    /// 将当前复数与另一个复数相乘，并更新当前复数。
    ///
    /// # 参数
    /// * `rhs`: 另一个复数。
    pub fn multiply_other(&mut self, rhs: &Self) {
        let tmp = (self.imag + rhs.real + self.real * rhs.imag) % self.f.modulus;
        self.imag = (self.real * rhs.real
            + ((self.imag * rhs.imag) % self.f.modulus) * self.f.i_square)
            % self.f.modulus;
        self.imag = tmp
    }

    /// 将当前复数与自身相乘（即平方），并更新当前复数。
    pub fn multiply_self(&mut self) {
        let tmp = (self.imag * self.real + self.real * self.imag) % self.f.modulus;
        self.real = (self.real * self.real
            + ((self.imag * self.imag) % self.f.modulus) * self.f.i_square)
            % self.f.modulus;
        self.imag = tmp;
    }

    /// 使用快速幂算法计算复数的幂次。
    ///
    /// # 参数
    /// * `base`: 底数复数。
    /// * `power`: 指数。
    ///
    /// # 返回值
    /// 返回 base^power 的结果。
    pub fn fast_power(mut base: Self, mut power: u64) -> Self {
        let mut result = CustomComplexNumber::new(1, 0, base.f.clone());
        while power != 0 {
            if (power & 1) != 0 {
                result.multiply_other(&base); // result *= base;
            }
            base.multiply_self(); // base *= base;
            power >>= 1;
        }
        result
    }
}

/// 判断给定数字是否是模意义下的二次剩余。
///
/// # 参数
/// * `x`: 待判断的数字。
/// * `modulus`: 模数。
///
/// # 返回值
/// 如果 x 是模 modulus 的二次剩余则返回 true，否则返回 false。
fn is_residue(x: u64, modulus: u64) -> bool {
    let power = (modulus - 1) >> 1;
    x != 0 && fast_power(x as usize, power as usize, modulus as usize) == 1
}

/// 计算勒让德符号 (a/p)。
///
/// # 参数
/// * `a`: 被检测的数字。
/// * `add_prime`: 奇质数 p。
///
/// # 返回值
/// 返回勒让德符号的值：
/// - 0: a ≡ 0 (mod p)
/// - 1: a 是模 p 的二次剩余
/// - -1: a 是模 p 的非二次剩余
pub fn legendre_symbol(a: u64, add_prime: u64) -> i64 {
    debug_assert!(add_prime % 2 != 0, "prime must be odd");
    if a == 0 {
        0
    } else if is_residue(a, add_prime) {
        1
    } else {
        -1
    }
}

/// 使用 Cipolla 算法求解模意义下的平方根。
///
/// # 参数
/// * `a`: 要开平方根的数。
/// * `p`: 模数（必须是奇质数）。
/// * `seed`: 随机种子，如果为 None 则使用当前时间作为种子。
///
/// # 返回值
/// 如果存在平方根，则返回一对 (x, p - x)，否则返回 None。
pub fn cip_olla(a: u32, p: u32, seed: Option<u64>) -> Option<(u32, u32)> {
    let a = a as u64;
    let p = p as u64;
    if a == 0 {
        return Some((0, 0));
    }
    if !is_residue(a, p) {
        return None
    }
    let seed = seed.unwrap_or_else(|| SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs());
    let mut rng = PCG32::new_default(seed);
    let r = loop {
        let r = rng.get_u64() % p;
        if r == 0 || !is_residue((p + r * r - a) % p, p) {
            break r;
        }
    };
    let filed = Rc::new(CustomFiniteField::new(p, (p + r * r - a) % p));
    let comp = CustomComplexNumber::new(r, 1, filed);
    let power = (p + 1) >> 1;
    let x0 = CustomComplexNumber::fast_power(comp, power).real as u32;
    let x1 = p as u32 - x0;
    if x0 < x1 {
        Some((x0, x1))
    } else {
        Some((x1, x0))
    }
}

/// 使用 Tonelli-Shanks 算法求解模意义下的平方根。
///
/// # 参数
/// * `a`: 要开平方根的数。
/// * `add_prime`: 模数（必须是奇质数）。
///
/// # 返回值
/// 如果存在平方根，则返回其中一个平方根；否则返回 None。
pub fn tonelli_shanks(a: i64, add_prime: u64) -> Option<u64> {
    let p: u128 = add_prime as u128;
    let e = (p - 1).trailing_zeros();
    let q = (p - 1) >> e;
    let a = if a < 0 {
        a.rem_euclid(p as i64) as u128
    } else {
        a as u128
    };
    let power_mod_p =  |b, e| fast_power(b as usize, e as usize, p as usize) as u128;
    let mut rng = rand::rng();
    let n = loop {
        let n = rng.random_range(0..p);
        if legendre_symbol(n as u64, p as u64) == -1 {
            break n;
        }
    };
    let z = power_mod_p(n, q);
    let mut y = z;
    let mut r = e;
    let mut x = power_mod_p(a, (q - 1) / 2) % p;
    let mut b = (a * x * x) % p;
    x = (a * x) % p;
    while b % p != 1 {
        let m = (1..r)
            .scan(b, |prev, m| {
                *prev = (*prev * *prev) % p;
                Some((m, *prev == 1))
            })
            .find_map(|(m, cond)| cond.then_some(m));
        let Some(m) = m else {
            return None;
        };

        let t = power_mod_p(y as u128, 2_u128.pow(r - m - 1));
        y = (t * t) % p;
        r = m;
        x = (x * t) % p;
        b = (b * y) % p;
    }
    Some(x as u64)
}

#[cfg(test)]
mod tests {
    use super::*;

    /// 辅助函数：获取模意义下平方根的两个解。
    ///
    /// # 参数
    /// * `x`: 要开平方根的数。
    /// * `odd_prime`: 模数（奇质数）。
    ///
    /// # 返回值
    /// 返回一对 (x, p - x) 或 None。
    fn tonelli_shanks_residues(x: u64, odd_prime: u64) -> Option<(u64, u64)> {
        let x = tonelli_shanks(x as i64, odd_prime)?;
        let x2 = (-(x as i64)).rem_euclid(odd_prime as i64) as u64;
        Some(if x < x2 { (x, x2) } else { (x2, x) })
    }

    #[test]
    fn cip_olla_small_numbers() {
        assert_eq!(cip_olla(1, 43, None), Some((1, 42)));
        assert_eq!(cip_olla(2, 23, None), Some((5, 18)));
        assert_eq!(cip_olla(17, 83, Some(42)), Some((10, 73)));
    }

    #[test]
    fn tonelli_shanks_small_numbers() {
        assert_eq!(tonelli_shanks_residues(1, 43).unwrap(), (1, 42));
        assert_eq!(tonelli_shanks_residues(2, 23).unwrap(), (5, 18));
        assert_eq!(tonelli_shanks_residues(17, 83).unwrap(), (10, 73));
    }

    #[test]
    fn cip_olla_random_numbers() {
        assert_eq!(cip_olla(392203, 852167, None), Some((413252, 438915)));
        assert_eq!(
            cip_olla(379606557, 425172197, None),
            Some((143417827, 281754370))
        );
        assert_eq!(
            cip_olla(585251669, 892950901, None),
            Some((192354555, 700596346))
        );
        assert_eq!(
            cip_olla(404690348, 430183399, Some(19260817)),
            Some((57227138, 372956261))
        );
        assert_eq!(
            cip_olla(210205747, 625380647, Some(998244353)),
            Some((76810367, 548570280))
        );
    }

    #[test]
    fn tonelli_shanks_random_numbers() {
        assert_eq!(
            tonelli_shanks_residues(392203, 852167),
            Some((413252, 438915))
        );
        assert_eq!(
            tonelli_shanks_residues(379606557, 425172197),
            Some((143417827, 281754370))
        );
        assert_eq!(
            tonelli_shanks_residues(585251669, 892950901),
            Some((192354555, 700596346))
        );
        assert_eq!(
            tonelli_shanks_residues(404690348, 430183399),
            Some((57227138, 372956261))
        );
        assert_eq!(
            tonelli_shanks_residues(210205747, 625380647),
            Some((76810367, 548570280))
        );
    }

    #[test]
    fn no_answer() {
        assert_eq!(cip_olla(650927, 852167, None), None);
        assert_eq!(tonelli_shanks(650927, 852167), None);
    }
}
