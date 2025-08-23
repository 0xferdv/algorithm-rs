use std::cmp::Ordering;

/// 鞍背搜索算法（Saddleback Search）用于在一个行和列均有序的二维矩阵中查找指定元素。
///
/// 算法从矩阵的右上角开始搜索，根据当前元素与目标元素的比较结果决定移动方向：
/// - 如果相等，则返回当前位置；
/// - 如果目标元素较大，则向下移动一行；
/// - 如果目标元素较小，则向左移动一列。
///
/// # 参数
/// * `matrix`: 一个二维向量，表示行和列都按升序排列的矩阵。
/// * `element`: 要查找的目标元素。
///
/// # 返回值
/// 返回一个元组 `(row, col)` 表示元素在矩阵中的位置（从1开始计数）。
/// 如果未找到元素，则返回 `(0, 0)`。
#[allow(unused)]
pub fn saddleback_search(matrix: &[Vec<i32>], element: i32) -> (usize, usize) {
    let mut left_idx = 0;
    let mut right_idx = matrix[0].len() - 1;

    // 从右上角开始进行鞍背搜索
    while left_idx < matrix.len() {
        match element.cmp(&matrix[left_idx][right_idx]) {
            Ordering::Equal => return (left_idx + 1, right_idx + 1),
            Ordering::Greater => left_idx += 1,
            Ordering::Less => {
                if right_idx == 0 {
                    break;
                }
                right_idx -= 1;
            }
        }
    }

    // 未找到目标元素，返回 (0, 0)
    (0, 0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_element_not_found() {
        let matrix = vec![vec![1, 10, 100], vec![2, 20, 200], vec![3, 30, 300]];
        assert_eq!(saddleback_search(&matrix, 123), (0, 0));
    }

    #[test]
    fn test_element_at_top_left() {
        let matrix = vec![vec![1, 10, 100], vec![2, 20, 200], vec![3, 30, 300]];
        assert_eq!(saddleback_search(&matrix, 1), (1, 1));
    }

    #[test]
    fn test_element_at_bottom_right() {
        let matrix = vec![vec![1, 10, 100], vec![2, 20, 200], vec![3, 30, 300]];
        assert_eq!(saddleback_search(&matrix, 300), (3, 3));
    }

    #[test]
    fn test_element_at_top_right() {
        let matrix = vec![vec![1, 10, 100], vec![2, 20, 200], vec![3, 30, 300]];
        assert_eq!(saddleback_search(&matrix, 100), (1, 3));
    }

    #[test]
    fn test_element_at_bottom_left() {
        let matrix = vec![vec![1, 10, 100], vec![2, 20, 200], vec![3, 30, 300]];
        assert_eq!(saddleback_search(&matrix, 3), (3, 1));
    }

    #[test]
    fn test_element_in_middle() {
        let matrix = vec![
            vec![1, 10, 100, 1000],
            vec![2, 20, 200, 2000],
            vec![3, 30, 300, 3000],
        ];
        assert_eq!(saddleback_search(&matrix, 200), (2, 3));
    }
}
