use std::cmp::min;

/// 跳跃搜索算法实现
///
/// 在一个已排序的数组中查找指定元素，使用跳跃搜索算法。
/// 算法首先以固定步长跳跃遍历数组，找到目标可能存在的区间，
/// 然后在该区间内进行线性搜索。
///
/// # 参数
/// * `item`: 要查找的目标元素的引用
/// * `arr`: 已排序的元素切片
///
/// # 返回值
/// 如果找到目标元素，返回其索引；否则返回 None
#[allow(unused)]
pub fn jump_search<T: Ord>(item: &T, arr: &[T]) -> Option<usize> {
    // 处理空数组的情况
    if arr.is_empty() {
        return None;
    }

    let len = arr.len();
    let mut step = (len as f64).sqrt() as usize;
    let mut prev = 0;

    // 跳跃搜索阶段：以步长为单位跳跃，找到目标可能所在的区间
    while &arr[min(len, step) - 1] < item {
        prev = step;
        step += (len as f64).sqrt() as usize;
        if prev >= len {
            return None;
        }
    }

    // 线性搜索阶段：在确定的区间内逐个比较元素
    while &arr[prev] < item {
        prev += 1;
    }

    // 检查是否找到目标元素
    if &arr[prev] == item {
        return Some(prev);
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty() {
        assert!(jump_search(&"a", &[]).is_none());
    }

    #[test]
    fn one_item() {
        assert_eq!(jump_search(&"a", &["a"]).unwrap(), 0);
    }

    #[test]
    fn search_strings() {
        assert_eq!(
            jump_search(&"a", &["a", "b", "c", "d", "google", "zoo"]).unwrap(),
            0
        );
    }

    #[test]
    fn search_ints() {
        let arr = [1, 2, 3, 4];
        assert_eq!(jump_search(&4, &arr).unwrap(), 3);
        assert_eq!(jump_search(&3, &arr).unwrap(), 2);
        assert_eq!(jump_search(&2, &arr).unwrap(), 1);
        assert_eq!(jump_search(&1, &arr).unwrap(), 0);
    }

    #[test]
    fn not_found() {
        let arr = [1, 2, 3, 4];

        assert!(jump_search(&5, &arr).is_none());
        assert!(jump_search(&0, &arr).is_none());
    }
}
