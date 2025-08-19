use std::collections::HashMap;

/// 判断两个字符串是否同构
///
/// 两个字符串同构的定义是：字符串s中的每个字符都可以唯一映射到字符串t中的一个字符，
/// 同时字符串t中的每个字符也可以唯一映射到字符串s中的一个字符。
///
/// # 参数
/// * [s](file://E:\0xferdv\001-rust\algorithm-rs\Rust-master\src\backtracking\mod.rs) - 第一个字符串切片
/// * [t](file://E:\0xferdv\001-rust\algorithm-rs\target\debug\build\zerocopy-191912f0e0e75127\output) - 第二个字符串切片
///
/// # 返回值
/// * `true` - 如果两个字符串同构
/// * `false` - 如果两个字符串不同构
///
/// # 约束条件
/// * 两个字符串长度必须相等，否则返回false
/// * 映射关系必须是一一对应的（双射）
///
/// # 示例
///
/// assert_eq!(isomorphism("egg", "add"), true); // e->a, g->d
/// assert_eq!(isomorphism("foo", "bar"), false); // o不能同时映射到a和r
///

///
/// # 算法说明
/// 该函数通过维护两个HashMap来实现双向映射检查：
/// 1. 从字符串s到字符串t的字符映射
/// 2. 从字符串t到字符串s的字符映射
///
/// 对于每一对对应位置的字符，检查是否存在冲突的映射关系。
/// 如果发现冲突，则返回false；如果所有字符都检查通过，则返回true。
#[allow(unused)]
pub fn isomorphism(s: &str, t: &str) -> bool {
    // 将字符串转换为字符向量以便按索引访问
    let s_chars: Vec<char> = s.chars().collect();
    let t_chars: Vec<char> = t.chars().collect();

    // 长度不相等的字符串不可能同构
    if s_chars.len() != t_chars.len() {
        return false;
    }

    // 创建两个映射表，分别记录s->t和t->s的映射关系
    let mut s_to_t_map = HashMap::new();
    let mut t_to_s_map = HashMap::new();

    // 遍历两个字符串中对应位置的字符对
    for (s_char, t_char) in s_chars.into_iter().zip(t_chars) {
        // 检查s->t的映射关系是否一致
        // 检查t->s的映射关系是否一致
        // 如果任一映射关系冲突，则返回false
        if !check_mapping(&mut s_to_t_map, s_char, t_char)
            || !check_mapping(&mut t_to_s_map, t_char, s_char)
        {
            return false;
        }
    }
    // 所有字符对都满足映射关系，返回true
    true
}

/// 检查并更新字符映射关系
///
/// 该函数用于检查给定的键值对是否与已存在的映射关系冲突，
/// 如果不冲突则将新的映射关系添加到映射表中。
///
/// # 参数
/// * `map` - 字符映射表的可变引用
/// * `key` - 映射关系的键字符
/// * `value` - 映射关系的值字符
///
/// # 返回值
/// * `true` - 映射关系一致或成功添加新映射
/// * `false` - 发现映射冲突
///
/// # 算法逻辑
/// 1. 如果键已存在于映射表中：
///    - 检查其对应的值是否与当前值相同
///    - 相同则返回true，不同则返回false
/// 2. 如果键不存在于映射表中：
///    - 将新的键值对插入映射表
///    - 返回true
fn check_mapping(map: &mut HashMap<char, char>, key: char, value: char) -> bool {
    match map.get(&key) {
        // 如果键已存在，检查值是否一致
        Some(&mapped_char) => mapped_char == value,
        // 如果键不存在，插入新的映射关系并返回true
        None => {
            map.insert(key, value);
            true
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! test_is_isomorphic {
        ($($name:ident: $inputs:expr,)*) => {
        $(
            #[test]
            fn $name() {
                let (s, t, expected) = $inputs;
                assert_eq!(isomorphism(s, t), expected);
                assert_eq!(isomorphism(t, s), expected);
                assert!(isomorphism(s, s));
                assert!(isomorphism(t, t));
            }
        )*
        }
    }

    test_is_isomorphic! {
        isomorphic: ("egg", "add", true),
        isomorphic_long: ("abcdaabdcdbbabababacdadad", "AbCdAAbdCdbbAbAbAbACdAdAd", true),
        not_isomorphic: ("egg", "adc", false),
        non_isomorphic_long: ("abcdaabdcdbbabababacdadad", "AACdAAbdCdbbAbAbAbACdAdAd", false),
        isomorphic_unicode: ("天苍苍", "野茫茫", true),
        isomorphic_unicode_different_byte_size: ("abb", "野茫茫", true),
        empty: ("", "", true),
        different_length: ("abc", "abcd", false),
    }
}
