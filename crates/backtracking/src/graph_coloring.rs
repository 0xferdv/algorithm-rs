/// 图着色错误类型枚举
///
/// 定义了图着色过程中可能出现的错误情况
#[derive(Debug, PartialEq, Eq)]
pub enum GraphColoringError {
    /// 邻接矩阵为空
    EmptyAdjacencyMatrix,
    /// 邻接矩阵格式不正确（非方阵）
    ImproperAdjacencyMatrix,
}

/// 生成图的所有有效着色方案
///
/// # 参数
/// * `adjacency_matrix` - 图的邻接矩阵，表示顶点之间的连接关系
/// * `num_colors` - 可用的颜色数量
///
/// # 返回值
/// * `Ok(Some(Vec<Vec<usize>>))` - 找到有效的着色方案，返回所有可能的着色结果
/// * `Ok(None)` - 没有找到有效的着色方案
/// * `Err(GraphColoringError)` - 输入参数有误，返回相应的错误类型
#[allow(unused)]
pub fn generate_colorings(
    adjacency_matrix: Vec<Vec<bool>>,
    num_colors: usize,
) -> Result<Option<Vec<Vec<usize>>>, GraphColoringError> {
    Ok(GraphColoring::new(adjacency_matrix)?.find_solutions(num_colors))
}

/// 图着色求解器结构体
///
/// 用于存储图的邻接矩阵、当前顶点着色状态以及找到的所有解
struct GraphColoring {
    /// 图的邻接矩阵
    adjacency_matrix: Vec<Vec<bool>>,
    /// 每个顶点当前分配的颜色
    vertex_colors: Vec<usize>,
    /// 存储所有有效的着色方案
    solutions: Vec<Vec<usize>>,
}

impl GraphColoring {
    /// 创建新的图着色求解器实例
    ///
    /// # 参数
    /// * `adjacency_matrix` - 图的邻接矩阵
    ///
    /// # 返回值
    /// * `Ok(GraphColoring)` - 成功创建实例
    /// * `Err(GraphColoringError)` - 邻接矩阵为空或不是方阵时返回错误
    fn new(adjacency_matrix: Vec<Vec<bool>>) -> Result<Self, GraphColoringError> {
        let num_vertices = adjacency_matrix.len();
        // 检查邻接矩阵是否为空
        if num_vertices == 0 {
            return Err(GraphColoringError::EmptyAdjacencyMatrix);
        }
        // 检查邻接矩阵是否为方阵
        if adjacency_matrix.iter().any(|row| row.len() != num_vertices) {
            return Err(GraphColoringError::ImproperAdjacencyMatrix);
        }
        Ok(GraphColoring {
            adjacency_matrix,
            vertex_colors: vec![usize::MAX; num_vertices],
            solutions: Vec::new(),
        })
    }

    /// 获取图中顶点的数量
    ///
    /// # 返回值
    /// * 顶点总数
    fn num_vertices(&self) -> usize {
        self.adjacency_matrix.len()
    }

    /// 检查给定顶点使用指定颜色是否有效
    ///
    /// # 参数
    /// * `vertex` - 要检查的顶点索引
    /// * `color` - 要检查的颜色值
    ///
    /// # 返回值
    /// * `true` - 颜色有效（与相邻顶点颜色不同）
    /// * `false` - 颜色无效（与某个相邻顶点颜色相同）
    fn is_color_valid(&self, vertex: usize, color: usize) -> bool {
        // 遍历所有顶点，检查是否有相邻顶点已经使用了相同的颜色
        for neighbor in 0..self.num_vertices() {
            if (self.adjacency_matrix[vertex][neighbor] || self.adjacency_matrix[neighbor][vertex])
                && self.vertex_colors[neighbor] == color {
                return false;
            }
        }
        true
    }

