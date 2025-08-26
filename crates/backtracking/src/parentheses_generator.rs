/**
 * 生成所有可能的并且有效的括号组合
 *
 * # 参数
 * * `n` - 括号对数
 *
 * # 返回值
 * 返回包含所有有效括号组合的字符串向量
 */
#[allow(unused)]
pub fn generate_parentheses(n: usize) -> Vec<String> {
    let mut result = Vec::new();
    // 只有当n大于0时才生成括号组合
    if n > 0 {
        generate("", 0, 0, n, &mut result);
    }
    result
}

/**
 * 递归生成有效括号组合
 *
 * # 参数
 * * `current` - 当前正在构建的括号字符串
 * * `open_count` - 当前已使用的左括号数量
 * * `close_count` - 当前已使用的右括号数量
 * * `n` - 需要生成的括号对数
 * * `result` - 存储所有有效括号组合的结果向量
 */
fn generate(
    current: &str,
    open_count: usize,
    close_count: usize,
    n: usize,
    result: &mut Vec<String>,
) {
    // 当字符串长度达到2*n时，说明已经生成了一个完整的括号组合
    if current.len() == (n * 2) {
        result.push(current.to_string());
        return;
    }
    // 如果左括号数量小于n，可以继续添加左括号
    if open_count < n {
        let new_str = current.to_string() + "(";
        generate(&new_str, open_count + 1, close_count, n, result);
    }
    // 如果右括号数量小于左括号数量，可以继续添加右括号
    if close_count < open_count {
        let new_str = current.to_string() + ")";
        generate(&new_str, open_count, close_count + 1, n, result);
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! generate_parentheses_tests {
        ($($name:ident: $test_case:expr,)*) => {
            $(
                #[test]
                fn $name() {
                    let (n, expected_result) = $test_case;
                    assert_eq!(generate_parentheses(n), expected_result);
                }
            )*
        };
    }

    generate_parentheses_tests! {
        test_generate_parentheses_0: (0, Vec::<String>::new()),
        test_generate_parentheses_1: (1, vec!["()"]),
        test_generate_parentheses_2: (2, vec!["(())", "()()"]),
        test_generate_parentheses_3: (3, vec!["((()))", "(()())", "(())()", "()(())", "()()()"]),
        test_generate_parentheses_4: (4, vec!["(((())))", "((()()))", "((())())", "((()))()", "(()(()))", "(()()())", "(()())()", "(())(())", "(())()()", "()((()))", "()(()())", "()(())()", "()()(())", "()()()()"]),
    }
}
