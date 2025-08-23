use std::fmt;
use std::collections::{HashMap, HashSet};
use std::fmt::Formatter;

/// 表示图中节点不存在的错误类型
#[allow(unused)]
#[derive(Debug, Clone)]
pub struct NodeNotInGraph;

/// 实现 Display trait，用于格式化输出 NodeNotInGraph 错误信息
impl fmt::Display for NodeNotInGraph {
    /// 格式化 NodeNotInGraph 错误信息
    ///
    /// # 参数
    /// * `f` - 格式化器引用
    ///
    /// # 返回值
    /// 返回格式化结果
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "accessing a node that is not in the graph")
    }
}

/// 有向图结构体，使用邻接表存储图结构
///
/// # 字段
/// * `adjacency_table` - 邻接表，键为节点名称，值为相邻节点列表及其权重
pub struct DirectedGraph {
    adjacency_table: HashMap<String, Vec<(String, i32)>>,  // (node, weight)
}

/// 为 DirectedGraph 实现 Graph trait
impl Graph for DirectedGraph {
    /// 创建一个新的空有向图
    ///
    /// # 返回值
    /// 返回一个新的 DirectedGraph 实例
    fn new() -> DirectedGraph {
        DirectedGraph {
            adjacency_table: HashMap::new(),
        }
    }

    /// 获取可变的邻接表引用
    ///
    /// # 返回值
    /// 返回邻接表的可变引用
    fn adjacency_table_mutable(&mut self) -> &mut HashMap<String, Vec<(String, i32)>> {
        &mut self.adjacency_table
    }

    /// 获取邻接表引用
    ///
    /// # 返回值
    /// 返回邻接表的不可变引用
    fn adjacency_table(&self) -> &HashMap<String, Vec<(String, i32)>> {
        &self.adjacency_table
    }
}

/// 无向图结构体，使用邻接表存储图结构
///
/// # 字段
/// * `adjacency_table` - 邻接表，键为节点名称，值为相邻节点列表及其权重
pub struct UndirectedGraph {
    adjacency_table: HashMap<String, Vec<(String, i32)>>,
}

/// 为 UndirectedGraph 实现 Graph trait
impl Graph for UndirectedGraph {
    /// 创建一个新的空无向图
    ///
    /// # 返回值
    /// 返回一个新的 UndirectedGraph 实例
    fn new() -> Self {
        Self {
            adjacency_table: HashMap::new(),
        }
    }

    /// 获取可变的邻接表引用
    ///
    /// # 返回值
    /// 返回邻接表的可变引用
    fn adjacency_table_mutable(&mut self) -> &mut HashMap<String, Vec<(String, i32)>> {
        &mut self.adjacency_table
    }

    /// 获取邻接表引用
    ///
    /// # 返回值
    /// 返回邻接表的不可变引用
    fn adjacency_table(&self) -> &HashMap<String, Vec<(String, i32)>> {
        &self.adjacency_table
    }

    /// 添加一条边到无向图中
    ///
    /// # 参数
    /// * `edge` - 边的元组，包含起始节点、结束节点和权重
    fn add_edge(&mut self, edge: (&str, &str, i32)) {
        // 确保边的两个节点都存在于图中
        self.add_node(edge.0);
        self.add_node(edge.1);

        // 在起始节点的邻接表中添加结束节点
        self.adjacency_table
            .entry(edge.0.to_string())
            .and_modify(|e| {
                e.push((edge.1.to_string(), edge.2));
            });

        // 在结束节点的邻接表中添加起始节点（无向图的特性）
        self.adjacency_table
            .entry(edge.1.to_string())
            .and_modify(|e| {
                e.push((edge.0.to_string(), edge.2));
            });
    }
}

/// 图的 trait，定义了图的基本操作
pub trait Graph {
    /// 创建一个新的图实例
    ///
    /// # 返回值
    /// 返回一个新的图实例
    fn new() -> Self;

    /// 获取可变的邻接表引用
    ///
    /// # 返回值
    /// 返回邻接表的可变引用
    fn adjacency_table_mutable(&mut self) -> &mut HashMap<String, Vec<(String, i32)>>;

    /// 获取邻接表引用
    ///
    /// # 返回值
    /// 返回邻接表的不可变引用
    fn adjacency_table(&self) -> &HashMap<String, Vec<(String, i32)>>;

    /// 向图中添加一个节点
    ///
    /// # 参数
    /// * `node` - 要添加的节点名称
    ///
    /// # 返回值
    /// 如果节点已存在返回 false，否则添加节点并返回 true
    fn add_node(&mut self, node: &str) -> bool {
        match self.adjacency_table().get(node) {
            None => {
                self.adjacency_table_mutable().insert((*node).to_string(), Vec::new());
                true
            }
            _ => false,
        }
    }

