mod bead_sort;
mod binary_insertion_sort;
mod bingo_sort;
mod bitonic_sort;
mod bubble_sort;
mod bucket_sort;
pub mod insertion_sort;
mod cocktail_shaker_sort;
mod comb_sort;
mod counting_sort;

#[cfg(test)]
/// 检查两个切片是否包含相同的元素（不考虑顺序）
///
/// # 参数
/// * `a` - 第一个元素切片
/// * `b` - 第二个元素切片
///
/// # 返回值
/// 如果两个切片包含相同的元素则返回true，否则返回false
///
/// # 约束条件
/// 元素类型T必须实现PartialOrd、Eq和Hash trait
pub fn have_same_elements<T>(a: &[T], b: &[T]) -> bool
where
    T: PartialOrd + Eq + std::hash::Hash
{
    use std::collections::HashSet;
    if a.len() == b.len() {
        // 使用HashSet实现O(n)时间复杂度的比较，适用于大数据集
        let set_a: HashSet<_> = a.iter().collect();
        let set_b: HashSet<_> = b.iter().collect();
        set_a == set_b
    } else {
        false
    }
}

#[cfg(test)]
/// 检查切片是否按升序排列
///
/// # 参数
/// * `arr` - 要检查的元素切片
///
/// # 返回值
/// 如果切片按升序排列则返回true，否则返回false
///
/// # 约束条件
/// 元素类型T必须实现PartialOrd trait
pub fn is_sorted<T>(arr: &[T]) -> bool
where
    T: PartialOrd
{
    arr.windows(2).all(|w| w[0] <= w[1])
}

#[cfg(test)]
/// 检查切片是否按降序排列
///
/// # 参数
/// * `arr` - 要检查的元素切片
///
/// # 返回值
/// 如果切片按降序排列则返回true，否则返回false
///
/// # 约束条件
/// 元素类型T必须实现PartialOrd trait
pub fn is_descending_sorted<T>(arr: &[T]) -> bool
where
    T: PartialOrd
{
    arr.windows(2).all(|w| w[0] <= w[1])
}
