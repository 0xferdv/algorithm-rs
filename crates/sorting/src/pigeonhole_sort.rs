/**
 * 鸽巢排序（Pigeonhole Sort）函数
 *
 * 该函数使用鸽巢排序算法对整数数组进行原地排序。鸽巢排序是一种适用于整数且数值范围较小的排序算法，
 * 其基本思想是为每个可能的数值创建一个"鸽巢"，统计每个数值出现的次数，然后按顺序将数值放回原数组。
 *
 * 参数:
 * arr: &mut [i32] - 待排序的整数数组的可变引用
 *
 * 返回值:
 * 无返回值，直接修改输入数组
 */
#[allow(unused)]
pub fn pigeonhole_sort<>(arr: &mut [i32]) {
    // 如果数组非空，则计算最小值和最大值
    if let (Some(min), Some(max)) = (arr.iter().min(), arr.iter().max()) {
        // 计算鸽巢的数量，即数值范围
        let holes_range: usize = (max - min + 1) as usize;
        // 创建两个向量：holes用于存储实际数值，holes_repeat用于记录每个数值出现的次数
        let mut holes = vec![0; holes_range];
        let mut holes_repeat = vec![0; holes_range];

        // 遍历原数组，填充鸽巢信息
        arr.iter().for_each(|item| {
            let cur_idx = *item - min;
            holes[cur_idx as usize] = *item;
            holes_repeat[cur_idx as usize] += 1;
        });

        // 根据鸽巢信息重新构建排序后的数组
        let mut idx = 0;
        (0..holes_range).for_each(|hole_idx| {
            // 将每个数值按照其出现次数依次放回原数组
            while holes_repeat[hole_idx] > 0 {
                arr[idx] = holes[hole_idx];
                holes_repeat[hole_idx] -= 1;
                idx += 1;
            }
        });
    }
}

#[cfg(test)]
mod tests {
    use super::super::is_sorted;
    use super::*;

    #[test]
    fn test1() {
        let mut arr1 = [3, 3, 3, 1, 2, 6, 5, 5, 5, 4, 1, 6, 3];
        pigeonhole_sort(&mut arr1);
        assert!(is_sorted(&arr1));
        let mut arr2 = [6, 5, 4, 3, 2, 1];
        pigeonhole_sort(&mut arr2);
        assert!(is_sorted(&arr2));
    }
}
