/// 使用煎饼排序算法对数组进行排序。
///
/// 煎饼排序是一种变换排序算法，其思想是每次找到未排序部分的最大元素，
/// 通过两次翻转操作将其放到正确的位置上。
///
/// # 参数
///
/// * `arr` - 需要排序的可变切片引用，元素类型必须实现 PartialEq、PartialOrd、Clone 和 Ord trait
///
/// # 返回值
///
/// 返回排序后元素的 `Vec<T>` 容器
#[allow(unused)]
pub fn pancake_sort<T: PartialEq + PartialOrd + Clone + Ord>(arr: &mut [T]) -> Vec<T> {
    let len = arr.len();
    if len <= 1 {
        arr.to_vec();
    }

    // 从数组末尾开始向前遍历，逐步确定每个位置的最终元素
    for idx in (0..len).rev() {
        // 在未排序部分（0 到 idx）中找到最大元素的索引
        let max_idx = arr
            .iter()
            .take(idx + 1)
            .enumerate()
            .max_by_key(|&(_, elem)| elem)
            .map(|(idx, _)| idx)
            .unwrap();

        // 如果最大元素不在当前考虑的末尾位置，则需要进行翻转操作
        if max_idx != idx {
            // 第一次翻转：将最大元素翻转到数组开头
            arr[0..=max_idx].reverse();
            // 第二次翻转：将最大元素从开头翻转到其最终位置（idx）
            arr[0..=idx].reverse();
        }
    }
    arr.to_vec()
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        let res = pancake_sort(&mut [6, 5, -8, 3, 2, 3]);
        assert_eq!(res, vec![-8, 2, 3, 3, 5, 6]);
    }

    #[test]
    fn already_sorted() {
        let res = pancake_sort(&mut ["a", "b", "c"]);
        assert_eq!(res, vec!["a", "b", "c"]);
    }

    #[test]
    fn odd_number_of_elements() {
        let res = pancake_sort(&mut ["d", "a", "c", "e", "b"]);
        assert_eq!(res, vec!["a", "b", "c", "d", "e"]);
    }

    #[test]
    fn one_element() {
        let res = pancake_sort(&mut [3]);
        assert_eq!(res, vec![3]);
    }

    #[test]
    fn empty() {
        let res = pancake_sort(&mut [] as &mut [u8]);
        assert_eq!(res, vec![]);
    }
}
