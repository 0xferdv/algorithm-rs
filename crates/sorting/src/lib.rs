pub mod bead_sort;
pub use bead_sort::*;

pub mod binary_insertion_sort;
pub use binary_insertion_sort::*;

pub mod bingo_sort;
pub use bingo_sort::*;

pub mod bitonic_sort;
pub use bitonic_sort::*;

pub mod bubble_sort;
pub use bubble_sort::*;

pub mod bucket_sort;
pub use bucket_sort::*;

pub mod insertion_sort;
pub use insertion_sort::*;

pub mod cocktail_shaker_sort;
pub use cocktail_shaker_sort::*;

pub mod comb_sort;
pub use comb_sort::*;

pub mod counting_sort;
pub use counting_sort::*;

pub mod cycle_sort;
pub use cycle_sort::*;

pub mod dutch_national_flag_sort;
pub use dutch_national_flag_sort::*;

pub mod exchange_sort;
pub use exchange_sort::*;

pub mod gnome_sort;
pub use gnome_sort::*;

pub mod heap_sort;
pub use heap_sort::*;

pub mod intro_sort;
pub use intro_sort::*;

pub mod merge_sort;
pub use merge_sort::*;

pub mod odd_even_sort;
pub use odd_even_sort::*;

pub mod pancake_sort;
pub use pancake_sort::*;


pub mod patience_sort;
pub use patience_sort::*;

pub mod pigeonhole_sort;
pub use pigeonhole_sort::*;

pub mod quick_sort;
pub use quick_sort::*;

pub mod quick_sort_3_ways;
pub use quick_sort_3_ways::*;

pub mod radix_sort;
pub use radix_sort::*;

pub mod selection_sort;
pub use selection_sort::*;

pub mod shell_sort;
pub use shell_sort::*;

pub mod sleep_sort;
pub use sleep_sort::*;

pub mod sort_util;
#[allow(unused)]
pub use sort_util::*;

pub mod stooge_sort;
pub use stooge_sort::*;

pub mod tim_sort;
pub use tim_sort::*;

pub mod tree_sort;
pub use tree_sort::*;

pub mod wave_sort;
pub use wave_sort::*;

pub mod wiggle_sort;
pub use wiggle_sort::*;


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
