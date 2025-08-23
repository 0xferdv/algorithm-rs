/// 使用三分查找算法在给定区间内寻找函数的最大值。
///
/// 该函数通过不断缩小搜索区间，最终逼近函数的最大值点。
/// 算法要求函数在给定区间内是单峰的（只有一个最大值）。
///
/// # 参数
///
/// * `f` - 要搜索的函数，接受一个 f32 参数并返回 f32 值。
/// * `start` - 搜索区间的起始点。
/// * `end` - 搜索区间的结束点。
/// * `absolute_precision` - 搜索的精度，当区间长度小于此值时停止搜索。
///
/// # 返回值
///
/// 返回找到的函数最大值。
#[allow(unused)]
pub fn ternary_search_max(
    f: fn(f32) -> f32,
    mut start: f32,
    mut end: f32,
    absolute_precision: f32,
) -> f32 {
    // 当搜索区间大于指定精度时继续迭代
    while (start - end).abs() >= absolute_precision {
        // 计算两个三等分点
        let mid1 = start + (end - start) / 3.0;
        let mid2 = end - (end - start) / 3.0;

        // 计算两个三等分点处的函数值
        let r1 = f(mid1);
        let r2 = f(mid2);

        // 根据函数值的比较结果缩小搜索区间
        if r1 < r2 {
            start = mid1;
        } else if r1 > r2 {
            end = mid2;
        } else {
            start = mid1;
            end = mid2;
        }
    }
    f(start)
}

/// 使用三分查找算法在给定区间内寻找函数的最小值。
///
/// 该函数通过不断缩小搜索区间，最终逼近函数的最小值点。
/// 算法要求函数在给定区间内是单峰的（只有一个最小值）。
///
/// # 参数
///
/// * `f` - 要搜索的函数，接受一个 f32 参数并返回 f32 值。
/// * `start` - 搜索区间的起始点。
/// * `end` - 搜索区间的结束点。
/// * `absolute_precision` - 搜索的精度，当区间长度小于此值时停止搜索。
///
/// # 返回值
///
/// 返回找到的函数最小值。
#[allow(unused)]
pub fn ternary_search_min(
    f: fn(f32) -> f32,
    mut start: f32,
    mut end: f32,
    absolute_precision: f32,
) -> f32 {
    // 当搜索区间大于指定精度时继续迭代
    while (start - end).abs() >= absolute_precision {
        // 计算两个三等分点
        let mid1 = start + (end - start) / 3.0;
        let mid2 = end - (end - start) / 3.0;

        // 计算两个三等分点处的函数值
        let r1 = f(mid1);
        let r2 = f(mid2);

        // 根据函数值的比较结果缩小搜索区间
        if r1 < r2 {
            end = mid2;
        } else if r1 > r2 {
            start = mid1;
        } else {
            start = mid1;
            end = mid2;
        }
    }
    f(start)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn finds_max_value() {
        let expected = 4.0;
        let f = |x: f32| -x * x - 2.0 * x + 3.0;

        let start: f32 = -10000000000.0;
        let end: f32 = 10000000000.0;
        let absolute_precision = 0.0000001;

        let result = ternary_search_max(f, start, end, absolute_precision);

        assert_eq!(result, expected);
    }

    #[test]
    fn finds_min_value() {
        let expected = 2.0;
        let f = |x: f32| x * x - 2.0 * x + 3.0;

        let start: f32 = -10000000000.0;
        let end: f32 = 10000000000.0;
        let absolute_precision = 0.0000001;

        let result = ternary_search_min(f, start, end, absolute_precision);

        assert_eq!(result, expected);
    }

    #[test]
    fn finds_max_value_2() {
        let expected = 7.25;
        let f = |x: f32| -x.powi(2) + 3.0 * x + 5.0;

        let start: f32 = -10000000000.0;
        let end: f32 = 10000000000.0;
        let absolute_precision = 0.000001;

        let result = ternary_search_max(f, start, end, absolute_precision);

        assert_eq!(result, expected);
    }

    #[test]
    fn finds_min_value_2() {
        let expected = 2.75;
        let f = |x: f32| x.powi(2) + 3.0 * x + 5.0;

        let start: f32 = -10000000000.0;
        let end: f32 = 10000000000.0;
        let absolute_precision = 0.000001;

        let result = ternary_search_min(f, start, end, absolute_precision);

        assert_eq!(result, expected);
    }
}
