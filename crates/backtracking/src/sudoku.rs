/// 解数独函数
///
/// 该函数接收一个9x9的二维数组作为参数，其中0表示空格，其他数字表示已填入的数字。
/// 函数会尝试求解这个数独问题，如果存在唯一解则返回Some(解)，否则返回None。
///
/// # 参数
/// * `board` - 一个9x9的二维数组，代表数独题目，0表示待填充的空格
///
/// # 返回值
/// * `Option<[[u8; 9]; 9]>` - 如果数独有解则返回Some(解)，否则返回None
#[allow(unused)]
pub fn sudoku_solver(board: &[[u8; 9]; 9]) -> Option<[[u8; 9]; 9]> {
    let mut solver = SudokuSolver::new(*board);
    if solver.solve() {
        Some(solver.board)
    } else {
        None
    }
}

/// 数独求解器结构体
///
/// 用于封装数独求解的相关数据和方法
struct SudokuSolver {
    board: [[u8; 9]; 9],
}

impl SudokuSolver {

    /// 创建一个新的数独求解器实例
    ///
    /// # 参数
    /// * `board` - 初始的数独题目
    ///
    /// # 返回值
    /// * `SudokuSolver` - 新创建的求解器实例
    fn new(board: [[u8; 9]; 9]) -> Self {
        Self { board }
    }

    /// 查找数独中的空格子
    ///
    /// 遍历整个数独板，找到第一个值为0的位置
    ///
    /// # 返回值
    /// * `Option<(usize, usize)>` - 如果找到空格子则返回其行列坐标，否则返回None
    fn find_empty_cell(&self) -> Option<(usize, usize)> {
        for row in 0..9 {
            for col in 0..9 {
                if self.board[row][col] == 0 {
                    return Some((row, col));
                }
            }
        }
        None
    }

    /// 检查在指定位置填入某个值是否合法
    ///
    /// 验证在给定坐标填入指定值是否符合数独规则：
    /// 1. 同一行不能有重复数字
    /// 2. 同一列不能有重复数字
    /// 3. 同一个3x3九宫格内不能有重复数字
    ///
    /// # 参数
    /// * `coordinates` - 要检查的位置坐标(行, 列)
    /// * `value` - 要检查的值
    ///
    /// # 返回值
    /// * `bool` - 如果合法返回true，否则返回false
    fn is_value_valid(&self, coordinates: (usize, usize), value: u8) -> bool {
        let (row, col) = coordinates;

        // 检查同一行是否有重复
        for cur_col in 0..9 {
            if self.board[row][cur_col] == value {
                return false;
            }
        }

        // 检查同一列是否有重复
        for cur_row in 0..9 {
            if self.board[cur_row][col] == value {
                return false;
            }
        }

        // 检查同一个3x3九宫格是否有重复
        let start_row = row / 3 * 3;
        let start_col = col / 3 * 3;
        for cur_row in start_row..start_row + 3 {
            for cur_col in start_col..start_col + 3 {
                if self.board[cur_row][cur_col] == value {
                    return false;
                }
            }
        }

        true
    }

    /// 使用回溯算法求解数独
    ///
    /// 递归地尝试填充每个空格子，如果当前填法导致后续无解则回退并尝试其他可能
    ///
    /// # 返回值
    /// * `bool` - 如果成功求解返回true，否则返回false
    fn solve(&mut self) -> bool {
        let empty_cell = self.find_empty_cell();

        // 如果还有空格子需要填充
        if let Some((row, col)) = empty_cell {
            // 尝试填入1-9的每个数字
            for val in 1..=9 {
                // 检查当前数字是否可以填入
                if self.is_value_valid((row, col), val) {
                    // 填入数字
                    self.board[row][col] = val;

                    // 递归求解剩余部分
                    if self.solve() {
                        return true;
                    }

                    // 如果递归求解失败，回退（重置为0）
                    self.board[row][col] = 0;
                }
            }
        } else {
            // 没有空格子了，说明已经求解完成
            return true;
        }

        // 所有可能都尝试过但都失败了
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! test_sudoku_solver {
        ($($name:ident: $board:expr, $expected:expr,)*) => {
            $(
                #[test]
                fn $name() {
                    let result = sudoku_solver(&$board);
                    assert_eq!(result, $expected);
                }
            )*
        };
    }

    test_sudoku_solver! {
        test_sudoku_correct: [
            [3, 0, 6, 5, 0, 8, 4, 0, 0],
            [5, 2, 0, 0, 0, 0, 0, 0, 0],
            [0, 8, 7, 0, 0, 0, 0, 3, 1],
            [0, 0, 3, 0, 1, 0, 0, 8, 0],
            [9, 0, 0, 8, 6, 3, 0, 0, 5],
            [0, 5, 0, 0, 9, 0, 6, 0, 0],
            [1, 3, 0, 0, 0, 0, 2, 5, 0],
            [0, 0, 0, 0, 0, 0, 0, 7, 4],
            [0, 0, 5, 2, 0, 6, 3, 0, 0],
        ], Some([
            [3, 1, 6, 5, 7, 8, 4, 9, 2],
            [5, 2, 9, 1, 3, 4, 7, 6, 8],
            [4, 8, 7, 6, 2, 9, 5, 3, 1],
            [2, 6, 3, 4, 1, 5, 9, 8, 7],
            [9, 7, 4, 8, 6, 3, 1, 2, 5],
            [8, 5, 1, 7, 9, 2, 6, 4, 3],
            [1, 3, 8, 9, 4, 7, 2, 5, 6],
            [6, 9, 2, 3, 5, 1, 8, 7, 4],
            [7, 4, 5, 2, 8, 6, 3, 1, 9],
        ]),

        test_sudoku_incorrect: [
            [6, 0, 3, 5, 0, 8, 4, 0, 0],
            [5, 2, 0, 0, 0, 0, 0, 0, 0],
            [0, 8, 7, 0, 0, 0, 0, 3, 1],
            [0, 0, 3, 0, 1, 0, 0, 8, 0],
            [9, 0, 0, 8, 6, 3, 0, 0, 5],
            [0, 5, 0, 0, 9, 0, 6, 0, 0],
            [1, 3, 0, 0, 0, 0, 2, 5, 0],
            [0, 0, 0, 0, 0, 0, 0, 7, 4],
            [0, 0, 5, 2, 0, 6, 3, 0, 0],
        ], None::<[[u8; 9]; 9]>,
    }
}
