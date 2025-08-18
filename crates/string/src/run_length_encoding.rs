/// 对输入字符串进行游程编码（Run-Length Encoding）
///
/// # 参数
/// * `target` - 需要编码的字符串引用
///
/// # 返回值
/// 返回编码后的字符串，格式为连续字符个数+字符的组合
///
/// # 示例
/// "aaa" -> "3a"
/// "aabbb" -> "2a3b"
#[allow(unused)]
pub fn run_length_encoding(target: &str) -> String {
    // 处理空字符串情况
    if target.trim().is_empty() {
        return "".to_string();
    }

    let mut count: i32 = 0;                    // 当前字符的计数
    let mut bash_character: String = "".to_string();  // 当前正在计数的字符
    let mut encoded_target = String::new();    // 存储编码结果

    // 遍历每个字符进行编码
    target.chars().for_each(|ch| {
        // 初始化第一个字符
        if bash_character == *"" {
            bash_character = ch.to_string();
        }

        // 如果当前字符与正在计数的字符相同，增加计数
        if ch.to_string() == bash_character {
            count += 1;
        } else {
            // 字符发生变化时，将计数和字符添加到结果中
            encoded_target.push_str(&count.to_string());
            count = 1;
            encoded_target.push_str(&bash_character);
            bash_character = ch.to_string();
        }
    });

    // 处理最后一组字符
    encoded_target.push_str(&count.to_string());
    encoded_target.push_str(&bash_character);
    encoded_target
}

/// 对游程编码的字符串进行解码
///
/// # 参数
/// * `target` - 需要解码的字符串引用，格式为数字+字符的组合
///
/// # 返回值
/// 返回解码后的原始字符串
///
/// # 示例
/// "3a2b" -> "aaabb"
/// "10a" -> "aaaaaaaaaa"
#[allow(unused)]
pub fn run_length_decoding(target: &str) -> String {
    // 处理空字符串情况
    if target.trim().is_empty() {
        return "".to_string();
    }

    let mut character_count = String::new();   // 临时存储字符计数
    let mut decoded_target = String::new();    // 存储解码结果

    // 遍历每个字符进行解码
    target.chars().for_each(|ch| {
        character_count.push(ch);
        let is_numeric: bool = character_count.parse::<i32>().is_ok();

        // 如果当前累积的字符串不是有效数字，说明遇到了字符部分
        if !is_numeric {
            let pop_char: char = character_count.pop().unwrap();  // 取出字符部分
            // 将字符重复指定次数后添加到结果中
            decoded_target.push_str(
                &pop_char.to_string().repeat(character_count.parse::<i32>().unwrap() as usize),
            );
            character_count = "".to_string();  // 重置计数器
        }
    });

    decoded_target
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! test_run_length {
        ($($name:ident: $test_case:expr,)*) => {
            $(
                #[test]
                fn $name() {
                    let (raw_str, encoded) = $test_case;
                    assert_eq!(run_length_encoding(raw_str), encoded);
                    assert_eq!(run_length_decoding(encoded), raw_str);
                }
            )*
        };
    }

    test_run_length! {
        empty_input: ("", ""),
        repeated_char: ("aaaaaaaaaa", "10a"),
        no_repeated: ("abcdefghijk", "1a1b1c1d1e1f1g1h1i1j1k"),
        regular_input: ("aaaaabbbcccccdddddddddd", "5a3b5c10d"),
        two_blocks_with_same_char: ("aaabbaaaa", "3a2b4a"),
        long_input: ("aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaabbbcccccdddddddddd", "200a3b5c10d"),
    }
}
