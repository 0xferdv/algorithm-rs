use std::cmp::PartialOrd;

/// 错误类型，用于表示范围查询中的错误情况
#[derive(Debug, PartialEq, Eq)]
pub enum RangeError {
    /// 表示无效的范围（如 start >= end）
    InvalidRange,
    /// 表示索引超出数据边界
    IndexOutOfBounds,
}

/// 范围最小值查询结构体，使用稀疏表实现，支持 O(1) 时间复杂度的范围最小值查询
pub struct RangeMinimumQuery<T: PartialOrd + Copy> {
    /// 原始输入数据
    data: Vec<T>,
    /// 稀疏表，用于快速查找范围内的最小值索引
    sparse_table: Vec<Vec<usize>>,
}

impl<T> RangeMinimumQuery<T>
where
    T: PartialOrd + Copy
{
    /// 创建一个新的 RangeMinimumQuery 实例
    ///
    /// # 参数
    /// * `input` - 输入的数据切片，用于构建稀疏表
    ///
    /// # 返回值
    /// 返回一个新的 RangeMinimumQuery 实例
    pub fn new(input: &[T]) -> RangeMinimumQuery<T> {
        RangeMinimumQuery {
            data: input.to_vec(),
            sparse_table: build_sparse_table(input),
        }
    }

    /// 查询指定范围 [start, end) 内的最小值
    ///
    /// # 参数
    /// * `start` - 查询范围的起始索引（包含）
    /// * `end` - 查询范围的结束索引（不包含）
    ///
    /// # 返回值
    /// 如果查询成功，返回范围内的最小值；否则返回相应的错误
    ///
    /// # 错误
    /// * `RangeError::InvalidRange` - 当 start >= end 时
    /// * `RangeError::IndexOutOfBounds` - 当索引超出数据边界时
    pub fn get_range_min(&self, start: usize, end: usize) -> Result<T, RangeError> {
        if start >= end {
            return Err(RangeError::InvalidRange);
        }
        if start >= self.data.len() || end > self.data.len() {
            return Err(RangeError::IndexOutOfBounds);
        }

        // 计算当前区间的长度对应的 log 值
        let log_len = (end - start).ilog2() as usize;
        // 计算第二个比较区间的起始位置
        let idx: usize = end - (1 << log_len);
        // 获取两个区间的最小值索引
        let min_idx_start = self.sparse_table[log_len][start];
        let min_idx_end = self.sparse_table[log_len][idx];

        // 比较两个索引对应的值，返回较小的那个
        if self.data[min_idx_start] < self.data[min_idx_end] {
            Ok(self.data[min_idx_start])
        } else {
            Ok(self.data[min_idx_end])
        }
    }
}

/// 构建稀疏表，用于范围最小值查询
///
/// # 参数
/// * `data` - 输入的数据切片
///
/// # 返回值
/// 返回构建好的稀疏表，其中每个元素表示对应区间内的最小值索引
fn build_sparse_table<T: PartialOrd>(data: &[T]) -> Vec<Vec<usize>> {
    // 初始化稀疏表的第一层，即每个单元素区间的最小值索引
    let mut sparse_table: Vec<Vec<usize>> = vec![(0..data.len()).collect()];
    let len = data.len();

    // 构建稀疏表的每一层
    for log_len in 1..=len.ilog2() {
        let mut row = Vec::new();
        // 对于当前层的每个区间
        for idx in 0..=len - (1 << log_len) {
            // 获取前一层中两个子区间的最小值索引
            let min_idx_start = sparse_table[sparse_table.len() - 1][idx];
            let min_idx_end = sparse_table[sparse_table.len() - 1][idx + (1 << (log_len - 1))];
            // 比较两个索引对应的值，选择较小的那个作为当前区间的最小值索引
            if data[min_idx_start] < data[min_idx_end] {
                row.push(min_idx_start);
            } else {
                row.push(min_idx_end);
            }
        }
        sparse_table.push(row);
    }

    sparse_table
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! test_build_sparse_table {
        ($($name:ident: $inputs:expr,)*) => {
            $(
                #[test]
                fn $name() {
                    let (data, expected) = $inputs;
                    assert_eq!(build_sparse_table(&data), expected);
                }
            )*
        }
    }

    test_build_sparse_table! {
        small: (
            [1, 6, 3],
            vec![
                vec![0, 1, 2],
                vec![0, 2]
            ]
        ),
        medium: (
            [1, 3, 6, 123, 7, 235, 3, -4, 6, 2],
            vec![
                vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9],
                vec![0, 1, 2, 4, 4, 6, 7, 7, 9],
                vec![0, 1, 2, 6, 7, 7, 7],
                vec![7, 7, 7]
            ]
        ),
        large: (
            [20, 13, -13, 2, 3634, -2, 56, 3, 67, 8, 23, 0, -23, 1, 5, 85, 3, 24, 5, -10, 3, 4, 20],
            vec![
                vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22],
                vec![1, 2, 2, 3, 5, 5, 7, 7, 9, 9, 11, 12, 12, 13, 14, 16, 16, 18, 19, 19, 20, 21],
                vec![2, 2, 2, 5, 5, 5, 7, 7, 11, 12, 12, 12, 12, 13, 16, 16, 19, 19, 19, 19],
                vec![2, 2, 2, 5, 5, 12, 12, 12, 12, 12, 12, 12, 12, 19, 19, 19],
                vec![12, 12, 12, 12, 12, 12, 12, 12]
            ]
        ),
    }

    #[test]
    fn simple_query_tests() {
        let rmq = RangeMinimumQuery::new(&[1, 3, 6, 123, 7, 235, 3, -4, 6, 2]);

        assert_eq!(rmq.get_range_min(1, 6), Ok(3));
        assert_eq!(rmq.get_range_min(0, 10), Ok(-4));
        assert_eq!(rmq.get_range_min(8, 9), Ok(6));
        assert_eq!(rmq.get_range_min(4, 3), Err(RangeError::InvalidRange));
        assert_eq!(rmq.get_range_min(0, 1000), Err(RangeError::IndexOutOfBounds));
        assert_eq!(
            rmq.get_range_min(1000, 1001),
            Err(RangeError::IndexOutOfBounds)
        );
    }

    #[test]
    fn float_query_tests() {
        let rmq = RangeMinimumQuery::new(&[0.4, -2.3, 0.0, 234.22, 12.2, -3.0]);

        assert_eq!(rmq.get_range_min(0, 6), Ok(-3.0));
        assert_eq!(rmq.get_range_min(0, 4), Ok(-2.3));
        assert_eq!(rmq.get_range_min(3, 5), Ok(12.2));
        assert_eq!(rmq.get_range_min(2, 3), Ok(0.0));
        assert_eq!(rmq.get_range_min(4, 3), Err(RangeError::InvalidRange));
        assert_eq!(rmq.get_range_min(0, 1000), Err(RangeError::IndexOutOfBounds));
        assert_eq!(
            rmq.get_range_min(1000, 1001),
            Err(RangeError::IndexOutOfBounds)
        );
    }
}
