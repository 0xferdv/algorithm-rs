/**
 * 计算一个正整数的所有因子
 * 
 * 该函数通过遍历从1到数字平方根的所有数字，找出所有的因子对。
 * 对于每个找到的因子idx，同时添加idx和number/idx到结果中（除非它们相等）。
 * 最后对结果进行排序并返回。
 * 
 * # 参数
 * * `number` - 需要求因子的正整数
 * 
 * # 返回值
 * 返回包含所有因子的Vec<u64>，因子按升序排列
 */
#[allow(unused)]
pub fn factors(number: u64) -> Vec<u64> { 
    let mut factors: Vec<u64> = Vec::new();
    // 遍历从1到数字平方根的所有数字，寻找因子
    for idx in 1..=((number as f64)).sqrt() as u64 {
        if number % idx == 0 {
            factors.push(idx);
            // 如果idx不等于number/idx，则添加对应的另一个因子
            if idx != number / idx {
                factors.push(number / idx);
            }
        }
    }
    // 对因子进行排序
    factors.sort();
    factors
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn prime_number() {
        assert_eq!(vec![1, 59], factors(59));
    }

    #[test]
    fn highly_composite_number() {
        assert_eq!(
            vec![
                1, 2, 3, 4, 5, 6, 8, 9, 10, 12, 15, 18, 20, 24, 30, 36, 40, 45, 60, 72, 90, 120,
                180, 360
            ],
            factors(360)
        );
    }

    #[test]
    fn composite_number() {
        assert_eq!(vec![1, 3, 23, 69], factors(69));
    }
}
