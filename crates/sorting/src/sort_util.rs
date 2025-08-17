#[allow(unused)]
use rand::Rng;

#[allow(unused)]
use std::time::Instant;

/// 生成一个包含随机整数的向量。
///
/// # 参数
/// * `n` - 向量中元素的数量
/// * `range_l` - 随机数的最小值（包含）
/// * `range_r` - 随机数的最大值（包含）
///
/// # 返回值
/// 返回一个包含 `n` 范围内随机整数的向量
#[cfg(test)]
pub fn generate_random_vec(n: u32, range_l: i32, range_r: i32) -> Vec<i32> {
    let mut arr = Vec::<i32>::with_capacity(n as usize);
    let mut rng = rand::rng();
    let mut count = n;
    // 循环生成随机数并添加到向量中
    while count > 0 {
        arr.push(rng.random_range(range_l..=range_r));
        count -= 1;
    }
    arr
}

/// 生成一个近似有序的向量，通过随机交换元素实现。
///
/// # 参数
/// * `n` - 向量中元素的数量
/// * `swap_times` - 随机交换的次数
///
/// # 返回值
/// 返回一个近似有序的向量，其中包含从 0 到 n-1 的整数，但经过 `swap_times` 次随机交换
#[cfg(test)]
pub fn generate_nearly_ordered_vec(n: u32, swap_times: u32) -> Vec<i32> {
    // 创建一个从 0 到 n-1 的有序向量
    let mut arr: Vec<i32> = (0..n as i32).collect();
    let mut rng = rand::rng();
    let mut count = swap_times;
    // 随机交换指定次数的元素
    while count > 0 {
        arr.swap(
            rng.random_range(0..n as usize),
            rng.random_range(0..n as usize),
        );
        count -= 1;
    }
    arr
}

/// 生成一个完全有序的向量。
///
/// # 参数
/// * `n`- 向量中元素的数量
///
/// # 返回值
/// 返回一个包含从 0 到 n-1 的有序整数向量
#[cfg(test)]
pub fn generate_ordered_vec(n: u32) -> Vec<i32> {
    generate_nearly_ordered_vec(n, 0)
}


/// 生成一个完全逆序的向量。
///
/// # 参数
/// * `n` - 向量中元素的数量
///
/// # 返回值
/// 返回一个包含从 n-1 到 0 的逆序整数向量
#[cfg(test)]
pub fn generate_reversed_ordered_vec(n: u32) -> Vec<i32> {
    let mut arr = generate_ordered_vec(n);
    // 将有序向量反转得到逆序向量
    arr.reverse();
    arr
}

/// 生成一个包含重复元素的向量。
///
/// # 参数
/// * `n` - 向量中元素的数量
/// * `unique_elements` - 不同元素的最大数量
///
/// # 返回值
/// 返回一个包含 `n` 个元素的向量，其中只有 `unique_elements` 个不同的值
#[cfg(test)]
pub fn generate_repeated_elements_vec(n: u32, unique_elements: u8) -> Vec<i32> {
    let mut rng = rand::rng();
    let v = rng.random_range(0..n as i32);
    // 生成一个只包含指定范围内值的向量
    generate_random_vec(n, v, v + unique_elements as i32)
}

/// 执行一个函数并记录其执行时间。
///
/// # 参数
/// * `test_name` - 测试的名称，用于输出时间日志
/// * `f` - 要执行的函数
///
/// # 泛型
/// * `F` - 任意只执行一次的函数类型
#[cfg(test)]
pub fn log_timed<F>(test_name: &str, f: F)
where
    F: FnOnce()
{
    let before = Instant::now();
    f();
    let after = Instant::now();
    // 输出测试名称和执行时间
    println!("{}: {:?}", test_name, after - before);
}

