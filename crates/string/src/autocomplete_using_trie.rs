use std::collections::HashMap;

#[allow(unused)]
use std::hash::Hash;

/// Trie树结构，用于存储字符串前缀信息
///
/// 该结构使用HashMap来存储字符到子Trie节点的映射关系，
/// 每个节点代表一个字符，通过递归结构构建完整的前缀树
#[derive(Debug)]
#[allow(unused)]
struct Trie(HashMap<char, Box<Trie>>);

/// 结束标记字符，用于标识单词的结尾
#[allow(unused)]
const END: char = '#';

impl Trie {

    /// 创建一个新的空Trie节点
    ///
    /// # Returns
    /// 返回一个初始化为空HashMap的Trie实例
    fn new() -> Self {
        Self(HashMap::new())
    }

    /// 向Trie树中插入一个字符串
    ///
    /// # Arguments
    /// * `text` - 需要插入的字符串切片
    ///
    /// 该方法会遍历字符串中的每个字符，在Trie树中创建相应的路径，
    /// 并在字符串末尾添加结束标记
    #[allow(unused)]
    fn insert(&mut self, text: &str) {
        let mut trie = self;
        for c in text.chars() {
            trie = trie.0.entry(c).or_insert_with(|| Box::new(Trie::new()))
        }
        // 在单词末尾添加结束标记
        trie.0.insert(END, Box::new(Trie::new()));
    }

    /// 查找具有指定前缀的所有字符串
    ///
    /// # Arguments
    /// * `prefix` - 要搜索的前缀字符串
    ///
    /// # Returns
    /// 返回所有以指定前缀开头的完整字符串向量
    #[allow(unused)]
    fn find(&self, prefix: &str) -> Vec<String> {
        let mut trie = self;
        // 遍历前缀中的每个字符，找到对应的Trie节点
        for c in prefix.chars() {
            let char_trie = trie.0.get(&c);
            if let Some(char_trie) = char_trie {
                trie = char_trie;
            } else {
                // 如果前缀不存在，返回空向量
                return vec![];
            }
        }
        // 获取该节点下的所有元素，并加上前缀
        Self::_elements(trie)
            .iter()
            .map(|s| prefix.to_owned() + s)
            .collect()
    }

    /// 递归获取Trie树中所有可能的字符串组合
    ///
    /// # Arguments
    /// * `map` - 当前要处理的Trie节点
    ///
    /// # Returns
    /// 返回从当前节点开始的所有可能字符串后缀向量
    fn _elements(map: &Trie) -> Vec<String> {
        let mut results = vec![];
        // 遍历当前节点的所有子节点
        for (c, v) in map.0.iter() {
            let mut sub_result = vec![];
            if c == &END {
                // 遇到结束标记，添加空字符串表示一个完整单词的结束
                sub_result.push("".to_owned())
            } else {
                // 递归处理子节点，并将当前字符添加到所有子结果前面
                Self::_elements(v)
                    .iter()
                    .map(|s| sub_result.push(c.to_string() + s))
                    .collect()
            }
            results.extend(sub_result)
        }
        results
    }
}

/// 自动补全结构体，提供基于前缀的单词查找功能
///
/// 封装了Trie树结构，对外提供简单的插入和查找接口
pub struct Autocomplete {
    trie: Trie,
}

impl Autocomplete {

    /// 创建一个新的自动补全实例
    ///
    /// # Returns
    /// 返回一个初始化的Autocomplete实例，内部包含空的Trie树
    fn new() -> Self {
        Self {
            trie: Trie::new()
        }
    }

    /// 批量插入单词到自动补全结构中
    ///
    /// # Arguments
    /// * `words` - 包含要插入单词的切片，支持任何可以转换为字符串引用的类型
    #[allow(unused)]
    fn insert_words<T: AsRef<str>>(&mut self, words: &[T]) {
        words.iter().for_each(|word| {
            self.trie.insert(word.as_ref())
        });
    }

    /// 根据前缀查找匹配的单词
    ///
    /// # Arguments
    /// * `prefix` - 要匹配的前缀字符串
    ///
    /// # Returns
    /// 返回所有以指定前缀开头的单词向量
    #[allow(unused)]
    #[inline]
    pub fn find_words(&self, prefix: &str) -> Vec<String> {
        self.trie.find(prefix)
    }
}

impl Default for Autocomplete {

    /// 为Autocomplete实现Default trait，提供默认实例创建方法
    ///
    /// # Returns
    /// 返回一个新的Autocomplete实例
    fn default() -> Self { Self::new() }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_autocomplete() {
        let words = vec!["apple", "orange", "oregano"];

        let mut auto_complete = Autocomplete::new();
        auto_complete.insert_words(&words);

        let prefix = "app";
        let mut auto_completed_words = auto_complete.find_words(prefix);

        let mut apple = vec!["apple"];
        apple.sort();

        auto_completed_words.sort();
        assert_eq!(auto_completed_words, apple);

        let prefix = "or";
        let mut auto_completed_words = auto_complete.find_words(prefix);

        let mut prefix_or = vec!["orange", "oregano"];
        prefix_or.sort();

        auto_completed_words.sort();
        assert_eq!(auto_completed_words, prefix_or);
    }
}
