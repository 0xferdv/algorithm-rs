use std::f64::consts::PI;

/// 地球半径常量，单位为米
const EARTH_RADIUS: f64 = 6371000.00;


/// 使用 Haversine 公式计算两个经纬度坐标点之间的直线距离
///
/// Haversine 公式用于计算球面上两点间的最短距离（大圆距离）
///
/// # 参数
/// * `lat1` - 第一个点的纬度（十进制度数）
/// * `lng1` - 第一个点的经度（十进制度数）
/// * `lat2` - 第二个点的纬度（十进制度数）
/// * `lng2` - 第二个点的经度（十进制度数）
///
/// # 返回值
/// 返回两个坐标点之间的直线距离，单位为米
///
/// # 算法说明
/// 1. 将经纬度从角度转换为弧度
/// 2. 计算纬度和经度的差值
/// 3. 应用 Haversine 公式计算中心角
/// 4. 通过地球半径计算实际距离
#[allow(unused)]
pub fn haversine(lat1: f64, lng1: f64, lat2: f64, lng2: f64) -> f64 {
    // 将经纬度差值从角度转换为弧度
    let delta_dist_lat = (lat2 - lat1) * PI / 180.0;
    let delta_dist_lng = (lng2 - lng1) * PI / 180.0;

    // 将纬度从角度转换为弧度
    let cos1 = lat1 * PI / 180.0;
    let cos2 = lat2 * PI / 180.0;

    // 计算 Haversine 公式中的中间值
    let delta_lat = (delta_dist_lat / 2.0).sin().powf(2.0);
    let delta_lng = (delta_dist_lng / 2.0).sin().powf(2.0);

    // 应用 Haversine 公式计算球面距离
    let a = delta_lat + delta_lng * cos1.cos() * cos2.cos();
    let result = 2.0 * a.asin().sqrt();

    // 通过地球半径将中心角转换为实际距离
    result * EARTH_RADIUS
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn testing() {
        assert_eq!(
            format!(
                "{:.2}km",
                haversine(52.375603, 4.903206, 52.366059, 4.926692) / 1000.0
            ),
            "1.92km"
        );
    }
}
