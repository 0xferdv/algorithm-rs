/// 二叉树节点结构体，用于表示二叉搜索树中的每个节点。
///
/// # 字段说明
/// * `value` - 节点存储的值
/// * `left` - 左子节点的可选指针
/// * `right` - 右子节点的可选指针
struct TreeNode<T> {
    value: T,
    left: Option<Box<TreeNode<T>>>,
    right: Option<Box<TreeNode<T>>>,
}

impl<T> TreeNode<T> {

    /// 创建一个新的树节点。
    ///
    /// # 参数
    /// * `value` - 要存储在节点中的值
    ///
    /// # 返回值
    /// 返回包含指定值的新节点
    fn new(value: T) -> Self {
        Self {
            value,
            left: None,
            right: None,
        }
    }
}

/// 二叉搜索树结构体，用于实现基于二叉树的排序算法。
///
/// # 字段说明
/// * `root` - 树根节点的可选指针
struct BinarySearchTree<T> {
    root: Option<Box<TreeNode<T>>>,
}


impl<T: Ord + Clone> BinarySearchTree<T> {

    /// 创建一个新的空二叉搜索树。
    ///
    /// # 返回值
    /// 返回一个空的二叉搜索树实例
    fn new() -> Self {
        Self { root: None }
    }

    /// 向二叉搜索树中插入一个新值。
    ///
    /// # 参数
    /// * `value` - 要插入的值
    fn insert(&mut self, value: T) {
        // 使用递归方式插入新值，并更新根节点
        self.root = Some(Self::insert_recursive(self.root.take(), value));
    }

    /// 递归地将值插入到以给定节点为根的子树中。
    ///
    /// # 参数
    /// * `root` - 子树的根节点（可为空）
    /// * `value` - 要插入的值
    ///
    /// # 返回值
    /// 返回插入新值后的子树根节点
    fn insert_recursive(root: Option<Box<TreeNode<T>>>, value: T) -> Box<TreeNode<T>> {
        match root {
            Some(mut node) => {
                // 根据值的大小关系决定插入到左子树还是右子树
                if value <= node.value {
                    node.left = Some(Self::insert_recursive(node.left.take(), value));
                } else {
                    node.right = Some(Self::insert_recursive(node.right.take(), value));
                }
                node
            }
            // 如果当前节点为空，则创建新节点
            None => Box::new(TreeNode::new(value)),
        }
    }

    /// 对二叉搜索树进行中序遍历，将结果存储到给定向量中。
    ///
    /// # 参数
    /// * `result` - 用于存储遍历结果的向量
    fn in_order_traversal(&self, result: &mut Vec<T>) {
        // 调用递归函数执行中序遍历
        Self::in_order_recursive(&self.root, result);
    }

    /// 递归地对以给定节点为根的子树进行中序遍历。
    ///
    /// # 参数
    /// * `root` - 子树的根节点引用
    /// * `result` - 用于存储遍历结果的向量
    fn in_order_recursive(root: &Option<Box<TreeNode<T>>>, result: &mut Vec<T>) {
        // 如果节点存在，则按左-根-右的顺序遍历
        if let Some(node) = root {
            Self::in_order_recursive(&node.left, result);
            result.push(node.value.clone());
            Self::in_order_recursive(&node.right, result);
        }
    }
}

/// 使用二叉搜索树实现的排序函数。
///
/// # 参数
/// * `arr` - 要排序的向量，排序后会被修改为有序状态
#[allow(unused)]
pub fn tree_sort<T: Ord + Clone>(arr: &mut Vec<T>) {
    // 创建二叉搜索树并插入所有元素
    let mut tree = BinarySearchTree::new();
    arr.iter().cloned().for_each(|item| { tree.insert(item) });

    // 执行中序遍历获取有序结果
    let mut result = Vec::new();
    tree.in_order_traversal(&mut result);

    // 将排序结果赋值回原数组
    *arr = result
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_array() {
        let mut arr: Vec<i32> = vec![];
        tree_sort(&mut arr);
        assert_eq!(arr, vec![]);
    }

    #[test]
    fn test_single_element() {
        let mut arr = vec![8];
        tree_sort(&mut arr);
        assert_eq!(arr, vec![8]);
    }

    #[test]
    fn test_already_sorted() {
        let mut arr = vec![1, 2, 3, 4, 5];
        tree_sort(&mut arr);
        assert_eq!(arr, vec![1, 2, 3, 4, 5]);
    }

    #[test]
    fn test_reverse_sorted() {
        let mut arr = vec![5, 4, 3, 2, 1];
        tree_sort(&mut arr);
        assert_eq!(arr, vec![1, 2, 3, 4, 5]);
    }

    #[test]
    fn test_random() {
        let mut arr = vec![9, 6, 10, 11, 2, 19];
        tree_sort(&mut arr);
        assert_eq!(arr, vec![2, 6, 9, 10, 11, 19]);
    }
}