    /// 向图中添加一条边
    ///
    /// # 参数
    /// * `edge` - 边的元组，包含起始节点、结束节点和权重
    fn add_edge(&mut self, edge: (&str, &str, i32)) {
        // 确保边的两个节点都存在于图中
        self.add_node(edge.0);
        self.add_node(edge.1);

        // 在起始节点的邻接表中添加结束节点和权重
        self.adjacency_table_mutable()
            .entry(edge.0.to_string())
            .and_modify(|e| {
                e.push((edge.1.to_string(), edge.2));
            });
    }

    /// 获取指定节点的所有邻居节点
    ///
    /// # 参数
    /// * `node` - 目标节点名称
    ///
    /// # 返回值
    /// 如果节点存在，返回其邻居节点列表的引用；否则返回 NodeNotInGraph 错误
    fn neighbours(&self, node: &str) -> Result<&Vec<(String, i32)>, NodeNotInGraph> {
        match self.adjacency_table().get(node) {
            None => Err(NodeNotInGraph),
            Some(i) => Ok(i),
        }
    }

    /// 检查图中是否包含指定节点
    ///
    /// # 参数
    /// * `node` - 要检查的节点名称
    ///
    /// # 返回值
    /// 如果节点存在返回 true，否则返回 false
    fn contains(&self, node: &str) -> bool {
        self.adjacency_table().get(node).is_some()
    }

    /// 获取图中所有节点
    ///
    /// # 返回值
    /// 返回包含所有节点名称的 HashSet
    fn nodes(&self) -> HashSet<&String> {
        self.adjacency_table().keys().collect()
    }

    /// 获取图中所有边
    ///
    /// # 返回值
    /// 返回包含所有边的向量，每条边表示为 (起始节点, 结束节点, 权重)
    fn edges(&self) -> Vec<(&String, &String, i32)> {
        let mut edges = Vec::new();

        // 遍历邻接表中的每个节点及其邻居
        for (from_node, from_node_neighbours) in self.adjacency_table() {
            for (to_node, weight) in from_node_neighbours {
                edges.push((from_node, to_node, *weight));
            }
        }
        edges
    }
}

#[cfg(test)]
mod test_undirected_graph {
    use super::Graph;
    use super::UndirectedGraph;

    #[test]
    fn test_add_edge() {
        let mut graph = UndirectedGraph::new();

        graph.add_edge(("a", "b", 5));
        graph.add_edge(("b", "c", 10));
        graph.add_edge(("c", "a", 7));

        let expected_edges = [
            (&String::from("a"), &String::from("b"), 5),
            (&String::from("b"), &String::from("a"), 5),
            (&String::from("c"), &String::from("a"), 7),
            (&String::from("a"), &String::from("c"), 7),
            (&String::from("b"), &String::from("c"), 10),
            (&String::from("c"), &String::from("b"), 10),
        ];
        for edge in expected_edges.iter() {
            assert!(graph.edges().contains(edge));
        }
    }

    #[test]
    fn test_neighbours() {
        let mut graph = UndirectedGraph::new();

        graph.add_edge(("a", "b", 5));
        graph.add_edge(("b", "c", 10));
        graph.add_edge(("c", "a", 7));

        assert_eq!(
            graph.neighbours("a").unwrap(),
            &vec![(String::from("b"), 5), (String::from("c"), 7)]
        );
    }
}

#[cfg(test)]
mod test_directed_graph {
    use super::DirectedGraph;
    use super::Graph;

    #[test]
    fn test_add_node() {
        let mut graph = DirectedGraph::new();
        graph.add_node("a");
        graph.add_node("b");
        graph.add_node("c");
        assert_eq!(
            graph.nodes(),
            [&String::from("a"), &String::from("b"), &String::from("c")]
                .iter()
                .cloned()
                .collect()
        );
    }

    #[test]
    fn test_add_edge() {
        let mut graph = DirectedGraph::new();

        graph.add_edge(("a", "b", 5));
        graph.add_edge(("c", "a", 7));
        graph.add_edge(("b", "c", 10));

        let expected_edges = [
            (&String::from("a"), &String::from("b"), 5),
            (&String::from("c"), &String::from("a"), 7),
            (&String::from("b"), &String::from("c"), 10),
        ];
        for edge in expected_edges.iter() {
            assert!(graph.edges().contains(edge));
        }
    }

    #[test]
    fn test_neighbours() {
        let mut graph = DirectedGraph::new();

        graph.add_edge(("a", "b", 5));
        graph.add_edge(("b", "c", 10));
        graph.add_edge(("c", "a", 7));

        assert_eq!(
            graph.neighbours("a").unwrap(),
            &vec![(String::from("b"), 5)]
        );
    }

    #[test]
    fn test_contains() {
        let mut graph = DirectedGraph::new();
        graph.add_node("a");
        graph.add_node("b");
        graph.add_node("c");
        assert!(graph.contains("a"));
        assert!(graph.contains("b"));
        assert!(graph.contains("c"));
        assert!(!graph.contains("d"));
    }
}
