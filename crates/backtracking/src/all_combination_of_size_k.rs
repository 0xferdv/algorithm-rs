/// 组合生成错误类型枚举
///
/// 定义了在生成组合过程中可能出现的两种错误情况
#[derive(Debug, PartialEq)]
pub enum CombinationError {
    /// 当要求选择的元素个数k大于可选元素总数n时返回此错误
    KGreaterThanN,
    /// 当可选元素范围为0但要求选择元素个数大于0时返回此错误
    InvalidZeroRange,
}

/// 生成从0到n-1这n个数字中选取k个数字的所有可能组合
///
/// 使用回溯算法生成所有可能的组合，每个组合包含k个不重复的数字，
/// 数字范围在0到n-1之间，组合内数字按升序排列。
///
/// # 参数
///
/// * `n` - 可选数字的总数，数字范围为0到n-1
/// * `k` - 每个组合中包含的数字个数
///
/// # 返回值
///
/// * `Ok(Vec<Vec<usize>>)` - 成功时返回包含所有组合的向量，每个组合是一个升序排列的数字向量
/// * `Err(CombinationError)` - 失败时返回相应的错误类型
///
/// # 示例
///
///
/// 从0,1,2,3中选择2个数字的所有组合
/// let result = generate_all_combinations(4, 2);
/// assert_eq!(result, Ok(vec![
/// vec![0, 1],
/// vec![0, 2],
/// vec![0, 3],
/// vec![1, 2],
/// vec![1, 3],
/// vec![2, 3],
/// ]));
///
///
/// # 错误处理
///
/// * 当k > n时，返回`CombinationError::KGreaterThanN`
/// * 当n = 0且k > 0时，返回`CombinationError::InvalidZeroRange`
#[allow(unused)]
pub fn generate_all_combinations(
    n: usize,
    k: usize,
) -> Result<Vec<Vec<usize>>, CombinationError> {
    // 检查边界条件：当没有可选数字但需要选择数字时
    if n == 0 && k > 0 {
        return Err(CombinationError::InvalidZeroRange);
    }
    // 检查边界条件：当需要选择的数字个数超过可选数字总数时
    if k > n {
        return Err(CombinationError::KGreaterThanN);
    }

    // 存储所有生成的组合
    let mut combinations = vec![];
    // 当前正在构建的组合，初始化为k个0
    let mut curr = vec![0; k];
    // 调用回溯函数生成所有组合
    backtrack(0, n, k, 0, &mut curr, &mut combinations);
    Ok(combinations)
}

/// 回溯算法核心函数，递归生成所有可能的组合
///
/// 通过深度优先搜索的方式，逐位填充组合中的每个位置，
/// 确保生成的组合满足升序且不重复的要求。
///
/// # 参数
///
/// * `start` - 当前可选择数字的起始位置，用于避免重复组合
/// * `n` - 可选数字的总数，数字范围为0到n-1
/// * `k` - 每个组合中包含的数字个数
/// * `index` - 当前正在填充的组合位置索引
/// * `curr` - 当前正在构建的组合向量（可变引用）
/// * `combinations` - 存储所有已完成组合的向量（可变引用）
fn backtrack(
    start: usize,
    n: usize,
    k: usize,
    index: usize,
    curr: &mut Vec<usize>,
    combinations: &mut Vec<Vec<usize>>,
) {
    // 基础情况：当组合已填满k个数字时，将当前组合加入结果集
    if index == k {
        combinations.push((curr).clone());
        return;
    }

    // 优化的循环范围：(start..=(n - k + index))
    // 确保剩余未填位置有足够的数字可选
    // 例如：当n=5, k=3, index=1时，当前位最多只能选到5-3+1=3
    (start..=(n - k + index)).for_each(|num| {
        // 将当前数字填入组合的指定位置
        curr[index] = num;
        // 递归填充下一个位置，起始数字为当前数字+1以避免重复
        backtrack(num + 1, n, k, index + 1, curr, combinations);
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! combination_tests {
        ($($name:ident: $test_case:expr,)*) => {
            $(
                #[test]
                fn $name() {
                    let (n, k, expected) = $test_case;
                    assert_eq!(generate_all_combinations(n, k), expected);
                }
            )*
        }
    }

    combination_tests! {
        test_generate_4_2: (4, 2, Ok(vec![
            vec![0, 1],
            vec![0, 2],
            vec![0, 3],
            vec![1, 2],
            vec![1, 3],
            vec![2, 3],
        ])),
        test_generate_4_3: (4, 3, Ok(vec![
            vec![0, 1, 2],
            vec![0, 1, 3],
            vec![0, 2, 3],
            vec![1, 2, 3],
        ])),
        test_generate_5_3: (5, 3, Ok(vec![
            vec![0, 1, 2],
            vec![0, 1, 3],
            vec![0, 1, 4],
            vec![0, 2, 3],
            vec![0, 2, 4],
            vec![0, 3, 4],
            vec![1, 2, 3],
            vec![1, 2, 4],
            vec![1, 3, 4],
            vec![2, 3, 4],
        ])),
        test_generate_5_1: (5, 1, Ok(vec![
            vec![0],
            vec![1],
            vec![2],
            vec![3],
            vec![4],
        ])),
        test_empty: (0, 0, Ok(vec![vec![]])),
        test_generate_n_eq_k: (3, 3, Ok(vec![
            vec![0, 1, 2],
        ])),
        test_generate_k_greater_than_n: (3, 4, Err(CombinationError::KGreaterThanN)),
        test_zero_range_with_nonzero_k: (0, 1, Err(CombinationError::InvalidZeroRange)),
    }
}
