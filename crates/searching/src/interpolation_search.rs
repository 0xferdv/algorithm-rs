/// 插值查找算法实现
///
/// 在一个已排序的整数切片中查找指定元素，使用插值查找算法。
/// 插值查找是一种改进的二分查找，它根据目标值在数据分布中的位置来预测其可能的位置，
/// 从而减少比较次数，提高查找效率。
///
/// # 参数
/// * `nums` - 一个已排序的整数切片，用于查找目标元素
/// * `item` - 要查找的目标整数
///
/// # 返回值
/// * `Ok(index)` - 如果找到目标元素，返回其在切片中的索引位置
/// * `Err(0)` - 如果未找到目标元素或输入切片为空，返回错误值0
#[allow(unused)]
pub fn interpolation_search<Ordering>(nums: &[i32], item: &i32) -> Result<usize, usize> {
    // 处理空数组的情况
    if nums.is_empty() {
        return Err(0);
    }

    let mut low: usize = 0;
    let mut high: usize = nums.len() - 1;

    // 插值查找主循环
    while low < high {
        // 如果目标值超出当前搜索范围，则直接退出
        if *item < nums[low] || *item > nums[high] {
            break;
        }

        // 计算插值位置偏移量
        let offest: usize = low + (((high - low) / (nums[high] - nums[low]) as usize) * (*item - nums[low]) as usize);

        // 比较目标值与插值位置的值
        match nums[offest].cmp(item) {
            std::cmp::Ordering::Equal => return Ok(offest),
            std::cmp::Ordering::Greater => high = offest - 1,
            std::cmp::Ordering::Less => low = offest + 1,
        }
    }

    // 未找到目标元素
    Err(0)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cmp::Ordering;

    #[test]
    fn returns_err_if_empty_slice() {
        let nums = [];
        assert_eq!(interpolation_search::<Ordering>(&nums, &3), Err(0));
    }

    #[test]
    fn returns_err_if_target_not_found() {
        let nums = [1, 2, 3, 4, 5, 6];
        assert_eq!(interpolation_search::<Ordering>(&nums, &10), Err(0));
    }

    #[test]
    fn returns_first_index() {
        let index: Result<usize, usize> = interpolation_search::<Ordering>(&[1, 2, 3, 4, 5], &1);
        assert_eq!(index, Ok(0));
    }

    #[test]
    fn returns_last_index() {
        let index: Result<usize, usize> = interpolation_search::<Ordering>(&[1, 2, 3, 4, 5], &5);
        assert_eq!(index, Ok(4));
    }

    #[test]
    fn returns_middle_index() {
        let index: Result<usize, usize> = interpolation_search::<Ordering>(&[1, 2, 3, 4, 5], &3);
        assert_eq!(index, Ok(2));
    }
}
