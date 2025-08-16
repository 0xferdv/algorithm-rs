#[derive(PartialOrd, PartialEq, Eq)]
#[allow(unused)]
pub enum Colors {
    Red,
    White,
    Blue,
}

use Colors::{Red, White, Blue};

#[allow(unused)]
pub fn dutch_national_flag_sort(mut sequences: Vec<Colors>) -> Vec<Colors> {
    let length = sequences.len();
    if length <= 1 {
        return sequences;
    }
    let mut low = 0;
    let mut mid = 0;
    let mut high = length - 1;
    while mid <= high {
        match sequences[mid] {
            Red => {
                sequences.swap(low, mid);
                low += 1;
                mid += 1;
            }
            White => {
                mid += 1;
            }
            Blue => {
                sequences.swap(mid, high);
                high -= 1;
            }
        }
    }
    sequences
}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::is_sorted;

    #[test]
    fn random_array() {
        let arr = vec![
            Red, Blue, White, White, Blue, Blue, Red, Red, White, Blue, White, Red, White, Blue,
        ];
        let arr = dutch_national_flag_sort(arr);
        assert!(is_sorted(&arr))
    }

    #[test]
    fn sorted_array() {
        let arr = vec![
            Red, Red, Red, Red, Red, White, White, White, White, White, Blue, Blue, Blue, Blue,
        ];
        let arr = dutch_national_flag_sort(arr);
        assert!(is_sorted(&arr))
    }
}
