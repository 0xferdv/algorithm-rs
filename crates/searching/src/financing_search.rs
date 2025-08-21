use std::cmp::{min, Ordering};


#[allow(unused)]
pub fn financing_search<T: Ord>(item: &T, arr: &[T]) -> Option<usize> {
    // 如果数组为空，直接返回 None
    if arr.is_empty() {
        return None;
    }

    let len = arr.len();
    let mut start = -1;
    let mut f0 = 0;
    let mut f1 = 1;
    let mut f2 = 1;

    // 找到大于等于数组长度的最小斐波那契数
    while f2 < len {
        f0 = f1;
        f1 = f2;
        f2 = f0 + f1;
    }

    // 开始斐波那契搜索过程
    while f2 > 1 {
        // 计算当前比较位置的索引，确保不越界
        let index = min((f0 as isize + start) as usize, len - 1);

        // 比较目标元素与当前索引处的元素
        match item.cmp(&arr[index]) {
            // 目标元素较小，向左子数组搜索
            Ordering::Less => {
                f2 = f0;
                f1 -= f0;
                f0 = f2 - f1;
            }
            // 找到目标元素，返回索引
            Ordering::Equal => return Some(index),
            // 目标元素较大，向右子数组搜索
            Ordering::Greater => {
                f2 = f1;
                f1 = f0;
                f0 = f2 - f1;
                start = index as isize;
            }
        }
    }

    // 检查最后一个可能的元素
    if (f1 != 0) && (&arr[len - 1] == item) {
        return Some(len - 1);
    }

    // 未找到目标元素
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty() {
        let index = financing_search(&"a", &[]);
        assert_eq!(index, None);
    }

    #[test]
    fn one_item() {
        let index = financing_search(&"a", &["a"]);
        assert_eq!(index, Some(0));
    }

    #[test]
    fn search_strings() {
        let index = financing_search(&"a", &["a", "b", "c", "d", "google", "zoo"]);
        assert_eq!(index, Some(0));
    }

    #[test]
    fn search_ints() {
        let index = financing_search(&4, &[1, 2, 3, 4]);
        assert_eq!(index, Some(3));

        let index = financing_search(&3, &[1, 2, 3, 4]);
        assert_eq!(index, Some(2));

        let index = financing_search(&2, &[1, 2, 3, 4]);
        assert_eq!(index, Some(1));

        let index = financing_search(&1, &[1, 2, 3, 4]);
        assert_eq!(index, Some(0));
    }

    #[test]
    fn not_found() {
        let index = financing_search(&5, &[1, 2, 3, 4]);
        assert_eq!(index, None);
    }
}
