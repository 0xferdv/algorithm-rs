use std::mem;
use std::fmt::Debug;
use std::convert::TryFrom;


struct Node<T> {
    keys: Vec<T>,
    children: Vec<Node<T>>,
}

/// B树数据结构，用于高效存储和检索有序数据
///
/// # 类型参数
/// * `T` - 树中存储的元素类型，必须实现Ord trait以支持排序
pub struct BTree<T> {
    /// 树的根节点
    root: Node<T>,
    /// B树的属性配置
    props: BTreeProps,
}

/// B树属性配置结构体
///
/// 包含B树的核心参数，如度数、最大键数等
struct BTreeProps {
    /// B树的度数，决定了每个节点的最大子节点数
    degree: usize,
    /// 每个节点允许的最大键数
    max_keys: usize,
    /// 中间键的索引位置，用于节点分裂操作
    mid_key_index: usize,
}

impl<T> Node<T>
where
    T: Ord
{
    /// 创建一个新的节点
    ///
    /// # 参数
    /// * `degree` - 节点的度数
    /// * `_keys` - 可选的键向量，如果为None则创建空向量
    /// * `_children` - 可选的子节点向量，如果为None则创建空向量
    ///
    /// # 返回值
    /// 返回新创建的Node实例
    fn new(degree: usize, _keys: Option<Vec<T>>, _children: Option<Vec<Node<T>>>) -> Self {
        Node {
            keys: match _keys {
                Some(_keys) => _keys,
                None => Vec::with_capacity(degree - 1),
            },
            children: match _children {
                Some(_children) => _children,
                None => Vec::with_capacity(degree),
            },
        }
    }

    /// 判断当前节点是否为叶节点
    ///
    /// # 返回值
    /// 如果节点没有子节点则返回true，否则返回false
    fn is_leaf(&self) -> bool {
        self.children.is_empty()
    }
}

impl BTreeProps {

    /// 创建新的B树属性配置
    ///
    /// # 参数
    /// * `degree` - B树的度数
    ///
    /// # 返回值
    /// 返回配置好的BTreeProps实例
    fn new(degree: usize) -> Self {
        Self {
            degree,
            max_keys: degree - 1,
            mid_key_index: (degree - 1) / 2,
        }
    }

    /// 检查节点是否已达到最大键数
    ///
    /// # 参数
    /// * `node` - 要检查的节点
    ///
    /// # 返回值
    /// 如果节点的键数等于最大键数则返回true，否则返回false
    fn is_maxed_out<T: Ord + Copy>(&self, node: &Node<T>) -> bool {
        node.keys.len() == self.max_keys
    }

    /// 分裂子节点
    ///
    /// 当子节点的键数达到最大值时，将其分裂成两个节点
    ///
    /// # 参数
    /// * `parent` - 父节点的可变引用
    /// * `child_index` - 需要分裂的子节点在父节点中的索引
    #[allow(unused_variables)]
    fn split_child<T: Ord + Copy + Default>(&self, parent: &mut Node<T>, child_index: usize) {
        let child = &mut parent.children[child_index];
        let middle_key = child.keys[self.mid_key_index];
        let right_keys = match child.keys.split_off(self.mid_key_index).split_first() {
            Some((_first, _others)) => {
                _others.to_vec()
            }
            node => Vec::with_capacity(self.max_keys),
        };
        let right_children = if !child.is_leaf() {
            Some(child.children.split_off(self.mid_key_index + 1))
        } else {
            None
        };
        let new_child_node: Node<T> = Node::new(self.degree, Some(right_keys), right_children);
        parent.keys.insert(child_index, middle_key);
        parent.children.insert(child_index + 1, new_child_node);
    }

    /// 在非满节点中插入键
    ///
    /// 递归地在B树中找到合适的位置插入键值
    ///
    /// # 参数
    /// * `node` - 当前处理的节点的可变引用
    /// * `key` - 要插入的键值
    fn insert_non_full<T: Ord + Copy + Default>(&mut self, node: &mut Node<T>, key: T) {
        // 从节点的最后一个键开始向前查找插入位置
        let mut index: isize = isize::try_from(node.keys.len()).ok().unwrap() - 1;
        while index >= 0 && node.keys[index as usize] >= key {
            index -= 1;
        }
        let mut u_index: usize = usize::try_from(index + 1).ok().unwrap();

        // 如果是叶节点，直接插入键值
        if node.is_leaf() {
            node.keys.insert(u_index, key);
        } else {
            // 如果不是叶节点，需要递归处理子节点
            if self.is_maxed_out(&node.children[u_index]) {
                self.split_child(node, u_index);
                if node.keys[u_index] < key {
                    u_index += 1;
                }
            }
            self.insert_non_full(&mut node.children[u_index], key);
        }
    }

