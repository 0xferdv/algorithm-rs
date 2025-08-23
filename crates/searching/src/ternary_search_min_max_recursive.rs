/// 使用递归三分查找法在给定区间内寻找函数的最大值。
///
/// 该函数假设目标函数在给定区间 `[start, end]` 内是单峰的（只有一个最大值），
/// 并通过不断缩小搜索区间来逼近最大值点。
///
/// # 参数
/// - `f`: 要查找最大值的目标函数，接受一个 `f32` 参数并返回 `f32` 值。
/// - `start`: 搜索区间的起始点。
/// - `end`: 搜索区间的结束点。
/// - `absolute_precision`: 查找精度，当区间长度小于此值时停止搜索。
///
/// # 返回值
/// 返回找到的最大值点对应的函数值。
#[allow(unused)]
pub fn ternary_search_max_rec(
    f: fn(f32) -> f32,
    start: f32,
    end: f32,
    absolute_precision: f32,
) -> f32 {
    // 当前搜索区间长度大于精度要求时继续细分
    if (end - start).abs() >= absolute_precision {
        // 将区间三等分，取两个三等分点
        let mid1 = start + (end - start) / 3.0;
        let mid2 = end - (end - start) / 3.0;

        // 计算两个三等分点处的函数值
        let r1 = f(mid1);
        let r2 = f(mid2);

        // 根据函数值比较结果决定下一步搜索区间
        if r1 < r2 {
            // 如果左三等分点函数值小于右三等分点，则最大值在右半部分
            return ternary_search_max_rec(f, mid1, end, absolute_precision);
        } else if r1 > r2 {
            // 如果左三等分点函数值大于右三等分点，则最大值在左半部分
            return ternary_search_max_rec(f, start, mid2, absolute_precision);
        }
        // 如果两点函数值相等，则最大值在中间部分
        return ternary_search_max_rec(f, mid1, mid2, absolute_precision);
    }
    // 区间足够小时，返回起点处的函数值作为近似最大值
    f(start)
}

/// 使用递归三分查找法在给定区间内寻找函数的最小值。
///
/// 该函数假设目标函数在给定区间 `[start, end]` 内是单峰的（只有一个最小值），
/// 并通过不断缩小搜索区间来逼近最小值点。
///
/// # 参数
/// - `f`: 要查找最小值的目标函数，接受一个 `f32` 参数并返回 `f32` 值。
/// - `start`: 搜索区间的起始点。
/// - `end`: 搜索区间的结束点。
/// - `absolute_precision`: 查找精度，当区间长度小于此值时停止搜索。
///
/// # 返回值
/// 返回找到的最小值点对应的函数值。
#[allow(unused)]
pub fn ternary_search_min_rec(
    f: fn(f32) -> f32,
    start: f32,
    end: f32,
    absolute_precision: f32,
) -> f32 {
    // 当前搜索区间长度大于精度要求时继续细分
    if (end - start).abs() >= absolute_precision {
        // 将区间三等分，取两个三等分点
        let mid1 = start + (end - start) / 3.0;
        let mid2 = end - (end - start) / 3.0;

        // 计算两个三等分点处的函数值
        let r1 = f(mid1);
        let r2 = f(mid2);

        // 根据函数值比较结果决定下一步搜索区间
        if r1 < r2 {
            // 如果左三等分点函数值小于右三等分点，则最小值在左半部分
            return ternary_search_min_rec(f, start, mid2, absolute_precision);
        } else if r1 > r2 {
            // 如果左三等分点函数值大于右三等分点，则最小值在右半部分
            return ternary_search_min_rec(f, mid1, end, absolute_precision);
        }
        // 如果两点函数值相等，则最小值在中间部分
        return ternary_search_min_rec(f, mid1, mid2, absolute_precision);
    }
    // 区间足够小时，返回起点处的函数值作为近似最小值
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

        let result = ternary_search_max_rec(f, start, end, absolute_precision);

        assert_eq!(result, expected);
    }

    #[test]
    fn finds_min_value() {
        let expected = 2.0;
        let f = |x: f32| x * x - 2.0 * x + 3.0;

        let start: f32 = -10000000000.0;
        let end: f32 = 10000000000.0;
        let absolute_precision = 0.0000001;

        let result = ternary_search_min_rec(f, start, end, absolute_precision);

        assert_eq!(result, expected);
    }

    #[test]
    fn finds_max_value_2() {
        let expected = 7.25;
        let f = |x: f32| -x.powi(2) + 3.0 * x + 5.0;

        let start: f32 = -10000000000.0;
        let end: f32 = 10000000000.0;
        let absolute_precision = 0.000001;

        let result = ternary_search_max_rec(f, start, end, absolute_precision);

        assert_eq!(result, expected);
    }

    #[test]
    fn finds_min_value_2() {
        let expected = 2.75;
        let f = |x: f32| x.powi(2) + 3.0 * x + 5.0;

        let start: f32 = -10000000000.0;
        let end: f32 = 10000000000.0;
        let absolute_precision = 0.000001;

        let result = ternary_search_min_rec(f, start, end, absolute_precision);

        assert_eq!(result, expected);
    }
}
