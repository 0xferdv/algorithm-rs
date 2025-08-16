#[allow(unused)]
pub fn cocktail_shaker_sort<T: Ord>(arr: &mut [T]) {
    let len = arr.len();
    if len == 0 {
        return;
    }
    loop {
        let mut swapped = false;
        (0..(len - 1).clamp(0, len)).for_each(|i| {
            if arr[i] > arr[i + 1] {
                arr.swap(i, i + 1);
                swapped = true;
            }
        });
        if !swapped {
            break;
        }
        swapped = false;
        (1..(len - 1).clamp(0, len)).rev().for_each(|i| {
            if arr[i] < arr[i - 1] {
                arr.swap(i, i - 1);
                swapped = true;
            }});
        if !swapped {
            break;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{have_same_elements, is_sorted};

    #[test]
    fn basic() {
        let mut arr = vec![5, 2, 1, 3, 4, 6];
        let cloned = arr.clone();
        cocktail_shaker_sort(&mut arr);
        assert!(is_sorted(&arr) && have_same_elements(&arr, &cloned));
    }

    #[test]
    fn empty() {
        let mut arr = Vec::<i32>::new();
        let cloned = arr.clone();
        cocktail_shaker_sort(&mut arr);
        assert!(is_sorted(&arr) && have_same_elements(&arr, &cloned));
    }

    #[test]
    fn one_element() {
        let mut arr = vec![1];
        let cloned = arr.clone();
        cocktail_shaker_sort(&mut arr);
        assert!(is_sorted(&arr) && have_same_elements(&arr, &cloned));
    }

    #[test]
    fn pre_sorted() {
        let mut arr = vec![1, 2, 3, 4, 5, 6];
        let cloned = arr.clone();
        cocktail_shaker_sort(&mut arr);
        assert!(is_sorted(&arr) && have_same_elements(&arr, &cloned));
    }
}
