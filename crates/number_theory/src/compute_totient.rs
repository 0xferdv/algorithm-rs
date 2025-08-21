/// 计算欧拉函数（Euler's totient function）值的函数
///
/// 该函数计算从1到n的所有正整数的欧拉函数值。欧拉函数φ(n)表示小于或等于n的正整数中
/// 与n互质的数的个数。
///
/// # 参数
/// * `n` - 需要计算欧拉函数值的上界（包含）
///
/// # 返回值
/// 返回一个Vec<i32>，其中第i个元素（索引从0开始）表示数字(i+1)的欧拉函数值
///
/// # 算法说明
/// 使用类似埃拉托斯特尼筛法的方法来高效计算多个欧拉函数值
#[allow(unused)]
pub fn compute_totient(n: i32) -> Vec<i32> {
    let mut phi: Vec<i32> = Vec::new();

    // 初始化phi数组，phi[i] = i
    (0..=n).for_each(|idx| {
        phi.push(idx)
    });

    // 使用筛法计算欧拉函数值
    // 对于每个质数p，更新其所有倍数的欧拉函数值
    (2..=n).for_each(|p| {
        if phi[(p) as usize] == p {
            // p是质数，φ(p) = p - 1
            phi[(p) as usize] = p - 1;

            // 更新p的所有倍数k的欧拉函数值
            // φ(k) = φ(k) * (1 - 1/p) = φ(k) / p * (p - 1)
            for k in (2 * p..=n).step_by(p as usize) {
                phi[(k) as usize] = (phi[k as usize] / p) * (p - 1);
            }
        }
    });

    // 返回从索引1开始的结果（因为φ(0)无意义）
    phi[1..].to_vec()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(
            compute_totient(12),
            vec![1, 1, 2, 2, 4, 2, 6, 4, 6, 4, 10, 4]
        );
    }

    #[test]
    fn test_2() {
        assert_eq!(compute_totient(7), vec![1, 1, 2, 2, 4, 2, 6]);
    }

    #[test]
    fn test_3() {
        assert_eq!(compute_totient(4), vec![1, 1, 2, 2]);
    }
}
