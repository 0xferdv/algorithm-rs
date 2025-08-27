use std::f64::consts::PI;

/// 计算两个经纬度坐标点之间的方位角（bearing）
///
/// 方位角是从第一个点指向第二个点的方向角度，以正北为0度，顺时针方向测量
/// 使用球面三角公式计算大圆航线的初始方位角
///
/// # 参数
/// * `lat1` - 第一个点的纬度（十进制度数）
/// * `lng1` - 第一个点的经度（十进制度数）
/// * `lat2` - 第二个点的纬度（十进制度数）
/// * `lng2` - 第二个点的经度（十进制度数）
///
/// # 返回值
/// 返回从第一个点到第二个点的方位角，范围为0-360度
///
/// # 算法说明
/// 1. 将经纬度从角度转换为弧度
/// 2. 使用球面三角公式计算方位角
/// 3. 将结果转换回角度并规范化到0-360度范围
#[allow(unused)]
pub fn bearing(lat1: f64, lng1: f64, lat2: f64, lng2: f64) -> f64 {
    // 将经纬度从角度转换为弧度
    let lat1 = lat1 * PI / 180.0;
    let lng1 = lng1 * PI / 180.0;
    let lat2 = lat2 * PI / 180.0;
    let lng2 = lng2 * PI / 180.0;

    // 计算经度差值
    let delta_longitude = lng2 - lng1;

    // 使用球面三角公式计算方位角的y和x分量
    let y = delta_longitude.sin() * lat2.cos();
    let x = lat1.cos() * lat2.sin() - lat1.sin() * lat2.cos() * delta_longitude.cos();

    // 计算方位角并转换为角度
    let mut bng = y.atan2(x);
    bng = bng.to_degrees();

    // 将方位角规范化到0-360度范围
    (bng + 360.0) % 360.0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn testing() {
        assert_eq!(
            format!(
                "{:.0}º",
                bearing(
                    -27.2020447088982,
                    -49.631891179172555,
                    -3.106362,
                    -60.025826,
                )
            ),
            "336º"
        );
    }
}
