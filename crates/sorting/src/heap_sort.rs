use std::cmp::Ordering;

fn build_heap<T: Ord>(arr: &mut [T], is_max_heap: bool) {
    let mut mid = arr.len() / 2 - 1;
    while 0 < mid {
        update_heap(arr, mid, is_max_heap);
        mid -= 1;
    }
    update_heap(arr, 0, is_max_heap);
}


fn update_heap<T: Ord>(arr: &mut [T], i: usize, is_max_heap: bool) {
    let cmp: fn(&T, &T) -> Ordering = if !is_max_heap {
        |a, b| b.cmp(a)
    } else {
        |a, b| a.cmp(b)
    };
    let mut idx = i;
    let left = 2 * i + 1;
    let right = 2 * i + 2;
    if left < arr.len() && cmp(&arr[left], &arr[idx]) == Ordering::Greater {
        idx = left;
    }
    if right < arr.len() && cmp(&arr[right], &arr[idx]) == Ordering::Greater {
        idx = right;
    }
    if idx != i {
        arr.swap(i, idx);
        update_heap(arr, idx, is_max_heap);
    }
}


#[allow(unused)]
pub fn heap_sort<T: Ord>(arr: &mut [T], ascending: bool) {
    let len = arr.len();
    if len <= 1 {
        return;
    }
    build_heap(arr, ascending);
    let mut end = len - 1;
    while end > 0 {
        arr.swap(0, end);
        update_heap(&mut arr[0..end], 0, ascending);
        end -= 1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{have_same_elements, is_sorted};

    macro_rules! test_heap_sort {
        ($($name:ident: $input:expr,)*) => {
            $(
                #[test]
                fn $name() {
                    let input_array = $input;
                    let mut arr_asc = input_array.clone();
                    heap_sort(&mut arr_asc, true);
                    assert!(is_sorted(&arr_asc) && have_same_elements(&arr_asc, &input_array));
                }
            )*
        }
    }

    test_heap_sort! {
        empty_array: Vec::<i32>::new(),
        single_element_array: vec![5],
        sorted: vec![1, 2, 3, 4, 5],
        sorted_desc: vec![5, 4, 3, 2, 1, 0],
        basic_0: vec![9, 8, 7, 6, 5],
        basic_1: vec![8, 3, 1, 5, 7],
        basic_2: vec![4, 5, 7, 1, 2, 3, 2, 8, 5, 4, 9, 9, 100, 1, 2, 3, 6, 4, 3],
        duplicated_elements: vec![5, 5, 5, 5, 5],
    }
}
