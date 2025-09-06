#![allow(unused)]

/// 使用高斯消元法求解线性方程组。
///
/// 该函数接收一个增广矩阵（系数矩阵与常数列合并），通过行变换将其化为上三角形式，
/// 然后回代求解得到方程组的解。
///
/// # 参数
/// * `matrix` - 表示线性方程组的增广矩阵，维度为 n × (n+1)，其中前 n 列是系数矩阵，最后一列是常数项。
///
/// # 返回值
/// 返回一个包含 n 个元素的向量，表示每个未知数的解。
///
/// # 注意事项
/// - 系数矩阵必须是方阵（即行数等于列数）。
/// - 若主元为 0，可能导致除零错误或提示无穷多解。
pub fn gaussian_elimination(matrix: &mut [Vec<f32>]) -> Vec<f32> {
    let size = matrix.len();
    // 确保输入矩阵是一个 n × (n+1) 的增广矩阵
    assert_eq!(size, matrix[0].len() - 1);

    // 第一阶段：将矩阵转换为上三角形式（行阶梯形）
    for i in 0..size - 1 {
        for j in i..size - 1 {
            echelon(matrix, i, j);
        }
    }

    // 第二阶段：从下往上进行回代，使矩阵变为简化行阶梯形
    for i in (1..size).rev() {
        eliminate(matrix, i);
    }

    // 检查是否有主元为 0，这可能意味着系统有无穷多解或无解
    #[allow(clippy::needless_range_loop)]
    for i in 0..size {
        if matrix[i][i] == 0f32 {
            println!("Infinite many solutions");
        }
    }

    // 构造结果向量，存储每个变量的解
    let mut result: Vec<f32> = vec![0f32; size];
    (0..size).for_each(|idx| {
        result[idx] = matrix[idx][size] / matrix[idx][idx];
    });

    result
}

/// 将矩阵的某一行减去另一行的倍数，以实现行阶梯形变换。
///
/// 此函数用于将第 j+1 行在第 i 列位置上的元素变为 0。
///
/// # 参数
/// * `matrix` - 要操作的增广矩阵。
/// * `i` - 当前行索引。
/// * `j` - 要被变换的行索引（j+1 行）。
fn echelon(matrix: &mut [Vec<f32>], i: usize, j: usize) {
    let size = matrix.len();
    if matrix[i][i] != 0f32 {
        // 计算需要消除的倍数因子
        let factor = matrix[j + 1][i] / matrix[i][i];
        // 对第 j+1 行执行行变换
        (i..=size).for_each(|k| {
            matrix[j + 1][k] -= factor * matrix[i][k];
        });
    }
}

/// 回代过程中的行变换函数。
///
/// 将当前行上方的所有行在当前列位置上的元素变为 0。
///
/// # 参数
/// * `matrix` - 要操作的增广矩阵。
/// * `i` - 当前行索引。
fn eliminate(matrix: &mut [Vec<f32>], i: usize) {
    let size = matrix.len();
    if matrix[i][i] != 0f32 {
        // 从当前行向上逐行进行消元
        for j in (1..=i).rev() {
            let factor = matrix[j - 1][i] / matrix[i][i];
            // 对第 j-1 行执行行变换
            for k in (0..=size).rev() {
                matrix[j - 1][k] -= factor * matrix[i][k];
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gauss() {
        let mut matrix: Vec<Vec<f32>> = vec![
            vec![1.5, 2.0, 1.0, -1.0, -2.0, 1.0, 1.0],
            vec![3.0, 3.0, -1.0, 16.0, 18.0, 1.0, 1.0],
            vec![1.0, 1.0, 3.0, -2.0, -6.0, 1.0, 1.0],
            vec![1.0, 1.0, 99.0, 19.0, 2.0, 1.0, 1.0],
            vec![1.0, -2.0, 16.0, 1.0, 9.0, 10.0, 1.0],
            vec![1.0, 3.0, 1.0, -5.0, 1.0, 1.0, 95.0],
        ];
        let result = vec![
            -264.05893, 159.63196, -6.156921, 35.310387, -18.806696, 81.67839,
        ];
        assert_eq!(gaussian_elimination(&mut matrix), result);
    }
}
