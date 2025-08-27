/// 错误类型枚举，用于表示迷宫求解过程中可能出现的错误
#[derive(Debug, PartialEq, Eq)]
pub enum MazeError {
    /// 迷宫为空
    EmptyMaze,
    /// 起始位置越界
    OutOfBoundPos,
    /// 迷宫表示不规范（行长度不一致）
    ImproperMazeRepr,
}

/// 在给定的迷宫中寻找从指定起点到右下角终点的路径
///
/// # 参数
/// * `maze` - 二维布尔向量，表示迷宫。true表示可通过，false表示障碍物
/// * `start_x` - 起点的行索引
/// * `start_y` - 起点的列索引
///
/// # 返回值
/// * `Ok(Some(solution))` - 找到路径，solution为标记路径的二维布尔向量
/// * `Ok(None)` - 未找到路径
/// * `Err(MazeError)` - 输入参数有误
pub fn find_path_in_maze(
    maze: &[Vec<bool>],
    start_x: usize,
    start_y: usize,
) -> Result<Option<Vec<Vec<bool>>>, MazeError> {
    // 检查迷宫是否为空
    if maze.is_empty() {
        return Err(MazeError::EmptyMaze);
    }
    // 检查起始位置是否越界
    if start_x >= maze.len() || start_y >= maze[0].len() {
        return Err(MazeError::OutOfBoundPos);
    }
    // 检查迷宫表示是否规范（所有行长度一致）
    if maze.iter().any(|row| row.len() != maze[0].len()) {
        return Err(MazeError::ImproperMazeRepr);
    }
    let maze_instance = Maze::new(maze.to_owned());
    Ok(maze_instance.find_path(start_x, start_y))
}

/// 迷宫结构体，封装迷宫数据和求解逻辑
struct Maze {
    maze: Vec<Vec<bool>>,
}

impl Maze {
    /// 四个方向的移动向量：右、下、左、上
    const MOVES: [(isize, isize); 4] = [(0, 1), (1, 0), (0, -1), (-1, 0)];

    /// 创建新的迷宫实例
    ///
    /// # 参数
    /// * `maze` - 迷宫数据
    fn new(maze: Vec<Vec<bool>>) -> Self {
        Self { maze }
    }

    /// 获取迷宫宽度（列数）
    fn width(&self) -> usize {
        self.maze[0].len()
    }

    /// 获取迷宫高度（行数）
    fn height(&self) -> usize {
        self.maze.len()
    }

    /// 寻找从指定起点到右下角终点的路径
    ///
    /// # 参数
    /// * `start_x` - 起点的行索引
    /// * `start_y` - 起点的列索引
    ///
    /// # 返回值
    /// * `Some(solution)` - 找到路径，solution为标记路径的二维布尔向量
    /// * `None` - 未找到路径
    fn find_path(&self, start_x: usize, start_y: usize) -> Option<Vec<Vec<bool>>> {
        // 初始化解矩阵，全部置为false
        let mut solution = vec![vec![false; self.width()]; self.height()];
        // 调用递归求解函数
        if self.solve(start_x as isize, start_y as isize, &mut solution) {
            Some(solution)
        } else {
            None
        }
    }

    /// 递归求解迷宫路径
    ///
    /// # 参数
    /// * `x` - 当前位置的行坐标
    /// * `y` - 当前位置的列坐标
    /// * `solution` - 解矩阵，记录已访问的位置
    ///
    /// # 返回值
    /// * `true` - 找到路径
    /// * `false` - 未找到路径
    fn solve(&self, x: isize, y: isize, solution: &mut [Vec<bool>]) -> bool {
        // 如果到达终点（右下角），标记该位置并返回true
        if x == (self.height() as isize - 1) && y == (self.width() as isize - 1) {
            solution[x as usize][y as usize] = true;
            return true;
        }
        // 检查当前位置是否有效
        if self.is_valid(x, y, solution) {
            // 标记当前位置为已访问
            solution[x as usize][y as usize] = true;
            // 尝试四个方向的移动
            for &(dx, dy) in &Self::MOVES {
                if self.solve(x + dx, y + dy, solution) {
                    return true;
                }
            }
            // 如果四个方向都无法到达终点，回溯，取消当前位置的标记
            solution[x as usize][y as usize] = false;
            return false;
        }
        false
    }

