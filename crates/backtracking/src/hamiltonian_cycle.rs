/// 错误类型，用于表示查找哈密顿回路过程中可能出现的错误。
#[derive(Debug, PartialEq, Eq)]
pub enum FindHamiltonianCycleError {
    /// 起始顶点索引超出图中顶点范围
    StartOutOfBounds,
    /// 邻接矩阵为空
    EmptyAdjacencyMatrix,
    /// 邻接矩阵不是方阵（行数与列数不一致）
    ImproperAdjacencyMatrix,
}

/// 图结构体，使用邻接矩阵表示图
struct Graph {
    adjacency_matrix: Vec<Vec<bool>>,
}

impl Graph {
    /// 创建一个新的图实例
    ///
    /// # 参数
    ///
    /// * `adjacency_matrix` - 表示图的邻接矩阵，其中 `true` 表示两个顶点之间有边
    ///
    /// # 返回值
    ///
    /// 成功时返回 `Graph` 实例，失败时返回相应的错误：
    /// - `EmptyAdjacencyMatrix`: 邻接矩阵为空
    /// - `ImproperAdjacencyMatrix`: 邻接矩阵不是方阵
    fn new(adjacency_matrix: Vec<Vec<bool>>) -> Result<Self, FindHamiltonianCycleError> {
        if adjacency_matrix.is_empty() {
            return Err(FindHamiltonianCycleError::EmptyAdjacencyMatrix);
        }
        if adjacency_matrix
            .iter()
            .any(|row| row.len() != adjacency_matrix.len()) {
            return Err(FindHamiltonianCycleError::ImproperAdjacencyMatrix);
        }
        Ok(Self { adjacency_matrix })
    }

    /// 获取图中的顶点数量
    fn num_vertices(&self) -> usize {
        self.adjacency_matrix.len()
    }

    /// 判断当前顶点是否可以安全地加入路径中
    ///
    /// # 参数
    ///
    /// * `v` - 当前要检查的顶点
    /// * `visited` - 标记顶点是否已被访问的布尔数组
    /// * `path` - 当前构建的路径
    /// * `pos` - 当前路径的位置
    ///
    /// # 返回值
    ///
    /// 如果顶点可以加入路径则返回 `true`，否则返回 `false`
    fn is_safe(&self, v: usize, visited: &[bool], path: &[Option<usize>], pos: usize) -> bool {
        // 检查当前顶点是否与路径中上一个顶点相连
        if !self.adjacency_matrix[path[pos - 1].unwrap()][v] {
            return false;
        }
        // 检查当前顶点是否已经被访问过
        !visited[v]
    }

    /// 递归辅助函数，用于寻找哈密顿回路
    ///
    /// # 参数
    ///
    /// * `path` - 当前构建的路径
    /// * `visited` - 标记顶点是否已被访问的布尔数组
    /// * `pos` - 当前路径的位置
    ///
    /// # 返回值
    ///
    /// 如果找到哈密顿回路则返回 `true`，否则返回 `false`
    fn hamiltonian_cycle_util(
        &self,
        path: &mut [Option<usize>],
        visited: &mut [bool],
        pos: usize,
    ) -> bool {
        // 如果路径已经包含所有顶点，则检查最后一个顶点是否与起始顶点相连形成回路
        if pos == self.num_vertices() {
            return self.adjacency_matrix[path[pos - 1].unwrap()][path[0].unwrap()];
        }
        // 尝试将每个未访问的顶点加入路径
        for v in 0..self.num_vertices() {
            if self.is_safe(v, visited, path, pos) {
                path[pos] = Some(v);
                visited[v] = true;
                if self.hamiltonian_cycle_util(path, visited, pos + 1) {
                    return true;
                }
                // 回溯：移除当前顶点并标记为未访问
                path[pos] = None;
                visited[v] = false;
            }
        }
        false
    }

    /// 查找从指定起始顶点开始的哈密顿回路
    ///
    /// # 参数
    ///
    /// * `start_vertex` - 起始顶点的索引
    ///
    /// # 返回值
    ///
    /// 成功时返回：
    /// - `Ok(Some(Vec<usize>))`: 找到哈密顿回路，返回路径
    /// - `Ok(None)`: 未找到哈密顿回路
    /// 失败时返回相应的错误：
    /// - `StartOutOfBounds`: 起始顶点索引超出范围
    fn find_hamiltonian_cycle(
        &self,
        start_vertex: usize,
    ) -> Result<Option<Vec<usize>>, FindHamiltonianCycleError> {
        if start_vertex >= self.num_vertices() {
            return Err(FindHamiltonianCycleError::StartOutOfBounds);
        }
        let mut path = vec![None; self.num_vertices()];
        path[0] = Some(start_vertex);
        let mut visited = vec![false; self.num_vertices()];
        visited[start_vertex] = true;
        if self.hamiltonian_cycle_util(&mut path, &mut visited, 1) {
            // 在路径末尾添加起始顶点以形成完整的回路
            path.push(Some(start_vertex));
            Ok(Some(path.into_iter().map(Option::unwrap).collect()))
        } else {
            Ok(None)
        }
    }
}

