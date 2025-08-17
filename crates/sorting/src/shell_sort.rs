/// 希尔排序（Shell Sort）实现。
///
/// 希尔排序是一种基于插入排序的不稳定排序算法，通过将原始数组分割成多个子序列来进行排序，
/// 每个子序列使用插入排序进行排序。随着步长逐渐减小，最终整个数组变为基本有序，再进行一次插入排序即可完成排序。
///
/// # 参数
///
/// * `arr` - 需要排序的可变切片，元素类型必须实现 `Ord` 和 `Copy` trait。
///
/// # 返回值
///
/// 无返回值。原地修改输入数组。
#[allow(unused)]
pub fn shell_sort<T: Ord + Copy>(arr: &mut [T]) {

    /// 插入排序的辅助函数，在给定起始位置和间隔的情况下对子序列进行插入排序。
    ///
    /// # 参数
    ///
    /// * `arr` - 要排序的数组切片。
    /// * `start` - 子序列的起始索引。
    /// * `gap` - 当前子序列中元素之间的间隔（步长）。
    fn insertion<T: Ord + Copy>(arr: &mut [T], start: usize, gap: usize) {
        let len = arr.len();
        // 对每个子序列中的元素执行插入排序逻辑
        ((start + gap)..len).step_by(gap).for_each(|idx | {
            let curr_val = arr[idx];
            let mut pos = idx;
            // 将当前元素向前移动到合适的位置
            while pos >= gap && arr[pos - gap] > curr_val {
                arr[pos] = arr[pos - gap];
                pos -= gap;
            }
            arr[pos] = curr_val;
        });
    }

    // 初始步长为数组长度的一半
    let mut count_sublist = arr.len() / 2;
    // 不断缩小步长，直到为0
    while count_sublist > 0 {
        // 对每个起始点执行插入排序
        (0..count_sublist).for_each(|pos_start| {
            insertion(arr, pos_start, count_sublist);
        });
        // 步长每次减半
        count_sublist /= 2;
    }
}


#[cfg(test)]
mod test {
    use super::*;
    use crate::{have_same_elements, is_sorted};

    #[test]
    fn basic() {
        let mut vec = vec![3, 5, 6, 3, 1, 4];
        let cloned = vec.clone();
        shell_sort(&mut vec);
        assert!(is_sorted(&vec) && have_same_elements(&vec, &cloned));
    }

    #[test]
    fn empty() {
        let mut vec: Vec<i32> = vec![];
        let cloned = vec.clone();
        shell_sort(&mut vec);
        assert!(is_sorted(&vec) && have_same_elements(&vec, &cloned));
    }

    #[test]
    fn reverse() {
        let mut vec = vec![6, 5, 4, 3, 2, 1];
        let cloned = vec.clone();
        shell_sort(&mut vec);
        assert!(is_sorted(&vec) && have_same_elements(&vec, &cloned));
    }

    #[test]
    fn already_sorted() {
        let mut vec = vec![1, 2, 3, 4, 5, 6];
        let cloned = vec.clone();
        shell_sort(&mut vec);
        assert!(is_sorted(&vec) && have_same_elements(&vec, &cloned));
    }
}
