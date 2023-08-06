fn qs(arr: &mut [i32], lo: usize, hi: usize) {
    if lo >= hi {
        return;
    }
    let pivot_idx = partition(arr, lo, hi);
    qs(arr, lo, pivot_idx - 1);
    qs(arr, pivot_idx + 1, hi);
}

fn partition(arr: &mut [i32], lo: usize, hi: usize) -> usize {
    let pivot = arr[hi];
    let mut idx = lo;
    for i in lo..hi {
        if arr[i] <= pivot {
            idx += 1;
            arr.swap(i, idx - 1);
        }
    }
    arr[hi] = arr[idx];
    arr[idx] = pivot;
    idx
}
fn quick_sort(arr: &mut Vec<i32>) {
    let len = arr.len();
    qs(arr, 0, len - 1);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let mut arr = vec![9, 3, 7, 4, 69, 420, 42];
        quick_sort(&mut arr);

        assert_eq!(arr, vec![3, 4, 7, 9, 42, 69, 420]);
    }
}
