use std::ops::{Add, AddAssign, Sub, SubAssign};


#[allow(unused)]
/// 树状数组（Fenwick Tree）数据结构，用于高效处理前缀和与区间更新操作。
///
/// 泛型参数 T 必须支持加法、减法及其赋值操作，并且可以复制和提供默认值。
pub struct FenwickTree<T>
where
    T: Add<Output = T> + AddAssign + Sub<Output = T> + SubAssign + Copy + Default
{
    data: Vec<T>,
}

/// 表示 FenwickTree 操作中可能发生的错误类型。
#[derive(Debug, PartialEq, Eq)]
pub enum FenwickTreeError {
    /// 查询或更新的范围无效。
    InvalidRange,
    /// 索引超出数组边界。
    IndexOutOfBounds,
}

impl<T> FenwickTree<T>
where
    T: Add<Output = T> + AddAssign + Sub<Output = T> + SubAssign + Copy + Default
{
    /// 创建一个具有指定容量的 FenwickTree 实例。
    ///
    /// # 参数
    /// * `capacity` - 树状数组能够容纳的元素数量。
    ///
    /// # 返回值
    /// 返回一个新的 FenwickTree 实例，内部初始化为全零。
    pub fn with_capacity(capacity: usize) -> Self {
        FenwickTree {
            data: vec![T::default(); capacity + 1],
        }
    }

    /// 在指定索引位置增加一个值。
    ///
    /// # 参数
    /// * `index` - 要更新的位置索引（从0开始）。
    /// * `value` - 要增加的值。
    ///
    /// # 返回值
    /// 如果索引合法则返回 Ok(())，否则返回 IndexOutOfBounds 错误。
    pub fn update(&mut self, index: usize, value: T) -> Result<(), FenwickTreeError> {
        if index >= self.data.len() - 1 {
            return Err(FenwickTreeError::IndexOutOfBounds)
        }
        let mut idx = index + 1;
        while idx < self.data.len() {
            self.data[idx] += value;
            idx += low_bit(idx);
        }
        Ok(())
    }

    /// 查询从索引0到指定索引的前缀和。
    ///
    /// # 参数
    /// * `index` - 查询的结束位置索引（包含该位置）。
    ///
    /// # 返回值
    /// 如果索引合法则返回前缀和，否则返回 IndexOutOfBounds 错误。
    pub fn prefix_query(&self, index: usize) -> Result<T, FenwickTreeError> {
        if index >= self.data.len() - 1 {
            return Err(FenwickTreeError::IndexOutOfBounds);
        }
        let mut idx = index + 1;
        let mut result = T::default();
        while idx > 0 {
            result += self.data[idx];
            idx -= low_bit(idx);
        }
        Ok(result)
    }


    /// 查询指定区间的和。
    ///
    /// # 参数
    /// * `left` - 区间起始索引（包含）。
    /// * `right` - 区间结束索引（包含）。
    ///
    /// # 返回值
    /// 如果区间有效则返回区间和，否则返回相应的错误。
    pub fn range_query(&self, left: usize, right: usize) -> Result<T, FenwickTreeError> {
        if left > right || right >= self.data.len() - 1 {
            return Err(FenwickTreeError::InvalidRange);
        }
        let right_query = self.prefix_query(right)?;
        let left_query = if left == 0 {
            T::default()
        } else {
            self.prefix_query(left - 1)?
        };
        Ok(right_query - left_query)
    }

    /// 查询指定位置的单点值。
    ///
    /// # 参数
    /// * `index` - 要查询的位置索引。
    ///
    /// # 返回值
    /// 如果索引合法则返回该位置的值，否则返回 IndexOutOfBounds 错误。
    pub fn point_query(&self, index: usize) -> Result<T, FenwickTreeError> {
        if index >= self.data.len() - 1 {
            return Err(FenwickTreeError::IndexOutOfBounds);
        }
        let index_query = self.prefix_query(index)?;
        let prev_query = if index == 0 {
            T::default()
        } else {
            self.prefix_query(index - 1)?
        };
        Ok(index_query - prev_query)
    }

    /// 将指定位置的值设置为新值。
    ///
    /// # 参数
    /// * `index` - 要设置的位置索引。
    /// * `value` - 新的值。
    ///
    /// # 返回值
    /// 如果索引合法则返回 Ok(())，否则返回 IndexOutOfBounds 错误。
    pub fn set(&mut self, index: usize, value: T) -> Result<(), FenwickTreeError> {
        self.update(index, value - self.point_query(index)?)
    }
}

