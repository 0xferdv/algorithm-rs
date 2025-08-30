/// T数组用于Doomsday算法中每个月的偏移量计算
/// 数组索引对应月份-1，值为该月相对于该年Doomsday的偏移天数
const T: [i32; 12] = [0, 3, 2, 5, 0, 3, 5, 1, 4, 6, 2, 4];

/// 使用Doomsday算法计算给定日期对应的星期数
///
/// # 参数
/// * `year` - 年份
/// * `month` - 月份 (1-12)
/// * `day` - 日期 (1-31)
///
/// # 返回值
/// 返回0-6的数字，其中0表示星期日，1表示星期一，依此类推
///
/// # 算法说明
/// 该算法通过计算给定日期与该年Doomsday的差值来确定星期几
/// 对于1月和2月，需要将年份减1进行计算，因为Doomsday算法将它们视为上一年的月份
#[allow(unused)]
pub fn doomsday(year: i32, month: i32, day: i32) -> i32 {
    // 如果是1月或2月，将年份减1进行计算
    let year = if month < 3 { year - 1 } else { year };
    // 使用Doomsday公式计算星期数
    (year + year / 4 - year / 100 + year / 400 + T[(month - 1) as usize] + day) % 7
}

/// 将数字星期转换为英文星期名称
///
/// # 参数
/// * `year` - 年份
/// * `month` - 月份 (1-12)
/// * `day` - 日期 (1-31)
///
/// # 返回值
/// 返回对应的英文星期名称字符串
pub fn get_week_day(year: i32, month: i32, day: i32) -> String {
    // 调用doomsday函数获取数字形式的星期
    let day = doomsday(year, month, day);
    // 将数字星期转换为英文名称
    let day_str = match day {
        0 => "Sunday",
        1 => "Monday",
        2 => "Tuesday",
        3 => "Wednesday",
        4 => "Thursday",
        5 => "Friday",
        6 => "Saturday",
        _ => "Unknown",
    };

    day_str.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    /// 测试Doomsday算法的正确性
    /// 使用已知日期验证算法结果是否正确
    #[test]
    fn doomsday_test() {
        assert_eq!(get_week_day(1990, 3, 21), "Wednesday");
        assert_eq!(get_week_day(2000, 8, 24), "Thursday");
        assert_eq!(get_week_day(2000, 10, 13), "Friday");
        assert_eq!(get_week_day(2001, 4, 18), "Wednesday");
        assert_eq!(get_week_day(2002, 3, 19), "Tuesday");
    }
}