    /// 遍历并打印节点信息
    ///
    /// 以层次化的方式遍历B树并打印节点内容
    ///
    /// # 参数
    /// * `node` - 当前处理的节点
    /// * `depth` - 当前节点的深度，用于格式化输出
    fn traverse_node<T: Ord + Debug>(node: &Node<T>, depth: usize) {
        if node.is_leaf() {
            // 叶节点：打印键值
            print!(" {0:{<1$}{2:?}{0:}<1$} ", "", depth, node.keys);
            println!("")
        } else {
            // 非叶节点：递归遍历子节点和键值
            let _depth = depth + 1;
            for (index, key) in node.keys.iter().enumerate() {
                Self::traverse_node(&node.children[index], _depth);
                print!("{0:{<1$}{2:?}{0:}<1$}", "", depth, key);
            }
            Self::traverse_node(node.children.last().unwrap(), _depth);
        }
    }
}

impl<T> BTree<T>
where
    T: Ord + Copy + Debug + Default
{
    /// 创建新的B树实例
    ///
    /// # 参数
    /// * `branch_factor` - 分支因子，决定了B树的度数(度数 = 2 * 分支因子)
    ///
    /// # 返回值
    /// 返回新创建的BTree实例
    pub fn new(branch_factor: usize) -> Self {
        let degree = 2 * branch_factor;
        Self {
            root: Node::new(degree, None, None),
            props: BTreeProps::new(degree),
        }
    }

    /// 向B树中插入键值
    ///
    /// # 参数
    /// * `key` - 要插入的键值
    pub fn insert(&mut self, key: T) {
        // 如果根节点已满，需要创建新根节点并分裂原根节点
        if self.props.is_maxed_out(&self.root) {
            let mut new_root = Node::new(self.props.degree, None, None);
            mem::swap(&mut new_root, &mut self.root);
            self.root.children.insert(0, new_root);
            self.props.split_child(&mut self.root, 0);
        }
        self.props.insert_non_full(&mut self.root, key);
    }

    /// 遍历并打印整个B树
    ///
    /// 以层次化格式打印B树的所有节点
    pub fn traverse(&self) {
        BTreeProps::traverse_node(&self.root, 0);
        println!();
    }

    /// 在B树中搜索指定键值
    ///
    /// # 参数
    /// * `key` - 要搜索的键值
    ///
    /// # 返回值
    /// 如果找到键值返回true，否则返回false
    pub fn search(&self, key: T) -> bool {
        let mut curr_node = &self.root;
        loop {
            match curr_node.keys.binary_search(&key) {
                Ok(_) => return true,
                Err(index) => {
                    if curr_node.is_leaf() {
                        return false;
                    }
                    curr_node = &curr_node.children[index];
                }
            }
        }
    }
}


#[cfg(test)]
mod test {
    use super::BTree;

    macro_rules! test_search {
        ($($name:ident: $number_of_children:expr,)*) => {
        $(
            #[test]
            fn $name() {
                let mut tree = BTree::new($number_of_children);
                tree.insert(10);
                tree.insert(20);
                tree.insert(30);
                tree.insert(5);
                tree.insert(6);
                tree.insert(7);
                tree.insert(11);
                tree.insert(12);
                tree.insert(15);
                assert!(!tree.search(4));
                assert!(tree.search(5));
                assert!(tree.search(6));
                assert!(tree.search(7));
                assert!(!tree.search(8));
                assert!(!tree.search(9));
                assert!(tree.search(10));
                assert!(tree.search(11));
                assert!(tree.search(12));
                assert!(!tree.search(13));
                assert!(!tree.search(14));
                assert!(tree.search(15));
                assert!(!tree.search(16));
            }
        )*
        }
    }

    test_search! {
        children_2: 2,
        children_3: 3,
        children_4: 4,
        children_5: 5,
        children_10: 10,
        children_60: 60,
        children_101: 101,
    }
}
