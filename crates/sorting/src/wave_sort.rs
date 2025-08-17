/// 对给定的数组进行波浪排序（Wave Sort）。
///
/// 波浪排序的结果是数组元素呈现波浪形状，即：
/// arr[0] <= arr[1] >= arr[2] <= arr[3] >= arr[4] ...
///
/// 该函数首先对数组进行排序，然后交换相邻元素以形成波浪形状。
///
/// # 参数
///
/// * `arr` - 一个可变引用的切片，包含实现了 `Ord` 和 `Copy` trait 的元素。
///
/// # 说明
///
/// 该函数会修改传入的数组内容。
#[allow(unused)]
pub fn wave_sort<T: Ord + Copy>(arr: &mut [T]) {
    let n = arr.len();
    // 如果数组长度小于等于1，无需排序直接返回
    if n <= 1 {
        return;
    }
    // 首先对数组进行排序
    arr.sort();
    // 交换相邻元素，步长为2，构造波浪形状
    (0..n - 1).step_by(2).for_each(|idx| {arr.swap(idx, idx + 1) });
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case_1() {
        let mut array = vec![10, 90, 49, 2, 1, 5, 23];
        wave_sort(&mut array);
        let expected = vec![2, 1, 10, 5, 49, 23, 90];
        assert_eq!(&array, &expected);
    }

    #[test]
    fn test_case_2() {
        let mut array = vec![1, 3, 4, 2, 7, 8];
        wave_sort(&mut array);
        let expected = vec![2, 1, 4, 3, 8, 7];
        assert_eq!(&array, &expected);
    }

    #[test]
    fn test_case_3() {
        let mut array = vec![3, 3, 3, 3];
        wave_sort(&mut array);
        let expected = vec![3, 3, 3, 3];
        assert_eq!(&array, &expected);
    }

    #[test]
    fn test_case_4() {
        let mut array = vec![9, 4, 6, 8, 14, 3];
        wave_sort(&mut array);
        let expected = vec![4, 3, 8, 6, 14, 9];
        assert_eq!(&array, &expected);
    }

    #[test]
    fn test_case_5() {
        let mut array = vec![5, 10, 15, 20, 25];
        wave_sort(&mut array);
        let expected = vec![10, 5, 20, 15, 25];
        assert_eq!(&array, &expected);
    }
}
