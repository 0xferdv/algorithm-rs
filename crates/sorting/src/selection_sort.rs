/// 选择排序算法实现
///
/// 该函数使用选择排序算法对切片进行原地排序。选择排序的基本思想是：
/// 每次从未排序的部分中找到最小元素，将其放到已排序部分的末尾。
///
/// # 参数
///
/// * `arr` - 需要排序的可变切片引用，元素类型必须实现Ord trait
///
/// # 返回值
///
/// 无返回值，直接修改输入切片
///
/// # 算法复杂度
///
/// * 时间复杂度：O(n²)
/// * 空间复杂度：O(1)
#[allow(unused)]
pub fn selection_sort<T: Ord>(arr: &mut [T]) {
    let len = arr.len();
    // 遍历数组的每个位置，作为当前要放置最小元素的位置
    (0..len).for_each(|left| {
        let mut smallest = left;
        // 在剩余未排序部分中寻找最小元素的索引
        (left + 1..len).for_each(|right| {
            if arr[right] < arr[smallest] {
                smallest = right;
            }
        });
        // 将找到的最小元素与当前位置交换
        arr.swap(smallest, left);
    });
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{have_same_elements, is_sorted};

    #[test]
    fn basic() {
        let mut res = vec!["d", "a", "c", "b"];
        let cloned = res.clone();
        selection_sort(&mut res);
        assert!(is_sorted(&res) && have_same_elements(&res, &cloned));
    }

    #[test]
    fn empty() {
        let mut res = Vec::<u8>::new();
        let cloned = res.clone();
        selection_sort(&mut res);
        assert!(is_sorted(&res) && have_same_elements(&res, &cloned));
    }

    #[test]
    fn one_element() {
        let mut res = vec!["a"];
        let cloned = res.clone();
        selection_sort(&mut res);
        assert!(is_sorted(&res) && have_same_elements(&res, &cloned));
    }

    #[test]
    fn pre_sorted() {
        let mut res = vec!["a", "b", "c"];
        let cloned = res.clone();
        selection_sort(&mut res);
        assert!(is_sorted(&res) && have_same_elements(&res, &cloned));
    }
}
