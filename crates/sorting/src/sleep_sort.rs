use std::sync::mpsc;
use std::thread;
use std::time::Duration;

/// 使用睡眠排序算法对输入的无符号整数切片进行排序。
///
/// 睡眠排序的原理是为每个元素创建一个线程，该线程睡眠与元素值成正比的时间，
/// 然后将元素发送到通道中。由于操作系统调度和睡眠时间的不同，较小的元素
/// 会先完成睡眠并被接收，从而实现排序效果。
///
/// # 参数
/// * `arr` - 需要排序的无符号整数切片引用
///
/// # 返回值
/// 返回一个按升序排列的无符号整数向量
///
/// # 注意事项
/// - 该算法的时间复杂度取决于输入值的大小，不适合处理大数值
/// - 算法依赖线程调度，可能在不同系统上有不同的表现
#[allow(unused)]
pub fn sleep_sort(arr: &[usize]) -> Vec<usize> {
    let len = arr.len();
    let (tx, rx) = mpsc::channel();

    // 为每个元素创建线程，线程睡眠时间与元素值成正比
    arr.iter().enumerate().for_each(|(idx, &val)| {
        let tx = tx.clone();
        thread::spawn(move || {
            thread::sleep(Duration::from_millis((20 * val) as u64));
            tx.send(val).expect("Failed to send value");
        });
    });

    let mut sorted_list: Vec<usize> = Vec::new();

    // 按顺序从通道接收元素，构建排序后的结果
    (0..len).for_each(|_| {
        sorted_list.push(rx.recv().expect("Failed to receive value"));
    });

    sorted_list
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty() {
        let res = sleep_sort(&[]);
        assert_eq!(res, &[]);
    }

    #[test]
    fn single_element() {
        let res = sleep_sort(&[1]);
        assert_eq!(res, &[1]);
    }

    #[test]
    fn sorted_array() {
        let res = sleep_sort(&[1, 2, 3, 4]);
        assert_eq!(res, &[1, 2, 3, 4]);
    }

    #[test]
    fn unsorted_array() {
        let res = sleep_sort(&[3, 4, 2, 1]);
        assert_eq!(res, &[1, 2, 3, 4]);
    }

    #[test]
    fn odd_number_of_elements() {
        let res = sleep_sort(&[3, 1, 7]);
        assert_eq!(res, &[1, 3, 7]);
    }

    #[test]
    fn repeated_elements() {
        let res = sleep_sort(&[1, 1, 1, 1]);
        assert_eq!(res, &[1, 1, 1, 1]);
    }

    #[test]
    fn random_elements() {
        let res = sleep_sort(&[5, 3, 7, 10, 1, 0, 8]);
        assert_eq!(res, &[0, 1, 3, 5, 7, 8, 10]);
    }
}