/// 计算一个整数的最低位1所代表的数值。
///
/// # 参数
/// * `x` - 输入的整数。
///
/// # 返回值
/// 返回 x 的二进制表示中最右边的1所对应的数值。
const fn low_bit(x: usize) -> usize {
    x & (!x + 1)
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fenwick_tree() {
        let mut fenwick_tree = FenwickTree::with_capacity(10);

        assert_eq!(fenwick_tree.update(0, 5), Ok(()));
        assert_eq!(fenwick_tree.update(1, 3), Ok(()));
        assert_eq!(fenwick_tree.update(2, -2), Ok(()));
        assert_eq!(fenwick_tree.update(3, 6), Ok(()));
        assert_eq!(fenwick_tree.update(4, -4), Ok(()));
        assert_eq!(fenwick_tree.update(5, 7), Ok(()));
        assert_eq!(fenwick_tree.update(6, -1), Ok(()));
        assert_eq!(fenwick_tree.update(7, 2), Ok(()));
        assert_eq!(fenwick_tree.update(8, -3), Ok(()));
        assert_eq!(fenwick_tree.update(9, 4), Ok(()));
        assert_eq!(fenwick_tree.set(3, 10), Ok(()));
        assert_eq!(fenwick_tree.point_query(3), Ok(10));
        assert_eq!(fenwick_tree.set(5, 0), Ok(()));
        assert_eq!(fenwick_tree.point_query(5), Ok(0));
        assert_eq!(
            fenwick_tree.update(10, 11),
            Err(FenwickTreeError::IndexOutOfBounds)
        );
        assert_eq!(
            fenwick_tree.set(10, 11),
            Err(FenwickTreeError::IndexOutOfBounds)
        );

        assert_eq!(fenwick_tree.prefix_query(0), Ok(5));
        assert_eq!(fenwick_tree.prefix_query(1), Ok(8));
        assert_eq!(fenwick_tree.prefix_query(2), Ok(6));
        assert_eq!(fenwick_tree.prefix_query(3), Ok(16));
        assert_eq!(fenwick_tree.prefix_query(4), Ok(12));
        assert_eq!(fenwick_tree.prefix_query(5), Ok(12));
        assert_eq!(fenwick_tree.prefix_query(6), Ok(11));
        assert_eq!(fenwick_tree.prefix_query(7), Ok(13));
        assert_eq!(fenwick_tree.prefix_query(8), Ok(10));
        assert_eq!(fenwick_tree.prefix_query(9), Ok(14));
        assert_eq!(
            fenwick_tree.prefix_query(10),
            Err(FenwickTreeError::IndexOutOfBounds)
        );

        assert_eq!(fenwick_tree.range_query(0, 4), Ok(12));
        assert_eq!(fenwick_tree.range_query(3, 7), Ok(7));
        assert_eq!(fenwick_tree.range_query(2, 5), Ok(4));
        assert_eq!(
            fenwick_tree.range_query(4, 3),
            Err(FenwickTreeError::InvalidRange)
        );
        assert_eq!(
            fenwick_tree.range_query(2, 10),
            Err(FenwickTreeError::InvalidRange)
        );

        assert_eq!(fenwick_tree.point_query(0), Ok(5));
        assert_eq!(fenwick_tree.point_query(4), Ok(-4));
        assert_eq!(fenwick_tree.point_query(9), Ok(4));
        assert_eq!(
            fenwick_tree.point_query(10),
            Err(FenwickTreeError::IndexOutOfBounds)
        );
    }
}
