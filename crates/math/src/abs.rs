/// 计算数值的绝对值
///
/// 该函数通过泛型实现，可以处理任何实现了相应数值特质的类型。
/// 对于负数返回其相反数，对于非负数则直接返回原值。
///
/// # 参数
/// * `num` - 需要计算绝对值的数值
///
/// # 返回值
/// 返回输入数值的绝对值，类型与输入相同
///
/// # 泛型约束
/// * `T` 必须实现 `std::ops::Neg` 特质，以便支持取负操作
/// * `T` 必须实现 `PartialOrd` 特质，以便支持比较操作
/// * `T` 必须实现 `Copy` 特质，以便支持复制操作
/// * `T` 必须实现 `num_traits::Zero` 特质，以便获取零值
#[allow(unused)]
pub fn abs<T>(num: T) -> T
where
    T: std::ops::Neg<Output = T> + PartialOrd + Copy + num_traits::Zero,
{
    // 判断数值是否小于零，如果是则返回其相反数，否则返回原值
    if num < T::zero() {
        -num
    } else {
        num
    }
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_negative_number_i32() {
        assert_eq!(69, abs(-69));
    }

    #[test]
    fn test_negative_number_f64() {
        assert_eq!(69.69, abs(-69.69));
    }

    #[test]
    fn zero() {
        assert_eq!(0.0, abs(0.0));
    }

    #[test]
    fn positive_number() {
        assert_eq!(69.69, abs(69.69));
    }
}
