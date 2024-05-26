fn partition<T: PartialOrd + Copy>(arr: &mut [T], pivot_idx: usize) -> usize {
    let pivot_value = arr[pivot_idx];
    arr.swap(pivot_idx, arr.len() - 1);
    let mut i = 0;
    for j in 0..arr.len() - 1 {
        if arr[j] < pivot_value {
            arr.swap(i, j);
            i += 1;
        }
    }
    arr.swap(i, arr.len() - 1);
    i
}

pub fn quick_select<T: PartialOrd + Copy>(mut arr: &mut [T], k: usize) -> &T {
    let mut k_updated = k; // k is relative to the current arr slice
    loop {
        let pivot_idx = arr.len() / 2;
        let pivot_idx = partition(arr, pivot_idx);
        if k_updated == pivot_idx {
            return &arr[k_updated];
        } else if k_updated < pivot_idx {
            arr = &mut arr[..pivot_idx];
        } else {
            arr = &mut arr[pivot_idx + 1..];
            k_updated -= pivot_idx + 1;
        }
    }
}

#[cfg(test)]
mod tests {
    use rand::rngs::StdRng;
    use rand::seq::SliceRandom;
    use rand::SeedableRng;

    use super::*;

    fn test_quick_select<T: PartialOrd + Copy>(arr: &[T], k: usize) -> T {
        let mut arr_clone = arr.to_vec();
        let ret = quick_select(&mut arr_clone, k);
        *ret
    }

    fn create_random_vec_distinct(len: usize) -> Vec<i32> {
        let mut vec = (0..len as i32).collect::<Vec<_>>();
        vec.shuffle(&mut StdRng::seed_from_u64(0));
        vec
    }

    #[test]
    fn test_quick_select_random_no_duplicates() {
        let len = 100;
        let arr = create_random_vec_distinct(len);
        let mut arr_sorted = arr.clone();
        arr_sorted.sort();
        for i in 0..arr.len() {
            let selected = test_quick_select(&arr, i);
            assert_eq!(selected, arr_sorted[i]);
        }
    }
}