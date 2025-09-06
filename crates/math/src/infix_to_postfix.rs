#[derive(Debug, Clone, PartialEq, Eq)]
pub enum InfixToPostfixError {
    UnmatchedParent,
    UnknownCharacter(char),
}

/// 将中缀表达式转换为后缀表达式（逆波兰表示法）
///
/// # 参数
/// * `infix` - 一个表示中缀表达式的字符串切片
///
/// # 返回值
/// * `Ok(String)` - 转换成功的后缀表达式字符串
/// * `Err(InfixToPostfixError)` - 转换过程中遇到的错误，包括：
///   - `UnmatchedParent`: 括号不匹配
///   - `UnknownCharacter(char)`: 表达式中包含未知字符
///
/// # 算法说明
/// 使用调度场算法（Shunting Yard Algorithm）实现中缀转后缀：
/// 1. 遍历输入表达式的每个字符
/// 2. 对于操作数，直接输出到结果
/// 3. 对于运算符，根据优先级处理栈中运算符
/// 4. 对于括号，控制运算符的优先级顺序
/// 5. 最后处理栈中剩余的运算符
pub fn infix_to_postfix(infix: &str) -> Result<String, InfixToPostfixError> {
    let mut postfix = String::new();
    let mut stack: Vec<char> = Vec::new();

    // 定义运算符优先级函数
    let precedence = |op: char| -> u8 {
        match op {
            '+' | '-' => 1,
            '*' | '/' => 2,
            '^' => 3,
            _ => 0,
        }
    };

    // 遍历中缀表达式的每个字符
    for token in infix.chars() {
        match token {
            // 处理操作数：直接添加到后缀表达式中
            c if c.is_alphanumeric() => {
                postfix.push(c);
            }
            // 处理左括号：压入栈中
            '(' => {
                stack.push('(')
            }
            // 处理右括号：弹出栈中元素直到遇到左括号
            ')' => {
                while let Some(top) = stack.pop() {
                    if top == '(' {
                        break;
                    }
                    postfix.push(top);
                }
            }
            // 处理运算符：根据优先级规则处理栈中运算符
            '+' | '-' | '*' | '/' | '^' => {
                while let Some(top) = stack.last() {
                    // 如果栈顶是左括号或当前运算符优先级更高，则停止弹出
                    if *top == '(' || precedence(*top) < precedence(token) {
                        break;
                    }
                    postfix.push(stack.pop().unwrap());
                }
                stack.push(token);
            }
            // 处理未知字符：返回错误
            other => return Err(InfixToPostfixError::UnknownCharacter(other)),
        }
    }

    // 处理栈中剩余的运算符
    while let Some(top) = stack.pop() {
        // 如果还有左括号未匹配，则返回错误
        if top == '(' {
            return Err(InfixToPostfixError::UnmatchedParent);
        }

        postfix.push(top);
    }

    Ok(postfix)
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! test_infix_to_postfix {
        ($($name:ident: $inputs:expr,)*) => {
            $(
                #[test]
                fn $name() {
                    let (infix, expected) = $inputs;
                    assert_eq!(infix_to_postfix(infix), expected)
                }
            )*
        }
    }

    test_infix_to_postfix! {
        single_symbol: ("x", Ok(String::from("x"))),
        simple_sum: ("x+y", Ok(String::from("xy+"))),
        multiply_sum_left: ("x*(y+z)", Ok(String::from("xyz+*"))),
        multiply_sum_right: ("(x+y)*z", Ok(String::from("xy+z*"))),
        multiply_two_sums: ("(a+b)*(c+d)", Ok(String::from("ab+cd+*"))),
        product_and_power: ("a*b^c", Ok(String::from("abc^*"))),
        power_and_product: ("a^b*c", Ok(String::from("ab^c*"))),
        product_of_powers: ("(a*b)^c", Ok(String::from("ab*c^"))),
        product_in_exponent: ("a^(b*c)", Ok(String::from("abc*^"))),
        regular_0: ("a-b+c-d*e", Ok(String::from("ab-c+de*-"))),
        regular_1: ("a*(b+c)+d/(e+f)", Ok(String::from("abc+*def+/+"))),
        regular_2: ("(a-b+c)*(d+e*f)", Ok(String::from("ab-c+def*+*"))),
        unknown_character: ("(a-b)*#", Err(InfixToPostfixError::UnknownCharacter('#'))),
        unmatched_paren: ("((a-b)", Err(InfixToPostfixError::UnmatchedParent)),
    }
}
