/// 生成给定整数向量的所有唯一排列（考虑重复元素）。
///
/// 该函数使用回溯算法来生成所有可能的排列，并通过排序和跳过重复元素的方式避免重复排列。
///
/// # 参数
///
/// * `nums` - 一个包含整数的向量，可以包含重复元素。
///
/// # 返回值
///
/// 返回一个二维向量，其中每个子向量是输入向量的一个唯一排列。
pub fn permute(
    mut nums: Vec<isize>
) -> Vec<Vec<isize>> {
    let mut permutations = Vec::new();
    let mut curr = Vec::new();
    let mut used = vec![false; nums.len()];
    nums.sort(); // 排序以便处理重复元素
    generate(&nums, &mut curr, &mut used, &mut permutations);
    permutations
}

/// 递归生成排列的辅助函数。
///
/// 使用回溯法生成所有排列。当当前路径长度等于输入数组长度时，将当前路径加入结果集。
/// 对于重复元素，通过判断前一个相同元素是否已被使用，来决定是否跳过当前元素以避免重复。
///
/// # 参数
///
/// * `nums` - 原始排序后的整数切片。
/// * `curr` - 当前正在构建的排列。
/// * `used` - 标记每个元素是否已被使用的布尔向量。
/// * `permutations` - 存储所有生成的排列的向量。
fn generate(
    nums: &[isize],
    curr: &mut Vec<isize>,
    used: &mut Vec<bool>,
    permutations: &mut Vec<Vec<isize>>,
) {
    // 如果当前排列长度等于原数组长度，说明已生成一个完整排列
    if curr.len() == nums.len() {
        permutations.push(curr.clone());
        return;
    }

    // 遍历所有元素尝试添加到当前排列中
    for idx in 0..nums.len() {
        // 跳过已被使用的元素
        if used[idx] {
            continue;
        }

        // 跳过会导致重复的元素：当前元素与前一个元素相同，且前一个元素未被使用
        if idx > 0 && nums[idx] == nums[idx - 1] && !used[idx - 1] {
            continue;
        }

        // 选择当前元素
        curr.push(nums[idx]);
        used[idx] = true;

        // 递归生成剩余部分的排列
        generate(nums, curr, used, permutations);

        // 回溯：撤销选择
        curr.pop();
        used[idx] = false;
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! permute_tests {
        ($($name:ident: $test_case:expr,)*) => {
            $(
                #[test]
                fn $name() {
                    let (input, expected) = $test_case;
                    assert_eq!(permute(input), expected);
                }
            )*
        }
    }

    permute_tests! {
        test_permute_basic: (vec![1, 2, 3], vec![
            vec![1, 2, 3],
            vec![1, 3, 2],
            vec![2, 1, 3],
            vec![2, 3, 1],
            vec![3, 1, 2],
            vec![3, 2, 1],
        ]),
        test_permute_empty: (Vec::<isize>::new(), vec![vec![]]),
        test_permute_single: (vec![1], vec![vec![1]]),
        test_permute_duplicates: (vec![1, 1, 2], vec![
            vec![1, 1, 2],
            vec![1, 2, 1],
            vec![2, 1, 1],
        ]),
        test_permute_all_duplicates: (vec![1, 1, 1, 1], vec![
            vec![1, 1, 1, 1],
        ]),
        test_permute_negative: (vec![-1, -2, -3], vec![
            vec![-3, -2, -1],
            vec![-3, -1, -2],
            vec![-2, -3, -1],
            vec![-2, -1, -3],
            vec![-1, -3, -2],
            vec![-1, -2, -3],
        ]),
        test_permute_mixed: (vec![-1, 0, 1], vec![
            vec![-1, 0, 1],
            vec![-1, 1, 0],
            vec![0, -1, 1],
            vec![0, 1, -1],
            vec![1, -1, 0],
            vec![1, 0, -1],
        ]),
        test_permute_larger: (vec![1, 2, 3, 4], vec![
            vec![1, 2, 3, 4],
            vec![1, 2, 4, 3],
            vec![1, 3, 2, 4],
            vec![1, 3, 4, 2],
            vec![1, 4, 2, 3],
            vec![1, 4, 3, 2],
            vec![2, 1, 3, 4],
            vec![2, 1, 4, 3],
            vec![2, 3, 1, 4],
            vec![2, 3, 4, 1],
            vec![2, 4, 1, 3],
            vec![2, 4, 3, 1],
            vec![3, 1, 2, 4],
            vec![3, 1, 4, 2],
            vec![3, 2, 1, 4],
            vec![3, 2, 4, 1],
            vec![3, 4, 1, 2],
            vec![3, 4, 2, 1],
            vec![4, 1, 2, 3],
            vec![4, 1, 3, 2],
            vec![4, 2, 1, 3],
            vec![4, 2, 3, 1],
            vec![4, 3, 1, 2],
            vec![4, 3, 2, 1],
        ]),
    }
}
