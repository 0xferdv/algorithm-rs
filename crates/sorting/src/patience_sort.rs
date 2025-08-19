/// 使用 Patience Sort 算法对给定的可变切片进行原地排序。
///
/// Patience Sort 是一种基于模拟纸牌游戏“Patience”（也称 Solitaire）的排序算法。
/// 它通过将元素依次放到不同的“堆”中，然后按顺序从这些堆中取出最小元素来完成排序。
///
/// # 参数
///
/// * `arr` - 一个可变引用的泛型切片，其中元素类型 `T` 必须实现 `Ord` 和 `Copy` trait。
///
/// # 行为
///
/// 该函数会对输入的数组进行原地排序，不返回任何值。
#[allow(unused)]
pub fn patience_sort<T: Ord + Copy>(arr: &mut [T]) {
    // 如果数组为空，直接返回
    if arr.is_empty() {
        return;
    }

    // 存储多个递增子序列的“堆”
    let mut piles: Vec<Vec<T>> = Vec::new();

    // 遍历数组中的每个元素，并将其放入合适的堆中
    for &item in arr.iter() {
        // 使用二分查找找到第一个堆顶元素大于等于当前元素的堆
        let mut left = 0usize;
        let mut right = piles.len();
        while left < right {
            let mid = left + (right - left) / 2;
            if piles[mid][piles[mid].len() - 1] >= item {
                right = mid;
            } else {
                left = mid + 1;
            }
        }

        // 如果没有找到合适的堆，则新建一个堆
        if left == piles.len() {
            piles.push(vec![item]);
        } else {
            // 否则将当前元素加入找到的堆中
            piles[left].push(item);
        }
    }

    // 按照堆顶元素的大小顺序依次取出最小元素，重构排序后的数组
    let mut idx = 0usize;
    while let Some((min_id, pile)) = piles
        .iter()
        .enumerate()
        .min_by_key(|(_, pile)| *pile.last().unwrap())
    {
        // 将当前最小元素写入原数组
        arr[idx] = *pile.last().unwrap();
        idx += 1;

        // 从对应的堆中移除该元素
        piles[min_id].pop();

        // 如果该堆变空，则将其从堆列表中删除
        if piles[min_id].is_empty() {
            piles.remove(min_id);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{have_same_elements, is_sorted};

    #[test]
    fn basic() {
        let mut array = vec![
            -2, 7, 15, -14, 0, 15, 0, 10_033, 7, -7, -4, -13, 5, 8, -14, 12,
        ];
        let cloned = array.clone();
        patience_sort(&mut array);
        assert!(is_sorted(&array) && have_same_elements(&array, &cloned));
    }

    #[test]
    fn empty() {
        let mut array = Vec::<i32>::new();
        let cloned = array.clone();
        patience_sort(&mut array);
        assert!(is_sorted(&array) && have_same_elements(&array, &cloned));
    }

    #[test]
    fn one_element() {
        let mut array = vec![3];
        let cloned = array.clone();
        patience_sort(&mut array);
        assert!(is_sorted(&array) && have_same_elements(&array, &cloned));
    }

    #[test]
    fn pre_sorted() {
        let mut array = vec![-123_456, 0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
        let cloned = array.clone();
        patience_sort(&mut array);
        assert!(is_sorted(&array) && have_same_elements(&array, &cloned));
    }
}
