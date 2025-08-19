/// 二分查找函数，用于在已排序的数组中查找插入位置。
///
/// 此函数使用二分查找算法，在给定的已排序数组中查找第一个大于目标值的元素位置。
/// 该位置即为目标值应插入的位置，以保持数组的有序性。
///
/// # 参数
/// * `arr` - 一个已排序的切片，用于查找插入位置
/// * `target` - 要查找插入位置的目标值
///
/// # 返回值
/// 返回目标值应该插入的位置索引
fn _binary_search<T: Ord>(arr: &[T], target: &T) -> usize {
    let mut low = 0;
    let mut high = arr.len();

    // 二分查找循环，不断缩小查找范围直到找到插入位置
    while low < high {
        let mid = low + (high - low) / 2;
        if arr[mid] <= *target {
            low = mid + 1;
        } else {
            high = mid;
        }
    }
    low
}

/// 二分插入排序函数，对给定的数组进行原地排序。
///
/// 该函数使用二分插入排序算法，通过不断将元素插入到已排序部分的正确位置来实现排序。
/// 相比普通插入排序，使用二分查找来确定插入位置，减少了比较次数。
///
/// # 参数
/// * `arr` - 需要排序的可变切片
#[allow(unused)]
pub fn binary_insertion_sort<T: Ord + Clone>(arr: &mut [T]) {
    let len = arr.len();

    // 遍历数组中的每个元素，将其插入到已排序部分的正确位置
    (0..len).for_each(|i| {
        let key = arr[i].clone();
        let index = _binary_search(&arr[..i], &key);

        // 将插入位置后的所有元素向右移动一位，为新元素腾出空间
        arr[index..=i].rotate_right(1);
        arr[index] = key;
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_binary_insertion_sort() {
        let mut arr1 = vec![64, 25, 12, 22, 11];
        let mut arr2 = vec![5, 4, 3, 2, 1];
        let mut arr3 = vec![1, 2, 3, 4, 5];
        let mut arr4: Vec<i32> = vec![]; // Explicitly specify the type for arr4

        binary_insertion_sort(&mut arr1);
        binary_insertion_sort(&mut arr2);
        binary_insertion_sort(&mut arr3);
        binary_insertion_sort(&mut arr4);

        assert_eq!(arr1, vec![11, 12, 22, 25, 64]);
        assert_eq!(arr2, vec![1, 2, 3, 4, 5]);
        assert_eq!(arr3, vec![1, 2, 3, 4, 5]);
        assert_eq!(arr4, Vec::<i32>::new());
    }
}
