/// 表示后缀树中的一个节点。
///
/// 每个节点包含一个字符串片段（sub）和指向其子节点的索引列表（ch）。
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Node {
    pub sub: String,
    pub ch: Vec<usize>,
}

impl Node {
    /// 创建一个新的节点。
    ///
    /// # 参数
    /// * `sub` - 节点存储的字符串片段。
    /// * `children` - 子节点索引的向量。
    ///
    /// # 返回值
    /// 返回一个新的 Node 实例。
    fn new(sub: String, children: Vec<usize>) -> Self {
        Node {
            sub,
            ch: children.to_vec(),
        }
    }

    /// 创建一个空节点。
    ///
    /// # 返回值
    /// 返回一个 sub 为空字符串且 ch 为空向量的 Node 实例。
    pub fn empty() -> Self {
        Node {
            sub: "".to_string(),
            ch: vec![],
        }
    }
}

/// 表示后缀树数据结构。
///
/// 后缀树是一种压缩字典树，用于高效地存储和检索字符串的所有后缀。
pub struct SuffixTree {
    pub nodes: Vec<Node>,
}

impl SuffixTree {
    /// 创建一个新的后缀树。
    ///
    /// # 参数
    /// * `s` - 用于构建后缀树的输入字符串。
    ///
    /// # 返回值
    /// 返回一个包含输入字符串所有后缀的 SuffixTree 实例。
    pub fn new(s: &str) -> Self {
        let mut suf_tree = SuffixTree {
            nodes: vec![Node::empty()],
        };
        // 遍历输入字符串的每个后缀并添加到树中
        for idx in 0..s.len() {
            let (_, substr) = s.split_at(idx);
            suf_tree.add_suffix(substr);
        }
        suf_tree
    }

    /// 向后缀树中添加一个后缀。
    ///
    /// # 参数
    /// * `suf` - 要添加的后缀字符串。
    fn add_suffix(&mut self, suf: &str) {
        let mut n = 0;  // 当前节点索引
        let mut idx = 0;  // 当前后缀处理位置

        // 遍历后缀中的每个字符
        while idx < suf.len() {
            let b = suf.chars().nth(idx);  // 获取当前字符
            let mut x2 = 0;  // 子节点索引
            let mut n2: usize;  // 下一个节点索引

            // 查找匹配的子节点
            loop {
                let children = &self.nodes[n].ch;
                // 如果没有更多子节点，创建新节点
                if children.len() == x2 {
                    n2 = self.nodes.len();
                    self.nodes.push(Node::new(
                        {
                            let (_, sub) = suf.split_at(idx);
                            sub.to_string()
                        },
                        vec![],
                    ));
                    self.nodes[n].ch.push(n2);
                    return;
                }
                n2 = children[x2];
                // 找到匹配的子节点则跳出循环
                if self.nodes[n2].sub.chars().next() == b {
                    break;
                }
                x2 += 1;
            }

            // 检查是否需要分割节点
            let sub2 = self.nodes[n2].sub.clone();
            let mut j = 0;
            while j < sub2.len() {
                // 如果字符不匹配，需要分割节点
                if suf.chars().nth(idx + j) != sub2.chars().nth(j) {
                    let n3 = n2;
                    n2 = self.nodes.len();
                    // 创建新的父节点
                    self.nodes.push(Node::new(
                        {
                            let (sub, _) = sub2.split_at(j);
                            sub.to_string()
                        },
                        vec![n3],
                    ));
                    // 更新原节点的字符串和父节点的子节点列表
                    let (_, temp_sub) = sub2.split_at(j);
                    self.nodes[n3].sub = temp_sub.to_string();
                    self.nodes[n].ch[x2] = n2;
                    break;
                }
                j += 1;
            }

            idx += 1;
            n = n2;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_suffix_tree() {
        let suf_tree = SuffixTree::new("banana$");
        assert_eq!(
            suf_tree.nodes,
            vec![
                Node {
                    sub: "".to_string(),
                    ch: vec![1, 8, 6, 10]
                },
                Node {
                    sub: "banana$".to_string(),
                    ch: vec![]
                },
                Node {
                    sub: "na$".to_string(),
                    ch: vec![]
                },
                Node {
                    sub: "na$".to_string(),
                    ch: vec![]
                },
                Node {
                    sub: "na".to_string(),
                    ch: vec![2, 5]
                },
                Node {
                    sub: "$".to_string(),
                    ch: vec![]
                },
                Node {
                    sub: "na".to_string(),
                    ch: vec![3, 7]
                },
                Node {
                    sub: "$".to_string(),
                    ch: vec![]
                },
                Node {
                    sub: "a".to_string(),
                    ch: vec![4, 9]
                },
                Node {
                    sub: "$".to_string(),
                    ch: vec![]
                },
                Node {
                    sub: "$".to_string(),
                    ch: vec![]
                }
            ]
        );
    }
}