    /// 使用回溯算法递归查找所有有效的图着色方案
    ///
    /// # 参数
    /// * `vertex` - 当前处理的顶点索引
    /// * `num_colors` - 可用的颜色数量
    fn find_colorings(&mut self, vertex: usize, num_colors: usize) {
        // 如果所有顶点都已着色，保存当前解
        if vertex == self.num_vertices() {
            self.solutions.push(self.vertex_colors.clone());
            return;
        }
        // 尝试为当前顶点分配每种可能的颜色
        for color in 0..num_colors {
            // 检查当前颜色是否有效
            if self.is_color_valid(vertex, color) {
                // 分配颜色并递归处理下一个顶点
                self.vertex_colors[vertex] = color;
                self.find_colorings(vertex + 1, num_colors);
                // 回溯：恢复顶点颜色为未分配状态
                self.vertex_colors[vertex] = usize::MAX;
            }
        }
    }

    /// 查找图的所有有效着色方案
    ///
    /// # 参数
    /// * `num_colors` - 可用的颜色数量
    ///
    /// # 返回值
    /// * `Some(Vec<Vec<usize>>)` - 存在有效着色方案，返回所有解
    /// * `None` - 不存在有效着色方案
    fn find_solutions(&mut self, num_colors: usize) -> Option<Vec<Vec<usize>>> {
        // 开始查找着色方案
        self.find_colorings(0, num_colors);
        // 根据是否找到解返回相应结果
        if self.solutions.is_empty() {
            None
        } else {
            Some(std::mem::take(&mut self.solutions))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! test_graph_coloring {
        ($($name:ident: $test_case:expr,)*) => {
            $(
                #[test]
                fn $name() {
                    let (adjacency_matrix, num_colors, expected) = $test_case;
                    let actual = generate_colorings(adjacency_matrix, num_colors);
                    assert_eq!(actual, expected);
                }
            )*
        };
    }

    test_graph_coloring! {
        test_complete_graph_with_3_colors: (
            vec![
                vec![false, true, true, true],
                vec![true, false, true, false],
                vec![true, true, false, true],
                vec![true, false, true, false],
            ],
            3,
            Ok(Some(vec![
                vec![0, 1, 2, 1],
                vec![0, 2, 1, 2],
                vec![1, 0, 2, 0],
                vec![1, 2, 0, 2],
                vec![2, 0, 1, 0],
                vec![2, 1, 0, 1],
            ]))
        ),
        test_linear_graph_with_2_colors: (
            vec![
                vec![false, true, false, false],
                vec![true, false, true, false],
                vec![false, true, false, true],
                vec![false, false, true, false],
            ],
            2,
            Ok(Some(vec![
                vec![0, 1, 0, 1],
                vec![1, 0, 1, 0],
            ]))
        ),
        test_incomplete_graph_with_insufficient_colors: (
            vec![
                vec![false, true, true],
                vec![true, false, true],
                vec![true, true, false],
            ],
            1,
            Ok(None::<Vec<Vec<usize>>>)
        ),
        test_empty_graph: (
            vec![],
            1,
            Err(GraphColoringError::EmptyAdjacencyMatrix)
        ),
        test_non_square_matrix: (
            vec![
                vec![false, true, true],
                vec![true, false, true],
            ],
            3,
            Err(GraphColoringError::ImproperAdjacencyMatrix)
        ),
        test_single_vertex_graph: (
            vec![
                vec![false],
            ],
            1,
            Ok(Some(vec![
                vec![0],
            ]))
        ),
        test_bipartite_graph_with_2_colors: (
            vec![
                vec![false, true, false, true],
                vec![true, false, true, false],
                vec![false, true, false, true],
                vec![true, false, true, false],
            ],
            2,
            Ok(Some(vec![
                vec![0, 1, 0, 1],
                vec![1, 0, 1, 0],
            ]))
        ),
        test_large_graph_with_3_colors: (
            vec![
                vec![false, true, true, false, true, true, false, true, true, false],
                vec![true, false, true, true, false, true, true, false, true, true],
                vec![true, true, false, true, true, false, true, true, false, true],
                vec![false, true, true, false, true, true, false, true, true, false],
                vec![true, false, true, true, false, true, true, false, true, true],
                vec![true, true, false, true, true, false, true, true, false, true],
                vec![false, true, true, false, true, true, false, true, true, false],
                vec![true, false, true, true, false, true, true, false, true, true],
                vec![true, true, false, true, true, false, true, true, false, true],
                vec![false, true, true, false, true, true, false, true, true, false],
            ],
            3,
            Ok(Some(vec![
                vec![0, 1, 2, 0, 1, 2, 0, 1, 2, 0],
                vec![0, 2, 1, 0, 2, 1, 0, 2, 1, 0],
                vec![1, 0, 2, 1, 0, 2, 1, 0, 2, 1],
                vec![1, 2, 0, 1, 2, 0, 1, 2, 0, 1],
                vec![2, 0, 1, 2, 0, 1, 2, 0, 1, 2],
                vec![2, 1, 0, 2, 1, 0, 2, 1, 0, 2],
            ]))
        ),
        test_disconnected_graph: (
            vec![
                vec![false, false, false],
                vec![false, false, false],
                vec![false, false, false],
            ],
            2,
            Ok(Some(vec![
                vec![0, 0, 0],
                vec![0, 0, 1],
                vec![0, 1, 0],
                vec![0, 1, 1],
                vec![1, 0, 0],
                vec![1, 0, 1],
                vec![1, 1, 0],
                vec![1, 1, 1],
            ]))
        ),
        test_no_valid_coloring: (
            vec![
                vec![false, true, true],
                vec![true, false, true],
                vec![true, true, false],
            ],
            2,
            Ok(None::<Vec<Vec<usize>>>)
        ),
        test_more_colors_than_nodes: (
            vec![
                vec![true, true],
                vec![true, true],
            ],
            3,
            Ok(Some(vec![
                vec![0, 1],
                vec![0, 2],
                vec![1, 0],
                vec![1, 2],
                vec![2, 0],
                vec![2, 1],
            ]))
        ),
        test_no_coloring_with_zero_colors: (
            vec![
                vec![true],
            ],
            0,
            Ok(None::<Vec<Vec<usize>>>)
        ),
        test_complete_graph_with_3_vertices_and_3_colors: (
            vec![
                vec![false, true, true],
                vec![true, false, true],
                vec![true, true, false],
            ],
            3,
            Ok(Some(vec![
                vec![0, 1, 2],
                vec![0, 2, 1],
                vec![1, 0, 2],
                vec![1, 2, 0],
                vec![2, 0, 1],
                vec![2, 1, 0],
            ]))
        ),
        test_directed_graph_with_3_colors: (
            vec![
                vec![false, true, false, true],
                vec![false, false, true, false],
                vec![true, false, false, true],
                vec![true, false, false, false],
            ],
            3,
            Ok(Some(vec![
                vec![0, 1, 2, 1],
                vec![0, 2, 1, 2],
                vec![1, 0, 2, 0],
                vec![1, 2, 0, 2],
                vec![2, 0, 1, 0],
                vec![2, 1, 0, 1],
            ]))
        ),
        test_directed_graph_no_valid_coloring: (
            vec![
                vec![false, true, false, true],
                vec![false, false, true, true],
                vec![true, false, false, true],
                vec![true, false, false, false],
            ],
            3,
            Ok(None::<Vec<Vec<usize>>>)
        ),
        test_large_directed_graph_with_3_colors: (
            vec![
                vec![false, true, false, false, true, false, false, true, false, false],
                vec![false, false, true, false, false, true, false, false, true, false],
                vec![false, false, false, true, false, false, true, false, false, true],
                vec![true, false, false, false, true, false, false, true, false, false],
                vec![false, true, false, false, false, true, false, false, true, false],
                vec![false, false, true, false, false, false, true, false, false, true],
                vec![true, false, false, false, true, false, false, true, false, false],
                vec![false, true, false, false, false, true, false, false, true, false],
                vec![false, false, true, false, false, false, true, false, false, true],
                vec![true, false, false, false, true, false, false, true, false, false],
            ],
            3,
            Ok(Some(vec![
                vec![0, 1, 2, 1, 2, 0, 1, 2, 0, 1],
                vec![0, 2, 1, 2, 1, 0, 2, 1, 0, 2],
                vec![1, 0, 2, 0, 2, 1, 0, 2, 1, 0],
                vec![1, 2, 0, 2, 0, 1, 2, 0, 1, 2],
                vec![2, 0, 1, 0, 1, 2, 0, 1, 2, 0],
                vec![2, 1, 0, 1, 0, 2, 1, 0, 2, 1]
            ]))
        ),
    }
}
