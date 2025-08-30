use std::ops::{Add, Mul, MulAssign, Sub};

/// 表示一个64位浮点数的复数类型。
///
/// 复数由实部 `re` 和虚部 `im` 组成。
#[derive(Clone, Copy, Debug)]
pub struct Complex64 {
    /// 实部
    pub re: f64,
    /// 虚部
    pub im: f64,
}

impl Complex64 {

    /// 创建一个新的复数。
    ///
    /// # 参数
    /// * `re` - 实部
    /// * `im` - 虚部
    ///
    /// # 返回值
    /// 返回一个新的 `Complex64` 实例。
    #[inline]
    #[allow(unused)]
    pub fn new(re: f64, im: f64) -> Self {
        Self { re, im }
    }

    /// 计算复数的模的平方。
    ///
    /// # 返回值
    /// 返回复数模的平方，即 `re^2 + im^2`。
    #[inline]
    #[allow(unused)]
    pub fn square_norm(&self) -> f64 {
        self.re * self.re + self.im * self.im
    }

    /// 计算复数的模。
    ///
    /// # 返回值
    /// 返回复数的模，即 `sqrt(re^2 + im^2)`。
    #[inline]
    #[allow(unused)]
    pub fn norm(&self) -> f64 {
        self.square_norm().sqrt()
    }

    /// 计算复数的倒数。
    ///
    /// # 返回值
    /// 返回当前复数的倒数。
    #[inline]
    #[allow(unused)]
    pub fn inverse(&self) -> Complex64 {
        let nrm = self.square_norm();
        Self {
            re: self.re / nrm,
            im: -self.im / nrm,
        }
    }
}

impl Default for Complex64 {

    /// 返回默认的复数实例，实部和虚部都为 0.0。
    ///
    /// # 返回值
    /// 返回一个 `Complex64` 实例，其中 `re = 0.0`, `im = 0.0`。
    #[inline]
    fn default() -> Self {
        Self { re: 0.0, im: 0.0 }
    }
}

impl Add<Complex64> for Complex64 {
    type Output = Complex64;

    /// 实现两个复数的加法运算。
    ///
    /// # 参数
    /// * `rhs` - 右操作数
    ///
    /// # 返回值
    /// 返回两个复数相加的结果。
    #[inline]
    fn add(self, rhs: Self) -> Self::Output {
        Self {
            re: self.re + rhs.re,
            im: self.im + rhs.im,
        }
    }
}


impl Sub for Complex64 {
    type Output = Complex64;

    /// 实现两个复数的减法运算。
    ///
    /// # 参数
    /// * `rhs` - 右操作数
    ///
    /// # 返回值
    /// 返回两个复数相减的结果。
    #[inline]
    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            re: self.re - rhs.re,
            im: self.im - rhs.im,
        }
    }
}

impl Mul<Complex64> for Complex64 {
    type Output = Complex64;

    /// 实现两个复数的乘法运算。
    ///
    /// # 参数
    /// * `rhs` - 右操作数
    ///
    /// # 返回值
    /// 返回两个复数相乘的结果。
    #[inline]
    fn mul(self, rhs: Self) -> Self::Output {
        Self {
            re: self.re * rhs.re - self.im * rhs.im,
            im: self.re * rhs.im + self.im * rhs.re,
        }
    }
}

impl MulAssign<Complex64> for Complex64 {

    /// 实现复数的乘法赋值运算。
    ///
    /// # 参数
    /// * `other` - 要相乘的复数
    #[inline]
    fn mul_assign(&mut self, other: Self) {
        let tmp = self.re * other.im + self.im * other.re;
        self.re = self.re * other.re - self.im * other.im;
        self.im = tmp;
    }
}

/// 计算快速傅里叶变换（FFT）所需的输入置换数组。
///
/// 该函数用于生成输入数据的位反转排列，这是FFT算法中常用的预处理步骤。
///
/// # 参数
/// * `length` - 输入数据的长度，必须是2的幂次
///
/// # 返回值
/// 返回一个表示输入数据位反转排列的索引向量。
#[allow(unused)]
pub fn fast_fourier_transform_input_permutation(length: usize) -> Vec<usize> {
    let mut result = Vec::new();
    result.reserve_exact(length);
    for i in 0..length {
        result.push(i);
    }
    let mut reverse = 0_usize;
    let mut position = 1_usize;
    while position < length {
        let mut bit = length >> 1;
        while bit & reverse != 0 {
            reverse ^= bit;
            bit >>= 1;
        }
        reverse ^= bit;
        if position < reverse {
            result.swap(position, reverse);
        }
        position += 1;
    }
    result
}

