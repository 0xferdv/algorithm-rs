/// 颜色枚举，用于表示荷兰国旗问题中的三种颜色
/// Red: 红色
/// White: 白色
/// Blue: 蓝色
#[derive(PartialOrd, PartialEq, Eq)]
#[allow(unused)]
pub enum Colors {
    Red,
    White,
    Blue,
}

use Colors::{Red, White, Blue};

/// 荷兰国旗排序算法实现
///
/// 使用三路快排的思想，将只包含红白蓝三种颜色的数组进行排序，
/// 使得相同颜色的元素相邻，并按照红、白、蓝的顺序排列。
///
/// # 参数
/// * `sequences` - 包含Colors枚举值的可变数组
///
/// # 返回值
/// 返回排序后的Colors数组，顺序为红、白、蓝
#[allow(unused)]
pub fn dutch_national_flag_sort(mut sequences: Vec<Colors>) -> Vec<Colors> {
    let length = sequences.len();
    if length <= 1 {
        return sequences;
    }

    // 初始化三个指针
    // low: 指向下一个红色元素应该放置的位置
    // mid: 当前正在处理的元素位置
    // high: 指向下一个蓝色元素应该放置的位置
    let mut low = 0;
    let mut mid = 0;
    let mut high = length - 1;

    // 三路分区算法核心循环
    // 当mid指针超过high指针时停止
    while mid <= high {
        match sequences[mid] {
            // 遇到红色元素：与low位置元素交换，然后两个指针都前移
            Red => {
                sequences.swap(low, mid);
                low += 1;
                mid += 1;
            }
            // 遇到白色元素：保持原位，只移动mid指针
            White => {
                mid += 1;
            }
            // 遇到蓝色元素：与high位置元素交换，只移动high指针
            Blue => {
                sequences.swap(mid, high);
                high -= 1;
            }
        }
    }
    sequences
}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::is_sorted;

    #[test]
    fn random_array() {
        let arr = vec![
            Red, Blue, White, White, Blue, Blue, Red, Red, White, Blue, White, Red, White, Blue,
        ];
        let arr = dutch_national_flag_sort(arr);
        assert!(is_sorted(&arr))
    }

    #[test]
    fn sorted_array() {
        let arr = vec![
            Red, Red, Red, Red, Red, White, White, White, White, White, Blue, Blue, Blue, Blue,
        ];
        let arr = dutch_national_flag_sort(arr);
        assert!(is_sorted(&arr))
    }
}
