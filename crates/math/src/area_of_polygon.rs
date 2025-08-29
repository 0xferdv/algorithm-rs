/// 表示二维平面上的一个点。
///
/// 包含 x 和 y 两个坐标值，均为 f64 类型。
pub struct Point {
    x: f64,
    y: f64,
}

/// 计算由一系列点构成的多边形的面积。
///
/// 使用鞋带公式（Shoelace formula）来计算简单多边形的面积。
/// 多边形的顶点应按顺序给出，可以是顺时针或逆时针方向。
///
/// # 参数
/// * `points` - 一个包含 Point 结构体的切片，表示多边形的各个顶点
///
/// # 返回值
/// 返回多边形的面积，类型为 f64。如果点的数量少于3，则结果可能无意义。
#[allow(unused)]
pub fn area_of_polygon(points: &[Point]) -> f64 {
    let mut res = 0.0;
    // 遍历所有顶点，应用鞋带公式计算面积
    for idx in 0..points.len() {
        // 获取当前点的前一个点（首尾相连）
        let p  = if idx > 0 {
            &points[idx - 1]
        } else {
            &points[points.len() - 1]
        };
        // 获取当前点
        let q = &points[idx];
        // 累加面积计算中的交叉乘积项
        res += (p.x - q.x) * (p.y + q.y);
    }
    // 取绝对值并除以2得到最终面积
    f64::abs(res) / 2.0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_area_triangle() {
        let points = vec![
            Point { x: 0.0, y: 0.0 },
            Point { x: 1.0, y: 0.0 },
            Point { x: 0.0, y: 1.0 },
        ];

        assert_eq!(area_of_polygon(&points), 0.5);
    }

    #[test]
    fn test_area_square() {
        let points = vec![
            Point { x: 0.0, y: 0.0 },
            Point { x: 1.0, y: 0.0 },
            Point { x: 1.0, y: 1.0 },
            Point { x: 0.0, y: 1.0 },
        ];

        assert_eq!(area_of_polygon(&points), 1.0);
    }

    #[test]
    fn test_area_hexagon() {
        let points = vec![
            Point { x: 0.0, y: 0.0 },
            Point { x: 1.0, y: 0.0 },
            Point { x: 1.5, y: 0.866 },
            Point { x: 1.0, y: 1.732 },
            Point { x: 0.0, y: 1.732 },
            Point { x: -0.5, y: 0.866 },
        ];

        assert_eq!(area_of_polygon(&points), 2.598);
    }
}
