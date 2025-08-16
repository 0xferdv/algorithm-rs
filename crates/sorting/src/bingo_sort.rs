use std::cmp::{max, min};

/// 遍历数组，更新最小值和最大值
///
/// # Arguments
///
/// * `vec` - 输入的整数切片，用于查找最小值和最大值
/// * `bingo` - 可变引用，用于存储当前找到的最小值
/// * `next_bingo` - 可变引用，用于存储当前找到的最大值
#[allow(unused)]
fn max_min(vec: &[i32], bingo: &mut i32, next_bingo: &mut i32) {
    // 从第二个元素开始遍历，更新最小值和最大值
    for &element in vec.iter().skip(1) {
        *bingo = min(*bingo, element);
        *next_bingo = max(*next_bingo, element);
    }
}


/// 使用Bingo排序算法对整数数组进行原地排序
///
/// Bingo排序是一种变种的计数排序，通过不断找到当前范围内的最小值并将其放到正确位置来实现排序
///
/// # Arguments
///
/// * `vec` - 可变引用的整数切片，将被原地排序
#[allow(unused)]
pub fn bingo_sort(vec: &mut [i32]) {
    // 如果数组为空，直接返回
    if vec.is_empty() {
        return;
    }

    // 初始化最小值和最大值为第一个元素
    let mut bingo = vec[0];
    let mut next_bingo = vec[0];

    // 查找数组中的最小值和最大值
    max_min(vec, &mut bingo, &mut next_bingo);

    // 获取数组中的最大元素
    let largest_element = next_bingo;

    // 记录下一个待放置元素的位置
    let mut next_element_pos = 0;

    // 从最小值到最大值依次处理每个值
    for (bingo, _next_bingo) in (bingo..=largest_element).zip(bingo..=largest_element) {
        let start_pos = next_element_pos;

        // 在未排序部分查找当前值，并将其交换到正确位置
        for i in start_pos..vec.len() {
            if vec[i] == bingo {
                vec.swap(i, next_element_pos);
                next_element_pos += 1;
            }
        }
    }
}

/// 打印数组元素
///
/// # Arguments
///
/// * `arr` - 要打印的整数切片
#[allow(unused)]
fn print_array(arr: &[i32]) {
    print!("Sorted array: ");
    for &element in arr {
        print!("{} ", element);
    }
    println!();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bingo_sort() {
        let mut arr = vec![5, 4, 8, 5, 4, 8, 5, 4, 4, 4];
        bingo_sort(&mut arr);
        print_array(&arr);
        assert_eq!(arr, vec![4, 4, 4, 4, 4, 5, 5, 5, 8, 8]);

        let mut arr2 = vec![10, 9, 8, 7, 6, 5, 4, 3, 2, 1];
        bingo_sort(&mut arr2);
        print_array(&arr2);
        assert_eq!(arr2, vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10]);

        let mut arr3 = vec![0, 1, 0, 1, 0, 1];
        bingo_sort(&mut arr3);
        print_array(&arr3);
        assert_eq!(arr3, vec![0, 0, 0, 1, 1, 1]);
    }

    #[test]
    fn test_empty_array() {
        let mut arr = Vec::new();
        bingo_sort(&mut arr);
        assert_eq!(arr, Vec::new());
    }

    #[test]
    fn test_single_element_array() {
        let mut arr = vec![42];
        bingo_sort(&mut arr);
        assert_eq!(arr, vec![42]);
    }

    #[test]
    fn test_negative_numbers() {
        let mut arr = vec![-5, -4, -3, -2, -1];
        bingo_sort(&mut arr);
        assert_eq!(arr, vec![-5, -4, -3, -2, -1]);
    }

    #[test]
    fn test_already_sorted() {
        let mut arr = vec![1, 2, 3, 4, 5];
        bingo_sort(&mut arr);
        assert_eq!(arr, vec![1, 2, 3, 4, 5]);
    }

    #[test]
    fn test_reverse_sorted() {
        let mut arr = vec![5, 4, 3, 2, 1];
        bingo_sort(&mut arr);
        assert_eq!(arr, vec![1, 2, 3, 4, 5]);
    }

    #[test]
    fn test_duplicates() {
        let mut arr = vec![1, 2, 3, 4, 5, 1, 2, 3, 4, 5];
        bingo_sort(&mut arr);
        assert_eq!(arr, vec![1, 1, 2, 2, 3, 3, 4, 4, 5, 5]);
    }
}