/// 公共接口函数，用于查找给定图中从指定起始顶点开始的哈密顿回路
///
/// # 参数
///
/// * `adjacency_matrix` - 表示图的邻接矩阵
/// * `start_vertex` - 起始顶点的索引
///
/// # 返回值
///
/// 成功时返回：
/// - `Ok(Some(Vec<usize>))`: 找到哈密顿回路，返回路径
/// - `Ok(None)`: 未找到哈密顿回路
/// 失败时返回相应的错误：
/// - `EmptyAdjacencyMatrix`: 邻接矩阵为空
/// - `ImproperAdjacencyMatrix`: 邻接矩阵不是方阵
/// - `StartOutOfBounds`: 起始顶点索引超出范围
#[allow(unused)]
pub fn find_hamiltonian_cycle(
    adjacency_matrix: Vec<Vec<bool>>,
    start_vertex: usize,
) -> Result<Option<Vec<usize>>, FindHamiltonianCycleError> {
    Graph::new(adjacency_matrix)?.find_hamiltonian_cycle(start_vertex)
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! hamiltonian_cycle_tests {
        ($($name:ident: $test_case:expr,)*) => {
            $(
                #[test]
                fn $name() {
                    let (adjacency_matrix, start_vertex, expected) = $test_case;
                    let result = find_hamiltonian_cycle(adjacency_matrix, start_vertex);
                    assert_eq!(result, expected);
                }
            )*
        };
    }

    hamiltonian_cycle_tests! {
        test_complete_graph: (
            vec![
                vec![false, true, true, true],
                vec![true, false, true, true],
                vec![true, true, false, true],
                vec![true, true, true, false],
            ],
            0,
            Ok(Some(vec![0, 1, 2, 3, 0]))
        ),
        test_directed_graph_with_cycle: (
            vec![
                vec![false, true, false, false, false],
                vec![false, false, true, true, false],
                vec![true, false, false, true, true],
                vec![false, false, true, false, true],
                vec![true, true, false, false, false],
            ],
            2,
            Ok(Some(vec![2, 3, 4, 0, 1, 2]))
        ),
        test_undirected_graph_with_cycle: (
            vec![
                vec![false, true, false, false, true],
                vec![true, false, true, false, false],
                vec![false, true, false, true, false],
                vec![false, false, true, false, true],
                vec![true, false, false, true, false],
            ],
            2,
            Ok(Some(vec![2, 1, 0, 4, 3, 2]))
        ),
        test_directed_graph_no_cycle: (
            vec![
                vec![false, true, false, true, false],
                vec![false, false, true, true, false],
                vec![false, false, false, true, false],
                vec![false, false, false, false, true],
                vec![false, false, true, false, false],
            ],
            0,
            Ok(None::<Vec<usize>>)
        ),
        test_undirected_graph_no_cycle: (
            vec![
                vec![false, true, false, false, false],
                vec![true, false, true, true, false],
                vec![false, true, false, true, true],
                vec![false, true, true, false, true],
                vec![false, false, true, true, false],
            ],
            0,
            Ok(None::<Vec<usize>>)
        ),
        test_triangle_graph: (
            vec![
                vec![false, true, false],
                vec![false, false, true],
                vec![true, false, false],
            ],
            1,
            Ok(Some(vec![1, 2, 0, 1]))
        ),
        test_tree_graph: (
            vec![
                vec![false, true, false, true, false],
                vec![true, false, true, true, false],
                vec![false, true, false, false, false],
                vec![true, true, false, false, true],
                vec![false, false, false, true, false],
            ],
            0,
            Ok(None::<Vec<usize>>)
        ),
        test_empty_graph: (
            vec![],
            0,
            Err(FindHamiltonianCycleError::EmptyAdjacencyMatrix)
        ),
        test_improper_graph: (
            vec![
                vec![false, true],
                vec![true],
                vec![false, true, true],
                vec![true, true, true, false]
            ],
            0,
            Err(FindHamiltonianCycleError::ImproperAdjacencyMatrix)
        ),
        test_start_out_of_bound: (
            vec![
                vec![false, true, true],
                vec![true, false, true],
                vec![true, true, false],
            ],
            3,
            Err(FindHamiltonianCycleError::StartOutOfBounds)
        ),
        test_complex_directed_graph: (
            vec![
                vec![false, true, false, true, false, false],
                vec![false, false, true, false, true, false],
                vec![false, false, false, true, false, false],
                vec![false, true, false, false, true, false],
                vec![false, false, true, false, false, true],
                vec![true, false, false, false, false, false],
            ],
            0,
            Ok(Some(vec![0, 1, 2, 3, 4, 5, 0]))
        ),
        single_node_self_loop: (
            vec![
                vec![true],
            ],
            0,
            Ok(Some(vec![0, 0]))
        ),
        single_node: (
            vec![
                vec![false],
            ],
            0,
            Ok(None)
        ),
    }
}
