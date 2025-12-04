#![allow(unused)]


pub fn relu(arr: &mut Vec<f32>) -> &mut Vec<f32> {
    (&mut *arr).iter_mut().for_each(|val| {
        if val <= &mut 0. {
            *val = 0.0;
        }
    });
    arr
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_relu() {
        let mut test: Vec<f32> = Vec::from([1.0, 0.5, -1.0, 0.0, 0.3]);
        assert_eq!(relu(&mut test), &mut Vec::<f32>::from([1.0, 0.5, 0.0, 0.0, 0.3]));
    }
}