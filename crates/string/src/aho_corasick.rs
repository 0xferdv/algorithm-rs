use std::cell::RefCell;
use std::rc::{Rc, Weak};
use std::collections::{BTreeMap, VecDeque};


/// AC自动机的节点结构，用于构建字典树和后缀链接
///
/// 该结构体表示AC自动机中的一个节点，包含模式串长度信息、后缀链接和转移边。
#[derive(Default)]
struct ACNode {
    /// 存储以当前节点为结尾的模式串长度列表
    ///
    /// 当一个模式串在字典树中匹配到当前节点时，将该模式串的长度存储在此向量中。
    /// 用于在搜索时确定匹配到的模式串。
    lengths: Vec<usize>,

    /// 指向当前节点的后缀链接（失败指针）
    ///
    /// 后缀链接指向当前节点所代表字符串的最长真后缀所对应的节点。
    /// 在AC自动机中用于匹配失败时的跳转，避免从头开始匹配。
    /// 使用Weak引用避免循环引用导致的内存泄漏。
    suffix: Weak<RefCell<ACNode>>,

    /// 转移边映射表，字符到子节点的映射
    ///
    /// BTreeMap保证了字符的有序性，键为转移字符，值为对应的子节点。
    /// 用于在字典树中根据字符进行状态转移。
    trans: BTreeMap<char, Rc<RefCell<ACNode>>>,
}

/// AC自动机主结构体，用于多模式字符串匹配
///
/// AC自动机是一种用于多模式字符串匹配的算法，可以在一次扫描中找到文本中
/// 所有出现在模式串集合中的子串。
#[derive(Default)]
pub struct AhoCorasick {
    /// 根节点
    ///
    /// AC自动机的根节点，是整个字典树的起点。
    /// 使用Rc<RefCell<>>包装以支持多所有者和内部可变性。
    root: Rc<RefCell<ACNode>>,
}

impl AhoCorasick {

    /// 创建一个新的AC自动机实例
    ///
    /// # 参数
    /// * `words` - 需要匹配的模式串切片
    ///
    /// # 返回值
    /// 返回构建好的AC自动机实例
    ///
    /// # 数据结构说明
    /// 该函数使用Rc和RefCell来管理节点之间的复杂引用关系：
    /// - Rc（Reference Counted）：允许多个所有者共享同一数据，实现节点间的共享引用
    /// - RefCell：提供运行时借用检查，允许在不可变引用的情况下修改内部数据
    ///
    /// 这种组合使得能够构建复杂的树形结构，其中节点可以被多个其他节点引用，
    /// 同时保持内部可变性以在构建过程中修改节点内容。
    pub fn new(words: &[&str]) -> Self {
        // 创建根节点，使用Rc包装以支持共享所有权
        let root = Rc::new(RefCell::new(ACNode::default()));

        // 构建字典树
        words.iter().for_each(|word| {
            // 从根节点开始构建当前单词的路径
            let mut cur = Rc::clone(&root);

            // 遍历单词中的每个字符，构建对应的路径
            for c in word.chars() {
                // 使用entry API获取或创建字符对应的子节点
                // Rc::clone增加引用计数，不复制实际数据
                cur = Rc::clone(Rc::clone(&cur).borrow_mut().trans.entry(c).or_default());
            }

            // 在单词结尾节点记录该单词的长度
            cur.borrow_mut().lengths.push(word.len());
        });

        // 构建后缀链接（失败指针）
        Self::build_suffix(Rc::clone(&root));

        Self { root }
    }