    /// 检查位置是否有效（在边界内、可通过且未访问过）
    ///
    /// # 参数
    /// * `x` - 行坐标
    /// * `y` - 列坐标
    /// * `solution` - 解矩阵，用于检查是否已访问
    ///
    /// # 返回值
    /// * `true` - 位置有效
    /// * `false` - 位置无效
    fn is_valid(&self, x: isize, y: isize, solution: &[Vec<bool>]) -> bool {
        x >= 0
            && y >= 0
            && x < self.height() as isize
            && y < self.width() as isize
            && self.maze[x as usize][y as usize]  // 检查是否可通过
            && !solution[x as usize][y as usize]  // 检查是否已访问
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// 测试宏，用于批量生成测试用例
    macro_rules! test_find_path_in_maze {
        ($($name:ident: $start_x:expr, $start_y:expr, $maze:expr, $expected:expr,)*) => {
            $(
                #[test]
                fn $name() {
                    let solution = find_path_in_maze($maze, $start_x, $start_y);
                    assert_eq!(solution, $expected);
                    if let Ok(Some(expected_solution)) = &solution {
                        assert_eq!(expected_solution[$start_x][$start_y], true);
                    }
                }
            )*
        }
    }

    test_find_path_in_maze! {
        maze_with_solution_5x5: 0, 0, &[
            vec![true, false, true, false, false],
            vec![true, true, false, true, false],
            vec![false, true, true, true, false],
            vec![false, false, false, true, true],
            vec![false, true, false, false, true],
        ], Ok(Some(vec![
            vec![true, false, false, false, false],
            vec![true, true, false, false, false],
            vec![false, true, true, true, false],
            vec![false, false, false, true, true],
            vec![false, false, false, false, true],
        ])),
        maze_with_solution_6x6: 0, 0, &[
            vec![true, false, true, false, true, false],
            vec![true, true, false, true, false, true],
            vec![false, true, true, true, true, false],
            vec![false, false, false, true, true, true],
            vec![false, true, false, false, true, false],
            vec![true, true, true, true, true, true],
        ], Ok(Some(vec![
            vec![true, false, false, false, false, false],
            vec![true, true, false, false, false, false],
            vec![false, true, true, true, true, false],
            vec![false, false, false, false, true, false],
            vec![false, false, false, false, true, false],
            vec![false, false, false, false, true, true],
        ])),
        maze_with_solution_8x8: 0, 0, &[
            vec![true, false, false, false, false, false, false, true],
            vec![true, true, false, true, true, true, false, false],
            vec![false, true, true, true, false, false, false, false],
            vec![false, false, false, true, false, true, true, false],
            vec![false, true, false, true, true, true, false, true],
            vec![true, false, true, false, false, true, true, true],
            vec![false, false, true, true, true, false, true, true],
            vec![true, true, true, false, true, true, true, true],
        ], Ok(Some(vec![
            vec![true, false, false, false, false, false, false, false],
            vec![true, true, false, false, false, false, false, false],
            vec![false, true, true, true, false, false, false, false],
            vec![false, false, false, true, false, false, false, false],
            vec![false, false, false, true, true, true, false, false],
            vec![false, false, false, false, false, true, true, true],
            vec![false, false, false, false, false, false, false, true],
            vec![false, false, false, false, false, false, false, true],
        ])),
        maze_without_solution_4x4: 0, 0, &[
            vec![true, false, false, false],
            vec![true, true, false, false],
            vec![false, false, true, false],
            vec![false, false, false, true],
        ], Ok(None::<Vec<Vec<bool>>>),
        maze_with_solution_3x4: 0, 0, &[
            vec![true, false, true, true],
            vec![true, true, true, false],
            vec![false, true, true, true],
        ], Ok(Some(vec![
            vec![true, false, false, false],
            vec![true, true, true, false],
            vec![false, false, true, true],
        ])),
        maze_without_solution_3x4: 0, 0, &[
            vec![true, false, true, true],
            vec![true, false, true, false],
            vec![false, true, false, true],
        ], Ok(None::<Vec<Vec<bool>>>),
        improper_maze_representation: 0, 0, &[
            vec![true],
            vec![true, true],
            vec![true, true, true],
            vec![true, true, true, true]
        ], Err(MazeError::ImproperMazeRepr),
        out_of_bound_start: 0, 3, &[
            vec![true, false, true],
            vec![true, true],
            vec![false, true, true],
        ], Err(MazeError::OutOfBoundPos),
        empty_maze: 0, 0, &[], Err(MazeError::EmptyMaze),
        maze_with_single_cell: 0, 0, &[
            vec![true],
        ], Ok(Some(vec![
                vec![true]
        ])),
        maze_with_one_row_and_multiple_columns: 0, 0, &[
            vec![true, false, true, true, false]
        ], Ok(None::<Vec<Vec<bool>>>),
        maze_with_multiple_rows_and_one_column: 0, 0, &[
            vec![true],
            vec![true],
            vec![false],
            vec![true],
        ], Ok(None::<Vec<Vec<bool>>>),
        maze_with_walls_surrounding_border: 0, 0, &[
            vec![false, false, false],
            vec![false, true, false],
            vec![false, false, false],
        ], Ok(None::<Vec<Vec<bool>>>),
        maze_with_no_walls: 0, 0, &[
            vec![true, true, true],
            vec![true, true, true],
            vec![true, true, true],
        ], Ok(Some(vec![
            vec![true, true, true],
            vec![false, false, true],
            vec![false, false, true],
        ])),
        maze_with_going_back: 0, 0, &[
            vec![true,  true,  true,  true, true,   true],
            vec![false, false, false, true, false,  true],
            vec![true,  true,  true,  true,  false, false],
            vec![true,  false, false, false, false, false],
            vec![true,  false, false, false, true, true],
            vec![true,  false, true,  true,  true,  false],
            vec![true,  false, true , false, true,  false],
            vec![true,  true,  true,  false, true,  true],
        ], Ok(Some(vec![
            vec![true,  true,  true,  true, false,  false],
            vec![false, false, false, true, false,  false],
            vec![true,  true,  true,  true,  false, false],
            vec![true,  false, false, false, false, false],
            vec![true,  false, false, false, false, false],
            vec![true,  false, true,  true,  true,  false],
            vec![true,  false, true , false, true,  false],
            vec![true,  true,  true,  false, true,  true],
        ])),
    }
}
