/// Burrows-Wheeler 变换（BWT）是一种用于数据压缩的预处理算法。
/// 该函数对输入字符串执行 BWT 编码，返回编码后的字符串和原始字符串在变换表中的索引。
///
/// # 参数
/// * `input`: 需要进行 BWT 编码的字符串引用。
///
/// # 返回值
/// 返回一个元组：
/// - 第一个元素是经过 BWT 编码后的字符串；
/// - 第二个元素是原始字符串在排序后变换表中的行索引。
#[allow(unused)]
pub fn burrows_wheeler_transform(input: &str) -> (String, usize) {
    let len = input.len();
    // 构造所有循环移位的字符串列表
    let mut table = Vec::<String>::with_capacity(len);
    (0..len).for_each(|idx| {
        table.push(input[idx..].to_owned() + &input[..idx]);
    });
    // 按字典序对所有循环移位字符串进行排序
    table.sort_by_key(|item| item.to_lowercase());
    let mut encoded = String::new();
    let mut idx: usize = 0;
    // 提取每行最后一个字符组成编码结果，并记录原始字符串所在位置
    table.iter().enumerate().take(len).for_each(|(next_idx, item)| {
        encoded.push(item.chars().last().unwrap());
        if item.eq(&input) { idx = next_idx }
    });
    (encoded, idx)
}

/// Burrows-Wheeler 变换的逆变换（I B W T），用于将 BWT 编码还原为原始字符串。
///
/// # 参数
/// * `input`: 一个元组，包含：
///   - 第一个元素是 BWT 编码后的字符串；
///   - 第二个元素是原始字符串在变换表中的索引。
///
/// # 返回值
/// 返回解码后的原始字符串。
#[allow(unused)]
pub fn inv_burrows_wheeler_transform<T: AsRef<str>>(input: (T, usize)) -> String {
    let len = input.0.as_ref().len();
    // 构建字符与其原始索引的映射表
    let mut table = Vec::<(usize, char)>::with_capacity(len);
    (0..len).for_each(|idx| {
        table.push((idx, input.0.as_ref().chars().nth(idx).unwrap()));
    });
    // 根据字符对表进行排序，以恢复原始顺序
    table.sort_by(|item_a, item_b| item_a.1.cmp(&item_b.1));
    let mut decoded = String::new();
    let mut idx = input.1;
    // 使用索引链重构原始字符串
    (0..len).for_each(|_| {
        decoded.push(table[idx].1);
        idx = table[idx].0;
    });
    decoded
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn stand_alone_function() {
        assert_eq!(
            burrows_wheeler_transform("CARROT"),
            ("CTRRAO".to_owned(), 1usize)
        );
        assert_eq!(inv_burrows_wheeler_transform(("CTRRAO", 1usize)), "CARROT");
        assert_eq!(
            burrows_wheeler_transform("THEALGORITHMS"),
            ("EHLTTRAHGOMSI".to_owned(), 11usize)
        );
        assert_eq!(
            inv_burrows_wheeler_transform(("EHLTTRAHGOMSI".to_string(), 11usize)),
            "THEALGORITHMS"
        );
        assert_eq!(
            burrows_wheeler_transform("!.!.!??.=::"),
            (":..!!?:=.?!".to_owned(), 0usize)
        );
        assert_eq!(
            inv_burrows_wheeler_transform((":..!!?:=.?!", 0usize)),
            "!.!.!??.=::"
        );
    }
    #[test]
    fn basic_characters() {
        assert_eq!(
            inv_burrows_wheeler_transform(burrows_wheeler_transform("CARROT")),
            "CARROT"
        );
        assert_eq!(
            inv_burrows_wheeler_transform(burrows_wheeler_transform("TOMATO")),
            "TOMATO"
        );
        assert_eq!(
            inv_burrows_wheeler_transform(burrows_wheeler_transform("THISISATEST")),
            "THISISATEST"
        );
        assert_eq!(
            inv_burrows_wheeler_transform(burrows_wheeler_transform("THEALGORITHMS")),
            "THEALGORITHMS"
        );
        assert_eq!(
            inv_burrows_wheeler_transform(burrows_wheeler_transform("RUST")),
            "RUST"
        );
    }

    #[test]
    fn special_characters() {
        assert_eq!(
            inv_burrows_wheeler_transform(burrows_wheeler_transform("!.!.!??.=::")),
            "!.!.!??.=::"
        );
        assert_eq!(
            inv_burrows_wheeler_transform(burrows_wheeler_transform("!{}{}(((&&%%!??.=::")),
            "!{}{}(((&&%%!??.=::"
        );
        assert_eq!(
            inv_burrows_wheeler_transform(burrows_wheeler_transform("//&$[]")),
            "//&$[]"
        );
    }

    #[test]
    fn empty() {
        assert_eq!(
            inv_burrows_wheeler_transform(burrows_wheeler_transform("")),
            ""
        );
    }
}
