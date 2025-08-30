#[allow(unused)]
pub fn combinations(n: i64, k: i64) -> i64 {
    if n < 0 || k < 0 {
        panic!("please insert positive integers.")
    }
    let mut res: i64 = 1;
    // 使用迭代方式计算组合数 C(n,k) = n!/(k!(n-k)!)
    // 通过逐步计算 n*(n-1)*...*(n-k+1) / (1*2*...*k) 来避免大数溢出
    (0..k).for_each(|idx| {
        res *= n - idx;
        res /= idx + 1;
    });
    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_combinations_10_choose_5() {
        assert_eq!(combinations(10, 5), 252);
    }

    #[test]
    fn test_combinations_6_choose_3() {
        assert_eq!(combinations(6, 3), 20);
    }

    #[test]
    fn test_combinations_20_choose_5() {
        assert_eq!(combinations(20, 5), 15504);
    }

    // #[test]
    // #[should_panic(expected = "Please insert positive values")]
    // fn test_combinations_invalid_input() {
    //     combinations(-5, 10);
}
