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

/// 2.2-2
pub fn selection_sort(a: &mut [i32]) {
    for j in 0..a.len() - 1 {
        let (idx, m) = a[j..a.len()]
            .iter()
            .enumerate()
            .min_by_key(|(_, &el)| el)
            .unwrap();
        if idx == 0 {
            continue;
        }
        let k = a[j];
        a[j] = *m;
        a[idx + j] = k;
    }
}

pub struct MergeSort;

impl MergeSort {
    fn merge(a: &mut [i32], p: usize, q: usize, rr: usize) {
        // println!("merge {:?} {} {} {}", a, p, q, rr);
        let n1 = q - p;
        let n2 = rr - q;
        let mut l = Vec::with_capacity(n1 + 1);
        l.extend_from_slice(
            a[p..p + n1]
                .iter()
                .map(|e| Some(*e))
                .collect::<Vec<_>>()
                .as_ref(),
        );
        l.resize(n1 + 1, None);
        let mut r = Vec::with_capacity(n2 + 1);
        r.extend_from_slice(
            a[q..q + n2]
                .iter()
                .map(|e| Some(*e))
                .collect::<Vec<_>>()
                .as_ref(),
        );
        r.resize(n2 + 1, None);

        // println!("target subarray: {:?}", a[p..rr].iter().collect::<Vec<_>>());
        // println!("gonna merge: {:?} {:?}", l, r);

        let mut i = 0;
        let mut j = 0;
        for k in p..rr {
            if l[i].is_some() && r[j].is_some() {
                if l[i] <= r[j] {
                    a[k] = l[i].unwrap();
                    i += 1;
                } else {
                    a[k] = r[j].unwrap();
                    j += 1;
                }
                continue;
            }

            if l[i].is_none() {
                a[k] = r[j].unwrap();
                j += 1;
                continue;
            }

            if r[j].is_none() {
                a[k] = l[i].unwrap();
                i += 1;
                continue;
            }
        }
        // println!("result subarray: {:?}", a[p..rr].iter().collect::<Vec<_>>());
    }

    fn merge_sort_priv(a: &mut [i32], p: usize, r: usize) {
        // println!(
        //     "run merge_sort_priv: {:?} {} {}",
        //     a[p..r].iter().collect::<Vec<_>>(),
        //     p,
        //     r
        // );
        if p + 1 < r {
            let q = (r + p) / 2;
            MergeSort::merge_sort_priv(a, p, q);
            MergeSort::merge_sort_priv(a, q, r);
            MergeSort::merge(a, p, q, r);
        }
    }

    pub fn merge_sort(a: &mut [i32]) {
        MergeSort::merge_sort_priv(a, 0, a.len());
    }
}

enum IntSentinel {
    Int(i32),
    Guard,
}

impl IntSentinel {
    fn int(&self) -> i32 {
        if let IntSentinel::Int(a) = self {
            *a
        } else {
            panic!("cant get value from IntSentinel::Guard");
        }
    }
}

impl PartialEq for IntSentinel {
    fn eq(&self, other: &IntSentinel) -> bool {
        if let IntSentinel::Int(a) = &self {
            if let IntSentinel::Int(b) = &other {
                return a == b;
            } else {
                false
            }
        } else {
            false
        }
    }
}

impl PartialOrd for IntSentinel {
    fn partial_cmp(&self, other: &IntSentinel) -> Option<std::cmp::Ordering> {
        if let IntSentinel::Int(a) = &self {
            if let IntSentinel::Int(b) = &other {
                return a.partial_cmp(b);
            } else {
                Some(std::cmp::Ordering::Less)
            }
        } else {
            Some(std::cmp::Ordering::Greater)
        }
    }
}

pub struct MergeSort2;

impl MergeSort2 {
    fn merge(sub: &mut [i32], l: Vec<i32>, r: Vec<i32>) {
        // println!("merge target: {:?} with l: {:?} and r: {:?}", sub, l, r);
        let l = l
            .into_iter()
            .map(|e| IntSentinel::Int(e))
            .chain(std::iter::once(IntSentinel::Guard))
            .collect::<Vec<_>>();
        let r = r
            .into_iter()
            .map(|e| IntSentinel::Int(e))
            .chain(std::iter::once(IntSentinel::Guard))
            .collect::<Vec<_>>();

        let mut i = l.iter().peekable();
        let mut j = r.iter().peekable();
        let mut k = sub.iter_mut();

        while let Some(k) = k.next() {
            let i_val = i.peek().unwrap();
            let j_val = j.peek().unwrap();

            if **i_val < **j_val {
                *k = i_val.int();
                i.next();
            } else {
                *k = j_val.int();
                j.next();
            }
        }
        // println!("target result: {:?}", sub);
    }

    fn merge_priv(a: &mut [i32], p: usize, r: usize) {
        // println!(
        //     "run merge_sort_priv: {:?} {} {}",
        //     a[p..r].iter().collect::<Vec<_>>(),
        //     p,
        //     r
        // );
        if p + 1 < r {
            let q = (r + p) / 2;
            MergeSort2::merge_priv(a, p, q);
            MergeSort2::merge_priv(a, q, r);
            let l = a[p..q].into();
            let rr = a[q..r].into();
            MergeSort2::merge(&mut a[p..r], l, rr);
        }
    }

    pub fn merge_sort(a: &mut [i32]) {
        MergeSort2::merge_priv(a, 0, a.len());
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn merge_sort_check() {
        let mut a = vec![6, 3];
        MergeSort::merge_sort(&mut a);
        assert_eq!(a, &[3, 6]);

        let mut a = vec![6, 3, 5];
        MergeSort::merge_sort(&mut a);
        assert_eq!(a, &[3, 5, 6]);

        let mut a = vec![6, 3, 5, 4];
        MergeSort::merge_sort(&mut a);
        assert_eq!(a, &[3, 4, 5, 6]);

        let mut a = vec![4, 5, 3, 10, 11];
        MergeSort::merge_sort(&mut a);
        assert_eq!(a, &[3, 4, 5, 10, 11]);
    }

    #[test]
    fn merge_sort_check2() {
        let mut a = vec![6, 3];
        MergeSort2::merge_sort(&mut a);
        assert_eq!(a, &[3, 6]);

        let mut a = vec![6, 3, 5];
        MergeSort2::merge_sort(&mut a);
        assert_eq!(a, &[3, 5, 6]);

        let mut a = vec![6, 3, 5, 4];
        MergeSort2::merge_sort(&mut a);
        assert_eq!(a, &[3, 4, 5, 6]);

        let mut a = vec![4, 5, 3, 10, 11];
        MergeSort2::merge_sort(&mut a);
        assert_eq!(a, &[3, 4, 5, 10, 11]);
    }

    #[test]
    fn selection_sort_check() {
        let mut a = vec![5, 2, 4, 6, 1, 3];
        selection_sort(&mut a);
        assert_eq!(a, &[1, 2, 3, 4, 5, 6]);
    }

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
