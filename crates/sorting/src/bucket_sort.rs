use crate::insertion_sort::insertion_sort;

/// 对给定的 usize 数组执行桶排序（Bucket Sort）算法。
///
/// 该函数将输入数组中的元素分配到若干个桶中，每个桶内部使用插入排序进行排序，
/// 最后将所有桶中的元素按顺序合并成一个有序数组。
///
/// # 参数
///
/// * `arr` - 一个待排序的 usize 类型切片。
///
/// # 返回值
///
/// 返回一个新的 Vec<usize>，其中包含按升序排列的元素。
#[allow(unused)]
pub fn bucket_sort(arr: &[usize]) -> Vec<usize> {
    // 处理空数组的情况
    if arr.is_empty() {
        return vec![];
    }

    // 找出数组中的最大值，用于确定桶的映射范围
    let max = *arr.iter().max().unwrap();
    let len = arr.len();

    // 创建 len + 1 个空桶
    let mut buckets = vec![vec![]; len + 1];

    // 将数组中的每个元素根据其值映射到对应的桶中
    for x in arr {
        buckets[len * *x / max].push(*x);
    }

    // 对每个非空桶使用插入排序进行排序
    for bucket in buckets.iter_mut() {
        insertion_sort(bucket)
    }

    // 合并所有桶中的元素，生成最终结果
    let mut result = vec![];
    for bucket in buckets {
        for x in bucket {
            result.push(x)
        }
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{have_same_elements, is_sorted};

    #[test]
    fn empty() {
        let arr: [usize; 0] = [];
        let cloned = arr;
        let res = bucket_sort(&arr);
        assert!(is_sorted(&res) && have_same_elements(&res, &cloned));
    }

    #[test]
    fn one_element() {
        let arr: [usize; 1] = [4];
        let cloned = arr;
        let res = bucket_sort(&arr);
        assert!(is_sorted(&res) && have_same_elements(&res, &cloned));
    }

    #[test]
    fn already_sorted() {
        let arr: [usize; 3] = [10, 19, 105];
        let cloned = arr;
        let res = bucket_sort(&arr);
        assert!(is_sorted(&res) && have_same_elements(&res, &cloned));
    }

    #[test]
    fn basic() {
        let arr: [usize; 4] = [35, 53, 1, 0];
        let cloned = arr;
        let res = bucket_sort(&arr);
        assert!(is_sorted(&res) && have_same_elements(&res, &cloned));
    }

    #[test]
    fn odd_number_of_elements() {
        let arr: [usize; 5] = [1, 21, 5, 11, 58];
        let cloned = arr;
        let res = bucket_sort(&arr);
        assert!(is_sorted(&res) && have_same_elements(&res, &cloned));
    }

    #[test]
    fn repeated_elements() {
        let arr: [usize; 4] = [542, 542, 542, 542];
        let cloned = arr;
        let res = bucket_sort(&arr);
        assert!(is_sorted(&res) && have_same_elements(&res, &cloned));
    }
}
