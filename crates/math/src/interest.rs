#![allow(unused)]

use std::f64::consts::E;

/// 计算简单利息和未来价值
///
/// # 参数
/// * `principal` - 本金金额
/// * `annual_rate` - 年利率（以小数形式表示，例如0.05表示5%）
/// * `years` - 投资年数
///
/// # 返回值
/// 返回一个元组，包含：
/// * 第一个元素：获得的利息金额
/// * 第二个元素：未来价值（本金+利息）
///
/// # 示例
///
/// let (interest, future_value) = simple_interest(1000.0, 0.05, 2.0);
///

pub fn simple_interest(principal: f64, annual_rate: f64, years: f64) -> (f64, f64) {
    let interest = principal * annual_rate * years;
    let value = principal * (1.0 + (annual_rate * years));
    println!("Interest earned: {interest}");
    println!("Future value: {value}");
    (interest, value)
}

/// 计算复利未来价值
///
/// # 参数
/// * `principal` - 本金金额
/// * `annual_rate` - 年利率（以小数形式表示）
/// * `years` - 投资年数
/// * `period` - 每年复利期数，如果为None则计算连续复利
///
/// # 返回值
/// 返回复利计算后的未来价值
///
/// # 说明
/// 当period为None时，使用自然常数e计算连续复利
/// 当period为Some值时，使用标准复利公式计算，但若period小于等于0则返回NaN
pub fn compound_interest(principal: f64, annual_rate: f64, years: f64, period: Option<f64>) -> f64 {
    // 根据是否提供复利期数选择计算方式
    let value = if period.is_none() {
        // 连续复利计算：P * e^(r*t)
        principal * E.powf(annual_rate * years)
    } else {
        let prim_period: f64 = period.unwrap_or(0.0);
        // 检查复利期数是否有效
        if prim_period <= 0.0_f64 {
            return f64::NAN;
        }
        // 标准复利计算：P * (1 + r/n)^(n*t)
        principal * (1.0 + (annual_rate / prim_period).powf(prim_period * years))
    };
    println!("Future value: {value}");
    value
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple() {
        let x = 385.65_f64 * 0.03_f64 * 5.0_f64;
        let y = 385.65_f64 * (1.0 + (0.03_f64 * 5.0_f64));
        assert_eq!(simple_interest(385.65_f64, 0.03_f64, 5.0_f64), (x, y));
    }

    #[test]
    fn test_compounding() {
        let x = 385.65_f64 * E.powf(0.03_f64 * 5.0_f64);
        assert_eq!(compound_interest(385.65_f64, 0.03_f64, 5.0_f64, None), x);

        let y = 385.65_f64 * (1.0 + (0.03_f64 / 5.0_f64).powf(5.0_f64 * 5.0_f64));
        assert_eq!(
            compound_interest(385.65_f64, 0.03_f64, 5.0_f64, Some(5.0_f64)),
            y
        );
        assert!(compound_interest(385.65_f64, 0.03_f64, 5.0_f64, Some(-5.0_f64)).is_nan());
        assert!(compound_interest(385.65_f64, 0.03_f64, 5.0_f64, Some(0.0_f64)).is_nan());
    }
}
