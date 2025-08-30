#[allow(unused)]
pub fn ceil(x: f64) -> f64 {
    // 将x向零取整，即截断小数部分
    let x_rounded_towards_zero = x as i32 as f64;

    // 如果x是负数或者x已经是整数，则直接返回截断后的值
    // 否则返回截断值加1，实现向上取整的功能
    if x < 0. || x_rounded_towards_zero == x {
        x_rounded_towards_zero
    } else {
        x_rounded_towards_zero + 1_f64
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn positive_decimal() {
        let num = 1.10;
        assert_eq!(ceil(num), num.ceil());
    }

    #[test]
    fn positive_decimal_with_small_number() {
        let num = 3.01;
        assert_eq!(ceil(num), num.ceil());
    }

    #[test]
    fn positive_integer() {
        let num = 1.00;
        assert_eq!(ceil(num), num.ceil());
    }

    #[test]
    fn negative_decimal() {
        let num = -1.10;
        assert_eq!(ceil(num), num.ceil());
    }

    #[test]
    fn negative_decimal_with_small_number() {
        let num = -1.01;
        assert_eq!(ceil(num), num.ceil());
    }

    #[test]
    fn negative_integer() {
        let num = -1.00;
        assert_eq!(ceil(num), num.ceil());
    }

    #[test]
    fn zero() {
        let num = 0.00;
        assert_eq!(ceil(num), num.ceil());
    }
}
