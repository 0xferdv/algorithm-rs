/// 判断给定的整数集合中是否存在一个子集，其元素之和等于目标值。
///
/// 使用回溯法递归地检查所有可能的子集组合。
///
/// # 参数
///
/// * `set` - 一个包含整数的切片，表示待检查的集合。
/// * `target` - 目标和，函数将判断是否存在子集的和等于该值。
///
/// # 返回值
///
/// 如果存在满足条件的子集，返回 `true`；否则返回 `false`。
#[allow(unused)]
pub fn has_subset_with_sum(set: &[isize], target: isize) -> bool {
    backtrack(set, set.len(), target)
}

/// 回溯函数，用于递归查找是否存在满足条件的子集。
///
/// 通过两种选择进行递归：
/// 1. 不包含当前元素；
/// 2. 包含当前元素（从目标值中减去该元素）。
///
/// # 参数
///
/// * `set` - 原始整数集合。
/// * `remaining_items` - 当前考虑的元素数量（从后往前）。
/// * `target` - 当前剩余的目标和。
///
/// # 返回值
///
/// 如果在当前状态下可以找到满足条件的子集，返回 `true`；否则返回 `false`。
fn backtrack(set: &[isize], remaining_items: usize, target: isize) -> bool {
    // 基本情况：如果目标和为0，说明找到了符合条件的子集。
    if target == 0 {
        return true;
    }
    // 基本情况：如果没有剩余元素可选，但目标和不为0，则无解。
    if remaining_items == 0 {
        return false;
    }
    // 递归尝试两种情况：
    // 1. 不选取当前元素；
    // 2. 选取当前元素，并从目标和中减去它。
    backtrack(set, remaining_items - 1, target)
        || backtrack(set, remaining_items - 1, target - set[remaining_items - 1])
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! has_subset_with_sum_tests {
        ($($name:ident: $test_case:expr,)*) => {
            $(
                #[test]
                fn $name() {
                    let (set, target, expected) = $test_case;
                    assert_eq!(has_subset_with_sum(set, target), expected);
                }
            )*
        }
    }

    has_subset_with_sum_tests! {
        test_small_set_with_sum: (&[3, 34, 4, 12, 5, 2], 9, true),
        test_small_set_without_sum: (&[3, 34, 4, 12, 5, 2], 30, false),
        test_consecutive_set_with_sum: (&[1, 2, 3, 4, 5, 6], 10, true),
        test_consecutive_set_without_sum: (&[1, 2, 3, 4, 5, 6], 22, false),
        test_large_set_with_sum: (&[5, 10, 12, 13, 15, 18, -1, 10, 50, -2, 3, 4], 30, true),
        test_empty_set: (&[], 0, true),
        test_empty_set_with_nonzero_sum: (&[], 10, false),
        test_single_element_equal_to_sum: (&[10], 10, true),
        test_single_element_not_equal_to_sum: (&[5], 10, false),
        test_negative_set_with_sum: (&[-7, -3, -2, 5, 8], 0, true),
        test_negative_sum: (&[1, 2, 3, 4, 5], -1, false),
        test_negative_sum_with_negatives: (&[-7, -3, -2, 5, 8], -4, true),
        test_negative_sum_with_negatives_no_solution: (&[-7, -3, -2, 5, 8], -14, false),
        test_even_inputs_odd_target: (&[2, 4, 6, 2, 8, -2, 10, 12, -24, 8, 12, 18], 3, false),
    }
}
