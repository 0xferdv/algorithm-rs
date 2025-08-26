/// 寻找骑士巡游路径的公共接口函数。
///
/// 该函数初始化一个骑士巡游求解器，并尝试从指定起点开始寻找一条遍历整个棋盘的路径。
///
/// # 参数
/// * `size_x`: 棋盘的宽度（列数）
/// * `size_y`: 棋盘的高度（行数）
/// * `start_x`: 起始位置的x坐标（列索引）
/// * `start_y`: 起始位置的y坐标（行索引）
///
/// # 返回值
/// 如果找到有效的骑士巡游路径，则返回包含路径信息的二维向量；否则返回None。
#[allow(unused)]
pub fn find_knight_tour(
    size_x: usize,
    size_y: usize,
    start_x: usize,
    start_y: usize,
) -> Option<Vec<Vec<usize>>> {
    let mut tour = KnightTour::new(size_x, size_y);
    tour.find_tour(start_x, start_y)
}

/// 骑士巡游问题求解器结构体
///
/// 用于存储当前棋盘状态并实现骑士巡游算法的核心逻辑。
struct KnightTour {
    board: Vec<Vec<usize>>,
}

impl KnightTour {
    /// 骑士在国际象棋中的8种可能移动方式（L型移动）
    const MOVES: [(isize, isize); 8] = [
        (2, 1),
        (1, 2),
        (-1, 2),
        (-2, 1),
        (-2, -1),
        (-1, -2),
        (1, -2),
        (2, -1),
    ];

    /// 创建一个新的骑士巡游求解器实例
    ///
    /// # 参数
    /// * `size_x`: 棋盘的宽度（列数）
    /// * `size_y`: 棋盘的高度（行数）
    ///
    /// # 返回值
    /// 返回初始化后的KnightTour实例，其中棋盘所有位置都为0（未访问）
    fn new(size_x: usize, size_y: usize) -> Self {
        let board = vec![vec![0; size_x]; size_y];
        KnightTour { board }
    }

    /// 获取棋盘的宽度（列数）
    fn size_x(&self) -> usize {
        self.board.len()
    }

    /// 获取棋盘的高度（行数）
    fn size_y(&self) -> usize {
        self.board[0].len()
    }

    /// 检查给定坐标是否在棋盘范围内且未被访问过
    ///
    /// # 参数
    /// * `x`: 待检查位置的x坐标
    /// * `y`: 待检查位置的y坐标
    ///
    /// # 返回值
    /// 如果位置合法且未被访问则返回true，否则返回false
    fn is_safe(&self, x: isize, y: isize) -> bool {
        x >= 0
            && y >= 0
            && x < self.size_x() as isize
            && y < self.size_y() as isize
            && self.board[x as usize][y as usize] == 0
    }

    /// 使用回溯法递归求解骑士巡游问题
    ///
    /// 从当前位置开始，尝试所有可能的移动方向，直到完成整个巡游或确定无解。
    ///
    /// # 参数
    /// * `x`: 当前位置的x坐标
    /// * `y`: 当前位置的y坐标
    /// * `move_count`: 已经完成的移动步数
    ///
    /// # 返回值
    /// 如果成功找到完整巡游路径则返回true，否则返回false
    fn solve_tour(
        &mut self,
        x: isize,
        y: isize,
        move_count: usize,
    ) -> bool {
        // 基本情况：如果已访问所有格子，说明找到了解决方案
        if move_count == self.size_x() * self.size_y() {
            return true;
        }

        // 尝试所有可能的移动方向
        for &(dx, dy) in &Self::MOVES {
            let next_x = x + dx;
            let next_y = y + dy;
            // 如果下一步位置合法
            if self.is_safe(next_x, next_y) {
                // 标记该位置为已访问（记录步数）
                self.board[next_x as usize][next_y as usize] = move_count + 1;
                // 递归求解下一步
                if self.solve_tour(next_x, next_y, move_count + 1) {
                    return true;
                }
                // 回溯：如果后续无法找到解，则取消标记
                self.board[next_x as usize][next_y as usize] = 0;
            }
        }
        false
    }

    /// 寻找从指定起点开始的骑士巡游路径
    ///
    /// 这是求解骑士巡游问题的入口方法，负责初始化起始位置并调用递归求解函数。
    ///
    /// # 参数
    /// * `start_x`: 起始位置的x坐标
    /// * `start_y`: 起始位置的y坐标
    ///
    /// # 返回值
    /// 如果找到有效路径则返回棋盘状态的副本，否则返回None
    fn find_tour(&mut self, start_x: usize, start_y: usize) -> Option<Vec<Vec<usize>>> {
        // 检查起始位置是否合法
        if !self.is_safe(start_x as isize, start_y as isize) {
            return None;
        }
        // 标记起始位置为第一步
        self.board[start_x][start_y] = 1;
        // 调用递归求解函数
        if !self.solve_tour(start_x as isize, start_y as isize, 1) {
            return None;
        }
        // 返回找到的解决方案
        Some(self.board.clone())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! test_find_knight_tour {
        ($($name:ident: $tc:expr,)*) => {
        $(
            #[test]
            fn $name() {
                let (size_x, size_y, start_x, start_y, expected) = $tc;
                if expected.is_some() {
                    assert_eq!(expected.clone().unwrap()[start_x][start_y], 1)
                }
                assert_eq!(find_knight_tour(size_x, size_y, start_x, start_y), expected);
            }
        )*
        }
    }
    test_find_knight_tour! {
        test_knight_tour_5x5: (5, 5, 0, 0, Some(vec![
            vec![1, 6, 15, 10, 21],
            vec![14, 9, 20, 5, 16],
            vec![19, 2, 7, 22, 11],
            vec![8, 13, 24, 17, 4],
            vec![25, 18, 3, 12, 23],
        ])),
        test_knight_tour_6x6: (6, 6, 0, 0, Some(vec![
            vec![1, 16, 7, 26, 11, 14],
            vec![34, 25, 12, 15, 6, 27],
            vec![17, 2, 33, 8, 13, 10],
            vec![32, 35, 24, 21, 28, 5],
            vec![23, 18, 3, 30, 9, 20],
            vec![36, 31, 22, 19, 4, 29],
        ])),
        test_knight_tour_8x8: (8, 8, 0, 0, Some(vec![
            vec![1, 60, 39, 34, 31, 18, 9, 64],
            vec![38, 35, 32, 61, 10, 63, 30, 17],
            vec![59, 2, 37, 40, 33, 28, 19, 8],
            vec![36, 49, 42, 27, 62, 11, 16, 29],
            vec![43, 58, 3, 50, 41, 24, 7, 20],
            vec![48, 51, 46, 55, 26, 21, 12, 15],
            vec![57, 44, 53, 4, 23, 14, 25, 6],
            vec![52, 47, 56, 45, 54, 5, 22, 13],
        ])),
        test_no_solution: (5, 5, 2, 1, None::<Vec<Vec<usize>>>),
        test_invalid_start_position: (8, 8, 10, 10, None::<Vec<Vec<usize>>>),
    }
}
