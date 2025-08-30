/// 生成给定正整数的考拉兹序列（Collatz sequence）
/// 
/// 考拉兹猜想是一个著名的未解决问题，它指出对于任何正整数n，
/// 通过以下规则变换最终都会到达1：
/// - 如果n是偶数，则n变为n/2
/// - 如果n是奇数，则n变为3*n+1
/// 
/// # 参数
/// * `n` - 起始的正整数，必须大于0
/// 
/// # 返回值
/// * `Some(Vec<usize>)` - 包含完整考拉兹序列的向量，从n开始到1结束
/// * `None` - 当输入为0时返回None，因为0不是有效的起始值
/// 
/// # 示例
///
/// let sequence = collatz_sequence(10);
/// assert_eq!(sequence.unwrap(), [10, 5, 16, 8, 4, 2, 1]);
///
#[allow(unused)]
pub fn collatz_sequence(mut n: usize) -> Option<Vec<usize>> {
    // 检查输入有效性，0不是考拉兹序列的有效起始值
    if n == 0 {
        return None;
    }

    // 初始化序列存储向量
    let mut list: Vec<usize> = Vec::new();

    // 执行考拉兹变换直到达到1
    while n != 1 {
        list.push(n);
        // 根据当前数字的奇偶性应用相应的变换规则
        if n % 2 == 0 {
            n /= 2;
        } else {
            n = 3 * n + 1;
        }
    }

    // 将最终的1添加到序列中
    list.push(1);
    Some(list)
}


#[cfg(test)]
mod tests {
    use super::collatz_sequence;

    #[test]
    fn validity_check() {
        assert_eq!(collatz_sequence(10).unwrap(), [10, 5, 16, 8, 4, 2, 1]);
        assert_eq!(
            collatz_sequence(15).unwrap(),
            [15, 46, 23, 70, 35, 106, 53, 160, 80, 40, 20, 10, 5, 16, 8, 4, 2, 1]
        );
        assert_eq!(collatz_sequence(0).unwrap_or_else(|| vec![0]), [0]);
    }
}
