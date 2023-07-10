fn bubble_sort(arr: &mut Vec<i32>) {
    let length = arr.len();
    for i in 0..length {
        for j in 0..length - i - 1 {
            if arr[j] > arr[j + 1] {
                arr.swap(j, j + 1);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let mut arr = vec![9, 3, 7, 4, 69, 420, 42];
        bubble_sort(&mut arr);
        assert_eq!(arr, vec![3, 4, 7, 9, 42, 69, 420]);
    }
}
