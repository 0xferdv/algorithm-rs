#[allow(unused)]
pub fn gnome_sort<T: PartialOrd + PartialEq + Clone>(arr: &[T]) -> Vec<T> {
    let mut i: usize = 1;
    let mut j: usize = 2;

    let len = arr.len();
    let mut arr = arr.to_vec();
    while i < len {
        if arr[i - 1] < arr[i] {
            i = j;
            j = i + 1;
        } else {
            arr.swap(i - 1, i);
            i -= 1;
            if i == 0 {
                i = j;
                j += 1;
            }
        }
    }
    arr
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{have_same_elements, is_sorted};

    #[test]
    fn basic() {
        let original = [6, 5, -8, 3, 2, 3];
        let res = gnome_sort(&original);
        assert!(is_sorted(&res) && have_same_elements(&res, &original));
    }

    #[test]
    fn already_sorted() {
        let original = gnome_sort(&["a", "b", "c"]);
        let res = gnome_sort(&original);
        assert!(is_sorted(&res) && have_same_elements(&res, &original));
    }

    #[test]
    fn odd_number_of_elements() {
        let original = gnome_sort(&["d", "a", "c", "e", "b"]);
        let res = gnome_sort(&original);
        assert!(is_sorted(&res) && have_same_elements(&res, &original));
    }

    #[test]
    fn one_element() {
        let original = gnome_sort(&[3]);
        let res = gnome_sort(&original);
        assert!(is_sorted(&res) && have_same_elements(&res, &original));
    }

    #[test]
    fn empty() {
        let original = gnome_sort(&Vec::<u8>::new());
        let res = gnome_sort(&original);
        assert!(is_sorted(&res) && have_same_elements(&res, &original));
    }
}
