#[allow(unused)]
pub fn bead_sort(a: &mut [usize]) {
    // 找到数组中的最大元素，用于确定珠子矩阵的列数
    let mut max_element = a[0];
    (1..a.len()).for_each(|i| {
        if a[i] > max_element {
            max_element = a[i];
        }
    });

    // 创建一个二维向量 beads，行数为数组长度，列数为最大元素值
    let mut beads = vec![vec![0; max_element]; a.len()];

    // 根据每个元素的值，在 beads 矩阵中放置相应数量的珠子（标记为1）
    for i in 0..a.len() {
        for j in (0..a[i]).rev() {
            beads[i][j] = 1;
        }
    }

    // 模拟珠子下落过程，按列统计并重新分配珠子位置，实现排序
    for j in 0..max_element {
        let mut sum = 0;
        // 统计当前列有多少个珠子，并将原位置清零
        (0..a.len()).for_each(|i| {
            sum += beads[i][j];
            beads[i][j] = 0;
        });
        // 将统计到的珠子从底部开始填充到数组中
        for k in ((a.len() - sum)..a.len()).rev() {
            a[k] = j + 1;
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::{have_same_elements, is_sorted};

    #[test]
    fn descending() {
        let mut a = [5, 4, 3, 2, 1];
        bead_sort(&mut a);
        assert_eq!(a, [1, 2, 3, 4, 5]);
    }

    #[test]
    fn ascending() {
        let mut a = [1, 2, 3, 4, 5];
        bead_sort(&mut a);
        assert_eq!(a, [1, 2, 3, 4, 5]);
    }

    #[test]
    fn mix_values() {
        //pre-sorted
        let mut ve2: [usize; 5] = [7, 9, 6, 2, 3];
        let cloned = ve2;
        bead_sort(&mut ve2);
        assert!(is_sorted(&ve2) && have_same_elements(&ve2, &cloned));
    }
}
