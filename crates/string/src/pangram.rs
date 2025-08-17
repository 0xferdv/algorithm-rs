use std::collections::{HashSet, HashMap};

/// 枚举类型，表示字符串是否为全字母句（Pangram）的状态。
///
/// - `NotPangram`: 不是全字母句。
/// - `Pangram`: 是全字母句但不是完美全字母句。
/// - `PerfectPangram`: 是完美全字母句（每个字母恰好出现一次）。
#[derive(PartialEq, Debug)]
pub enum PangramStatus {
    NotPangram,
    Pangram,
    PerfectPangram,
}

/// 统计输入字符串中每个英文字母的出现次数。
///
/// 该函数会忽略非英文字母字符，并将所有字母转换为小写进行统计。
///
/// # 参数
/// * `pangram_str`: 需要分析的字符串切片。
///
/// # 返回值
/// 返回一个 HashMap，键为英文字母，值为该字母在字符串中的出现次数。
fn compute_letter_counts(pangram_str: &str) -> HashMap<char, usize> {
    let mut letter_counts = HashMap::new();
    // 遍历字符串中的每个字符，过滤出英文字母并转为小写后计数
    for ch in pangram_str
        .to_lowercase()
        .chars()
        .filter(|ch| ch.is_ascii_alphabetic()) {
        *letter_counts.entry(ch).or_insert(0) += 1;
    }
    letter_counts
}

/// 判断给定字符串是否为全字母句，并返回相应的状态。
///
/// 全字母句是指包含英语字母表中所有26个字母的句子。
/// 完美全字母句是指每个字母恰好出现一次的全字母句。
///
/// # 参数
/// * `pangram_str`: 待检测的字符串切片。
///
/// # 返回值
/// 返回 `PangramStatus` 枚举值，表示字符串的类型：
/// - `NotPangram`: 不是全字母句；
/// - `Pangram`: 是全字母句但不是完美全字母句；
/// - `PerfectPangram`: 是完美全字母句。
#[allow(unused)]
pub fn pangram(pangram_str: &str) -> PangramStatus {
    let letter_counts = compute_letter_counts(pangram_str);
    let alphabet: HashSet<char> = ('a'..='z').collect();
    let used_letters: HashSet<char> = letter_counts.keys().cloned().collect();

    // 如果使用的字母集合不等于完整字母表，则不是全字母句
    if used_letters != alphabet {
        return PangramStatus::NotPangram;
    }

    // 如果所有字母都只出现一次，则是完美全字母句；否则是一般全字母句
    if letter_counts.values().all(|&count| count == 1) {
        PangramStatus::PerfectPangram
    } else {
        PangramStatus::Pangram
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! pangram_tests {
        ($($name:ident: $tc:expr,)*) => {
            $(
                #[test]
                fn $name() {
                    let (input, expected) = $tc;
                    assert_eq!(pangram(input), expected);
                }
            )*
        };
    }

    pangram_tests! {
        test_not_pangram_simple: ("This is not a pangram", PangramStatus::NotPangram),
        test_not_pangram_day: ("today is a good day", PangramStatus::NotPangram),
        test_pangram_standard: ("The quick brown fox jumps over the lazy dog", PangramStatus::Pangram),
        test_pangram_boxer: ("A mad boxer shot a quick, gloved jab to the jaw of his dizzy opponent", PangramStatus::Pangram),
        test_pangram_discotheques: ("Amazingly few discotheques provide jukeboxes", PangramStatus::Pangram),
        test_pangram_zebras: ("How vexingly quick daft zebras jump", PangramStatus::Pangram),
        test_perfect_pangram_jock: ("Mr. Jock, TV quiz PhD, bags few lynx", PangramStatus::PerfectPangram),
        test_empty_string: ("", PangramStatus::NotPangram),
        test_non_alphabetic: ("12345!@#$%", PangramStatus::NotPangram),
        test_mixed_case_pangram: ("ThE QuiCk BroWn FoX JumPs OveR tHe LaZy DoG", PangramStatus::Pangram),
        test_perfect_pangram_with_symbols: ("Mr. Jock, TV quiz PhD, bags few lynx!", PangramStatus::PerfectPangram),
        test_long_non_pangram: (&"a".repeat(1000), PangramStatus::NotPangram),
        test_near_pangram_missing_one_letter: ("The quick brown fox jumps over the lazy do", PangramStatus::NotPangram),
        test_near_pangram_with_special_characters: ("Th3 qu!ck brown f0x jumps 0v3r th3 l@zy d0g.", PangramStatus::NotPangram),
    }
}
