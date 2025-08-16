/// 地精排序（Gnome Sort）实现
///
/// 这是一个简单的排序算法，其工作方式类似于插入排序，但移动元素的方式类似于冒泡排序。
/// 算法通过比较相邻元素并交换它们来逐步将较小的元素向前移动，直到整个数组有序。
///
/// # 参数
/// * `arr` - 需要排序的切片，其中元素类型必须实现 PartialOrd、PartialEq 和 Clone trait
///
/// # 返回值
/// 返回一个新的已排序向量，包含原数组的所有元素
///
/// # 算法复杂度
/// 时间复杂度：最坏情况 O(n²)，最好情况 O(n)
/// 空间复杂度：O(n)（由于需要克隆输入数组）
#[allow(unused)]
pub fn gnome_sort<T: PartialOrd + PartialEq + Clone>(arr: &[T]) -> Vec<T> {
    let mut i: usize = 1;
    let mut j: usize = 2;

    let len = arr.len();
    let mut arr = arr.to_vec();

    // 主排序循环：使用两个指针 i 和 j 来遍历数组
    // i 指向当前比较的元素，j 用于记录下一个要检查的位置
    while i < len {
        // 如果前一个元素小于当前元素，说明这部分已经有序
        // 将 i 移动到 j 的位置，j 向前移动一位
        if arr[i - 1] < arr[i] {
            i = j;
            j = i + 1;
        } else {
            // 如果前一个元素大于当前元素，则交换它们
            arr.swap(i - 1, i);
            i -= 1;

            // 如果回到数组开始位置，则重置 i 和 j 继续排序
            if i == 0 {
                i = j;
                j += 1;
            }
        }
    }
    arr
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{have_same_elements, is_sorted};

    #[test]
    fn basic() {
        let original = [6, 5, -8, 3, 2, 3];
        let res = gnome_sort(&original);
        assert!(is_sorted(&res) && have_same_elements(&res, &original));
    }

    #[test]
    fn already_sorted() {
        let original = gnome_sort(&["a", "b", "c"]);
        let res = gnome_sort(&original);
        assert!(is_sorted(&res) && have_same_elements(&res, &original));
    }

    #[test]
    fn odd_number_of_elements() {
        let original = gnome_sort(&["d", "a", "c", "e", "b"]);
        let res = gnome_sort(&original);
        assert!(is_sorted(&res) && have_same_elements(&res, &original));
    }

    #[test]
    fn one_element() {
        let original = gnome_sort(&[3]);
        let res = gnome_sort(&original);
        assert!(is_sorted(&res) && have_same_elements(&res, &original));
    }

    #[test]
    fn empty() {
        let original = gnome_sort(&Vec::<u8>::new());
        let res = gnome_sort(&original);
        assert!(is_sorted(&res) && have_same_elements(&res, &original));
    }
}
