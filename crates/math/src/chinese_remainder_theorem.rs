use crate::extended_euclidean_algorithm;

/// 计算模逆元
///
/// 使用扩展欧几里得算法计算 x 在模 n 意义下的逆元
///
/// # 参数
/// * `x` - 要求逆元的数
/// * `n` - 模数
///
/// # 返回值
/// 如果 x 和 n 互质，则返回 Some(逆元)，否则返回 None
fn mod_inv(x: i32, n: i32) -> Option<i32> {
    let (g, x, _) = extended_euclidean_algorithm(x, n);
    if g == 1 {
        return Some(((x % n) + n) % n);
    }
    None
}

/// 中国剩余定理求解器
///
/// 根据给定的余数数组和模数数组，求解满足同余方程组的最小正整数解
///
/// # 参数
/// * `residues` - 余数数组，表示 x ≡ residues[i] (mod mod_ulli[i])
/// * `mod_ulli` - 模数数组，各项必须两两互质
///
/// # 返回值
/// 如果存在解则返回 Some(解)，否则返回 None
#[allow(unused)]
pub fn chinese_remainder_theorem(residues: &[i32], mod_ulli: &[i32]) -> Option<i32> {
    // 计算所有模数的乘积
    let prod = mod_ulli.iter().product::<i32>();
    let mut sum = 0;

    // 遍历每个同余方程，累加计算结果
    for (&residue, &modulus) in residues.iter().zip(mod_ulli) {
        let p = prod / modulus;
        sum += residue * mod_inv(p, modulus)? * p
    }

    Some(sum % prod)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        assert_eq!(chinese_remainder_theorem(&[3, 5, 7], &[2, 3, 1]), Some(5));
        assert_eq!(chinese_remainder_theorem(&[1, 4, 6], &[3, 5, 7]), Some(34));
        assert_eq!(chinese_remainder_theorem(&[1, 4, 6], &[1, 2, 0]), None);
        assert_eq!(chinese_remainder_theorem(&[2, 5, 7], &[6, 9, 15]), None);
    }
}