/// 执行快速傅里叶变换（FFT）。
///
/// 该函数使用Cooley-Tukey算法实现FFT，将时域信号转换为频域信号。
///
/// # 参数
/// * `input` - 输入的实数序列
/// * `input_permutation` - 输入数据的置换数组，通常由 `fast_fourier_transform_input_permutation` 生成
///
/// # 返回值
/// 返回输入序列的FFT结果，每个元素都是一个复数。
pub fn fast_fourier_transform(input: &[f64], input_permutation: &[usize]) -> Vec<Complex64> {
    let n = input.len();
    let mut result = Vec::new();
    result.reserve_exact(n);
    for position in input_permutation {
        result.push(Complex64::new(input[*position], 0.0));
    }

    // 迭代地执行FFT的各个阶段
    let mut segment_length = 1_usize;
    while segment_length < n {
        segment_length <<= 1;
        let angle: f64 = std::f64::consts::TAU / segment_length as f64;
        let w_len = Complex64::new(angle.cos(), angle.sin());

        // 处理每个段
        for segment_start in (0..n).step_by(segment_length) {
            let mut w = Complex64::new(1.0, 0.0);

            // 处理段内的每对元素
            for position in segment_start..(segment_start + segment_length / 2) {
                let a = result[position];
                let b = result[position + segment_length / 2] * w;
                result[position] = a + b;
                result[position + segment_length / 2] = a - b;
                w *= w_len;
            }
        }
    }
    result
}

/// 执行逆快速傅里叶变换（IFFT）。
///
/// 该函数将频域信号转换回时域信号。
///
/// # 参数
/// * `input` - 输入的复数序列（频域信号）
/// * `input_permutation` - 输入数据的置换数组，通常由 `fast_fourier_transform_input_permutation` 生成
///
/// # 返回值
/// 返回IFFT结果的实数序列，并进行归一化处理。
#[allow(unused)]
pub fn inverse_fast_fourier_transform(
    input: &[Complex64],
    input_permutation: &[usize],
) -> Vec<f64> {
    let n = input.len();
    let mut result = Vec::new();
    result.reserve_exact(n);
    for position in input_permutation {
        result.push(input[*position]);
    }

    // 迭代地执行IFFT的各个阶段
    let mut segment_length = 1_usize;
    while segment_length < n {
        segment_length <<= 1;
        let angle: f64 = -std::f64::consts::TAU / segment_length as f64;
        let w_len = Complex64::new(angle.cos(), angle.sin());

        // 处理每个段
        for segment_start in (0..n).step_by(segment_length) {
            let mut w = Complex64::new(1.0, 0.0);

            // 处理段内的每对元素
            for position in segment_start..(segment_start + segment_length / 2) {
                let a = result[position];
                let b = result[position + segment_length / 2] * w;
                result[position] = a + b;
                result[position + segment_length / 2] = a - b;
                w *= w_len;
            }
        }
    }

    // 归一化结果
    let scale = 1.0 / n as f64;
    result.iter().map(|x| x.re * scale).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn almost_equal(a: f64, b: f64, epsilon: f64) -> bool {
        (a - b).abs() < epsilon
    }

    const EPSILON: f64 = 1e-6;

    #[test]
    fn small_polynomial_returns_self() {
        let polynomial = vec![1.0f64, 1.0, 0.0, 2.5];
        let permutation = fast_fourier_transform_input_permutation(polynomial.len());
        let fft = fast_fourier_transform(&polynomial, &permutation);
        let ifft = inverse_fast_fourier_transform(&fft, &permutation);
        for (x, y) in ifft.iter().zip(polynomial.iter()) {
            assert!(almost_equal(*x, *y, EPSILON));
        }
    }

    #[test]
    fn square_small_polynomial() {
        let mut polynomial = vec![1.0f64, 1.0, 0.0, 2.0];
        polynomial.append(&mut vec![0.0; 4]);
        let permutation = fast_fourier_transform_input_permutation(polynomial.len());
        let mut fft = fast_fourier_transform(&polynomial, &permutation);
        fft.iter_mut().for_each(|num| *num *= *num);
        let ifft = inverse_fast_fourier_transform(&fft, &permutation);
        let expected = [1.0, 2.0, 1.0, 4.0, 4.0, 0.0, 4.0, 0.0, 0.0];
        for (x, y) in ifft.iter().zip(expected.iter()) {
            assert!(almost_equal(*x, *y, EPSILON));
        }
    }

    #[test]
    #[ignore]
    fn square_big_polynomial() {
        let n = 1 << 17; // ~100_000
        let mut polynomial = vec![1.0f64; n];
        polynomial.append(&mut vec![0.0f64; n]);
        let permutation = fast_fourier_transform_input_permutation(polynomial.len());
        let mut fft = fast_fourier_transform(&polynomial, &permutation);
        fft.iter_mut().for_each(|num| *num *= *num);
        let ifft = inverse_fast_fourier_transform(&fft, &permutation);
        let expected = (0..((n << 1) - 1)).map(|i| std::cmp::min(i + 1, (n << 1) - 1 - i) as f64);
        for (&x, y) in ifft.iter().zip(expected) {
            assert!(almost_equal(x, y, EPSILON));
        }
    }
}
