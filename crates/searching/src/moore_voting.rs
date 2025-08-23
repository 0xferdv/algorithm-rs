/// Moore投票算法实现，用于查找数组中的多数元素
/// 
/// 该函数使用Moore投票算法来寻找数组中出现次数超过一半的元素。
/// 算法分为两个阶段：第一阶段通过抵消机制找到候选元素，
/// 第二阶段验证候选元素是否真的出现次数超过一半。
/// 
/// # 参数
/// * `arr` - 输入的整数数组切片
/// 
/// # 返回值
/// 如果存在多数元素，返回该元素；否则返回-1
/// 
/// # 示例
///
/// let arr = vec![9, 1, 8, 1, 1];
/// let result = moore_voting(&arr);
/// assert_eq!(result, 1);
///

#[allow(unused)]
pub fn moore_voting(arr: &[i32]) -> i32 {
    let n = arr.len();
    let mut cnt = 0;
    let mut ele = 0;

    // 第一阶段：使用Moore投票算法找到候选元素
    // 通过计数器的增减来抵消不同元素，最终剩下的可能是多数元素
    for &item in arr.iter() {
        if cnt == 0 {
            cnt = 1;
            ele = item;
        } else if item == ele {
            cnt += 1;
        } else {
            cnt -= 1;
        }
    }

    // 第二阶段：验证候选元素是否真的出现次数超过一半
    let cnt_check = arr.iter().filter(|&&val| val == ele).count();
    if cnt_check > (n / 2) {
        ele
    } else {
        -1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_moore_voting() {
        let arr1: Vec<i32> = vec![9, 1, 8, 1, 1];
        assert!(moore_voting(&arr1) == 1);
        let arr2: Vec<i32> = vec![1, 2, 3, 4];
        assert!(moore_voting(&arr2) == -1);
    }
}
