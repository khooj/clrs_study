/// Doc test check
/// ```rust
/// use clrs_study::sorts::insert_sort;
/// let mut a = vec![3, 2, 1];
/// insert_sort(&mut a);
/// assert_eq!(a, &[1, 2, 3]);
/// ```
pub fn insert_sort(arr: &mut [i32]) {
    for j in 1..arr.len() {
        let k = arr[j];
        let mut i: i32 = (j as i32) - 1;
        while i >= 0 && arr[i as usize] > k {
            arr[(i + 1) as usize] = arr[i as usize];
            i -= 1;
        }
        arr[(i + 1) as usize] = k;
    }
}

pub fn insert_sort_decreasing(arr: &mut [i32]) {
    for j in 1..arr.len() {
        let k = arr[j];
        let mut i: i32 = (j as i32) - 1;
        while i >= 0 && arr[i as usize] < k {
            arr[(i + 1) as usize] = arr[i as usize];
            i -= 1;
        }
        arr[(i + 1) as usize] = k;
    }
}

pub fn insertion_sort2(a: &mut [i32]) {
    for j in 1..a.len() {
        let k = a[j];
        let shift_idx = a[0..j]
            .iter()
            .enumerate()
            .rev()
            .find(|(_, e)| **e < k)
            .map(|(idx, _)| idx as i32)
            .unwrap_or(-1);
        a.copy_within((shift_idx + 1) as usize..j, (shift_idx + 2) as usize);
        a[(shift_idx + 1) as usize] = k;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn insert_sort_check() {
        let mut arr = vec![5, 2, 4, 6, 1, 3];
        insert_sort(&mut arr);
        assert_eq!(arr, &[1, 2, 3, 4, 5, 6]);
    }

    #[test]
    fn insert_sort_decreasing_check() {
        let mut arr = vec![5, 2, 4, 6, 1, 3];
        insert_sort_decreasing(&mut arr);
        assert_eq!(arr, &[6, 5, 4, 3, 2, 1]);
    }

    #[test]
    fn insertion_sort2_check() {
        let mut arr = vec![5, 2, 4, 6, 1, 3];
        insertion_sort2(&mut arr);
        assert_eq!(arr, &[1, 2, 3, 4, 5, 6]);

        let mut arr = vec![5];
        insertion_sort2(&mut arr);
        assert_eq!(arr, &[5]);

        let mut arr = vec![];
        insertion_sort2(&mut arr);
        assert_eq!(arr, &[]);
    }

    #[test]
    fn check_iter() {
        let a = vec![1];
        let mut k = a.iter();
        assert_eq!(k.next(), Some(&1));
        assert_eq!(k.next(), None);

        let mut k = a[0..1].iter();
        assert_eq!(k.next(), Some(&1));
        assert_eq!(k.next(), None);
    }
}