    /// 构建AC自动机的后缀链接（失败指针）
    ///
    /// # 参数
    /// * `root` - AC自动机的根节点
    ///
    /// # 数据结构说明
    /// 此函数使用BFS遍历字典树节点来构建后缀链接：
    /// - VecDeque：作为BFS队列，存储待处理的节点
    /// - Weak引用：用于后缀链接，避免循环引用导致的内存泄漏
    /// - RefCell.borrow()和borrow_mut()：用于安全地访问和修改节点内容
    ///
    /// 在构建后缀链接时，使用Weak引用而不是Rc可以避免节点间的强引用循环，
    /// 确保在AC自动机不再被使用时能够正确释放内存。
    fn build_suffix(root: Rc<RefCell<ACNode>>) {
        // 使用双端队列实现BFS遍历
        let mut q = VecDeque::new();
        q.push_back(Rc::clone(&root));

        // 使用BFS遍历节点，构建后缀链接
        while let Some(parent) = q.pop_front() {
            // 获取父节点的不可变引用
            let parent = parent.borrow();

            // 遍历父节点的所有转移边
            for (c, child) in &parent.trans {
                // 将子节点加入队列以待处理
                q.push_back(Rc::clone(child));

                // 获取子节点的可变引用以修改其后缀链接
                let mut child = child.borrow_mut();

                // 获取父节点的后缀链接指向的节点
                let suffix = parent.suffix.upgrade();

                // 循环查找合适的后缀节点
                loop {
                    match &suffix {
                        None => {
                            // 如果没有后缀节点（即为根节点），则指向根节点
                            // 将根节点的长度信息扩展到当前节点
                            child.lengths.extend(root.borrow().lengths.clone());
                            // 设置后缀链接为指向根节点的Weak引用
                            child.suffix = Rc::downgrade(&root);
                            break;
                        }
                        Some(node) => {
                            // 如果后缀节点有对应的转移边，则建立链接
                            if node.borrow().trans.contains_key(c) {
                                let node = &node.borrow().trans[c];
                                // 将找到节点的长度信息扩展到当前节点
                                child.lengths.extend(node.borrow().lengths.clone());
                                // 设置后缀链接为指向找到节点的Weak引用
                                child.suffix = Rc::downgrade(node);
                                break;
                            }
                        }
                    }
                }
            }
        }
    }

    /// 在文本中搜索所有匹配的模式串
    ///
    /// # 参数
    /// * [s](file://E:\0xferdv\001-rust\algorithm-rs\Rust-master\src\backtracking\mod.rs) - 要搜索的文本字符串
    ///
    /// # 返回值
    /// 返回所有匹配到的模式串切片组成的向量
    ///
    /// # 数据结构说明
    /// 搜索过程中充分利用了Rc和RefCell提供的功能：
    /// - Rc::clone：在状态转移时共享节点所有权
    /// - borrow()：安全地获取节点的只读访问权限
    /// - Weak.upgrade()：在后缀链接跳转时安全地获取目标节点
    /// - UTF-8字符处理：正确处理多字节字符的位置计算
    pub fn search<'a>(&self, s: &'a str) -> Vec<&'a str> {
        let mut ans = vec![];
        // 当前状态节点，初始为根节点
        let mut cur = Rc::clone(&self.root);
        // 当前字符在文本中的位置（字节索引）
        let mut position: usize = 0;

        // 遍历文本中的每个字符
        for c in s.chars() {
            // 沿着转移边或后缀链接查找匹配
            loop {
                // 尝试通过当前字符进行转移
                if let Some(child) = Rc::clone(&cur).borrow().trans.get(&c) {
                    // 转移成功，更新当前状态
                    cur = Rc::clone(child);
                    break;
                }

                // 转移失败，沿着后缀链接跳转
                let suffix = cur.borrow().suffix.clone();
                match suffix.upgrade() {
                    // 成功获取后缀链接指向的节点
                    Some(node) => cur = node,
                    // 到达根节点且无后缀链接，停止跳转
                    None => break,
                }
            }

            // 更新当前位置（考虑UTF-8字符可能占用多个字节）
            position += c.len_utf8();

            // 收集所有在当前位置匹配的模式串
            for &len in &cur.borrow().lengths {
                // 通过位置和长度切出匹配的子串
                ans.push(&s[position - len..position]);
            }
        }
        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_aho_corasick() {
        let dict = ["abc", "abcd", "xyz", "acxy", "efg", "123", "678", "6543"];
        let ac = AhoCorasick::new(&dict);
        let res = ac.search("ababcxyzacxy12678acxy6543");
        assert_eq!(res, ["abc", "xyz", "acxy", "678", "acxy", "6543",]);
    }

    #[test]
    fn test_aho_corasick_with_utf8() {
        let dict = [
            "abc",
            "中文",
            "abc中",
            "abcd",
            "xyz",
            "acxy",
            "efg",
            "123",
            "678",
            "6543",
            "ハンバーガー",
        ];
        let ac = AhoCorasick::new(&dict);
        let res = ac.search("ababc中xyzacxy12678acxyハンバーガー6543中文");
        assert_eq!(
            res,
            [
                "abc",
                "abc中",
                "xyz",
                "acxy",
                "678",
                "acxy",
                "ハンバーガー",
                "6543",
                "中文"
            ]
        );
    }
}
