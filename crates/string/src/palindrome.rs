/// 判断一个字符串是否为回文串。
///
/// 该函数会忽略大小写、空格和标点符号，仅考虑字母和数字字符。
/// 判断方式是将处理后的字符串与其反转进行比较。
///
/// # 参数
///
/// * `s` - 待检测的字符串引用
///
/// # 返回值
///
/// 如果字符串是回文串则返回 true，否则返回 false
#[allow(unused)]
pub fn palindrome(s: &str) -> bool {
    // 过滤出字母和数字字符，并转换为小写，用于后续比较
    let mut chars = s
        .chars()
        .filter(|c| c.is_ascii_alphanumeric())
        .map(|c| c.to_ascii_lowercase());

    // 使用双指针从两端向中间逐一比较字符
    while let (Some(c1), Some(c2)) = (chars.next(), chars.next_back()) {
        if c1 != c2 {
            return false;
        }
    }

    true
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! palindrome_tests {
        ($($name:ident: $inputs:expr,)*) => {
            $(
                #[test]
                fn $name() {
                    let (input, expected) = $inputs;
                    assert_eq!(palindrome(input), expected);
                }
            )*
        }
    }

    palindrome_tests! {
        odd_palindrome: ("madam", true),
        even_palindrome: ("deified", true),
        single_character_palindrome: ("x", true),
        single_word_palindrome: ("eye", true),
        case_insensitive_palindrome: ("RaceCar", true),
        mixed_case_and_punctuation_palindrome: ("A man, a plan, a canal, Panama!", true),
        mixed_case_and_space_palindrome: ("No 'x' in Nixon", true),
        empty_string: ("", true),
        pompeii_palindrome: ("Roma-Olima-Milo-Amor", true),
        napoleon_palindrome: ("Able was I ere I saw Elba", true),
        john_taylor_palindrome: ("Lewd did I live, & evil I did dwel", true),
        well_know_english_palindrome: ("Never odd or even", true),
        palindromic_phrase: ("Rats live on no evil star", true),
        names_palindrome: ("Hannah", true),
        prime_minister_of_cambodia: ("Lon Nol", true),
        japanese_novelist_and_manga_writer: ("Nisio Isin", true),
        actor: ("Robert Trebor", true),
        rock_vocalist: ("Ola Salo", true),
        pokemon_species: ("Girafarig", true),
        lychrel_num_56: ("121", true),
        universal_palindrome_date: ("02/02/2020", true),
        french_palindrome: ("une Slave valse nu", true),
        finnish_palindrome: ("saippuakivikauppias", true),
        non_palindrome_simple: ("hello", false),
        non_palindrome_with_punctuation: ("hello!", false),
        non_palindrome_mixed_case: ("Hello, World", false),
    }
}
