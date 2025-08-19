/// 基数排序函数，对给定的 u64 数组进行原地排序。
///
/// # 参数
///
/// * `arr` - 需要排序的 u64 数组的可变引用
///
/// # 返回值
///
/// 无返回值，直接修改输入数组完成排序
#[allow(unused)]
pub fn radix_sort(arr: &mut [u64]) {
    // 获取数组中的最大值，如果数组为空则直接返回
    let max: usize = match arr.iter().max() {
        Some(&x) => x as usize,
        None => return,
    };

    // 设置基数为数组长度的下一个2的幂次，用于确定每轮排序的桶数量
    let radix = arr.len().next_power_of_two();

    // 从最低位开始，逐位进行计数排序
    let mut place = 1;
    while place <= max {
        // 定义获取当前位数字的闭包
        let digit = |x| x as usize / place % radix;

        // 初始化计数数组，用于统计每个数字出现的次数
        let mut count = vec![0; radix];

        // 统计当前位上每个数字的出现次数
        arr.iter().for_each(|&item| {
            count[digit(item)] += 1;
        });

        // 将计数数组转换为累积计数数组，表示每个数字在排序后数组中的位置
        (1..radix).for_each(|idx| {
            count[idx] += count[idx - 1];
        });

        // 从后向前遍历原数组，根据计数数组确定每个元素的最终位置
        arr.to_owned().iter().rev().for_each(|&item| {
            count[digit(item)] -= 1;
            arr[count[digit(item)]] = item;
        });

        // 移动到下一位
        place *= radix;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{have_same_elements, is_sorted};

    #[test]
    fn empty() {
        let mut a: [u64; 0] = [];
        let cloned = a;
        radix_sort(&mut a);
        assert!(is_sorted(&a) && have_same_elements(&a, &cloned));
    }

    #[test]
    fn descending() {
        let mut v = vec![201, 127, 64, 37, 24, 4, 1];
        let cloned = v.clone();
        radix_sort(&mut v);
        assert!(is_sorted(&v) && have_same_elements(&v, &cloned));
    }

    #[test]
    fn ascending() {
        let mut v = vec![1, 4, 24, 37, 64, 127, 201];
        let cloned = v.clone();
        radix_sort(&mut v);
        assert!(is_sorted(&v) && have_same_elements(&v, &cloned));
    }
}

