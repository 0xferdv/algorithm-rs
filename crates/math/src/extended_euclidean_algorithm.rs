/// 执行扩展欧几里得算法中的一步更新操作。
///
/// 此函数用于在扩展欧几里得算法中更新两个变量的值，按照公式：
/// a = old_a - quotient * a
/// old_a = a（更新前的值）
///
/// # 参数
/// * `a`: 当前值的可变引用
/// * `old_a`: 旧值的可变引用
/// * `quotient`: 商值，用于计算新的a值
fn update_step(a: &mut i32, old_a: &mut i32, quotient: i32) {
    let temp = *a;
    *a = *old_a - quotient * temp;
    *old_a = temp;
}

/// 使用扩展欧几里得算法计算两个整数的最大公约数及其贝祖系数。
///
/// 扩展欧几里得算法不仅计算两个整数a和b的最大公约数gcd(a,b)，
/// 还能找到满足等式 a*s + b*t = gcd(a,b) 的整数系数s和t。
///
/// # 参数
/// * `a`: 第一个整数
/// * `b`: 第二个整数
///
/// # 返回值
/// 返回一个三元组(gcd, s, t)，其中：
/// * `gcd`: a和b的最大公约数
/// * `s`: 满足等式 a*s + b*t = gcd 的系数s
/// * `t`: 满足等式 a*s + b*t = gcd 的系数t
#[allow(unused)]
pub fn extended_euclidean_algorithm(a: i32, b: i32) -> (i32, i32, i32) {
    // 初始化算法所需变量
    // r序列：余数序列，初始为a和b
    let (mut old_r, mut rem) = (a, b);
    // s序列：a的系数序列，初始为1和0
    let (mut old_s, mut coff_s) = (1, 0);
    // t序列：b的系数序列，初始为0和1
    let (mut old_t, mut coff_t) = (0, 1);

    // 当余数不为0时继续迭代
    while rem != 0 {
        // 计算商值
        let quotient = old_r / rem;
        // 更新余数序列
        update_step(&mut rem, &mut old_r, quotient);
        // 更新s系数序列
        update_step(&mut coff_s, &mut old_s, quotient);
        // 更新t系数序列
        update_step(&mut coff_t, &mut old_t, quotient);
    }
    // 返回最终结果：最大公约数和对应的贝祖系数
    (old_r, old_s, old_t)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        assert_eq!(extended_euclidean_algorithm(101, 13), (1, 4, -31));
        assert_eq!(extended_euclidean_algorithm(123, 19), (1, -2, 13));
        assert_eq!(extended_euclidean_algorithm(25, 36), (1, 13, -9));
        assert_eq!(extended_euclidean_algorithm(69, 54), (3, -7, 9));
        assert_eq!(extended_euclidean_algorithm(55, 79), (1, 23, -16));
        assert_eq!(extended_euclidean_algorithm(33, 44), (11, -1, 1));
        assert_eq!(extended_euclidean_algorithm(50, 70), (10, 3, -2));
    }
}
