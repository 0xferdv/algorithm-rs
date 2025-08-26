/// 解决 N 皇后问题的主函数。
///
/// 该函数会返回所有可能的解决方案，每个解决方案是一个二维字符串向量，
/// 其中 'Q' 表示皇后，'.' 表示空位。
///
/// # 参数
/// * `n` - 棋盘大小以及皇后的数量
///
/// # 返回值
/// 返回一个包含所有解决方案的向量，每个解决方案是一个字符串向量表示棋盘状态
#[allow(unused)]
pub fn n_queen_solver(n: usize) -> Vec<Vec<String>> {
    let mut solver = NQueenSolver::new(n);
    solver.solve()
}

/// N 皇后问题求解器结构体
///
/// 用于存储当前求解过程中的状态信息，包括棋盘大小、当前棋盘状态和已找到的所有解
struct NQueenSolver {
    /// 棋盘大小（n x n）
    size: usize,
    /// 当前棋盘状态，'Q' 表示皇后，'.' 表示空位
    board: Vec<Vec<char>>,
    /// 存储所有找到的有效解决方案
    solutions: Vec<Vec<String>>,
}

impl NQueenSolver {

    /// 创建一个新的 N 皇后求解器实例
    ///
    /// # 参数
    /// * `size`: 棋盘大小
    ///
    /// # 返回值
    /// 返回初始化后的 NQueenSolver 实例
    fn new(size: usize) -> Self {
        NQueenSolver {
            size,
            board: vec![vec!['.'; size]; size],
            solutions: Vec::new(),
        }
    }

    /// 启动求解过程并返回所有解
    ///
    /// # 返回值
    /// 返回所有有效的解决方案
    fn solve(&mut self) -> Vec<Vec<String>> {
        self.solve_helper(0);
        std::mem::take(&mut self.solutions)
    }

    /// 判断在指定位置放置皇后是否安全
    ///
    /// 检查当前位置是否与之前行中已放置的皇后冲突，包括同列、主对角线和副对角线
    ///
    /// # 参数
    /// * `row`: 要检查的位置行号
    /// * `col`: 要检查的位置列号
    ///
    /// # 返回值
    /// 如果位置安全则返回 true，否则返回 false
    fn is_safe(&self, row: usize, col: usize) -> bool {
        // 遍历之前的每一行进行冲突检测
        for i in 0..row {
            // 检查同列是否有皇后
            if self.board[i][col] == 'Q'
                // 检查左上对角线是否有皇后
                || (col >= row - i && self.board[i][col - (row - i)] == 'Q')
                // 检查右上对角线是否有皇后
                || (col + row - i < self.size && self.board[i][col + (row - i)] == 'Q') {
                return false;
            }
        }
        true
    }

    /// 递归求解函数
    ///
    /// 使用回溯法逐行尝试放置皇后，并收集所有有效解
    ///
    /// # 参数
    /// * `row`: 当前正在处理的行号
    fn solve_helper(&mut self, row: usize) {
        // 基本情况：如果已经处理完所有行，则找到了一个有效解
        if row == self.size {
            // 将当前棋盘状态转换为字符串形式并添加到解集合中
            self.solutions.push(
                self.board.iter().map(|row| row.iter().collect::<String>()).collect());
            return;
        }

        // 在当前行的每一列尝试放置皇后
        for col in 0..self.size {
            // 如果当前位置安全，则放置皇后并继续下一行
            if self.is_safe(row, col) {
                self.board[row][col] = 'Q';
                self.solve_helper(row + 1);
                // 回溯：移除皇后以尝试其他可能性
                self.board[row][col]= '.';
            }
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! test_n_queens_solver {
        ($($name:ident: $tc:expr,)*) => {
            $(
                #[test]
                fn $name() {
                    let (n, expected_solutions) = $tc;
                    let solutions = n_queen_solver(n);
                    assert_eq!(solutions, expected_solutions);
                }
            )*
        };
    }

    test_n_queens_solver! {
        test_0_queens: (0, vec![Vec::<String>::new()]),
        test_1_queen: (1, vec![vec!["Q"]]),
        test_2_queens:(2, Vec::<Vec<String>>::new()),
        test_3_queens:(3, Vec::<Vec<String>>::new()),
        test_4_queens: (4, vec![
            vec![".Q..",
                 "...Q",
                 "Q...",
                 "..Q."],
            vec!["..Q.",
                 "Q...",
                 "...Q",
                 ".Q.."],
        ]),
        test_5_queens:(5, vec![
            vec!["Q....",
                 "..Q..",
                 "....Q",
                 ".Q...",
                 "...Q."],
            vec!["Q....",
                 "...Q.",
                 ".Q...",
                 "....Q",
                 "..Q.."],
            vec![".Q...",
                 "...Q.",
                 "Q....",
                 "..Q..",
                 "....Q"],
            vec![".Q...",
                 "....Q",
                 "..Q..",
                 "Q....",
                 "...Q."],
            vec!["..Q..",
                 "Q....",
                 "...Q.",
                 ".Q...",
                 "....Q"],
            vec!["..Q..",
                 "....Q",
                 ".Q...",
                 "...Q.",
                 "Q...."],
            vec!["...Q.",
                 "Q....",
                 "..Q..",
                 "....Q",
                 ".Q..."],
            vec!["...Q.",
                 ".Q...",
                 "....Q",
                 "..Q..",
                 "Q...."],
            vec!["....Q",
                 ".Q...",
                 "...Q.",
                 "Q....",
                 "..Q.."],
            vec!["....Q",
                 "..Q..",
                 "Q....",
                 "...Q.",
                 ".Q..."],
        ]),
        test_6_queens: (6, vec![
            vec![".Q....",
                 "...Q..",
                 ".....Q",
                 "Q.....",
                 "..Q...",
                 "....Q."],
            vec!["..Q...",
                 ".....Q",
                 ".Q....",
                 "....Q.",
                 "Q.....",
                 "...Q.."],
            vec!["...Q..",
                 "Q.....",
                 "....Q.",
                 ".Q....",
                 ".....Q",
                 "..Q..."],
            vec!["....Q.",
                 "Q.....",
                 "..Q..",
                 ".....Q",
                 "...Q..",
                 ".Q...."],
        ]),
    }
}
