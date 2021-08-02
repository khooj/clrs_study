use super::insertion_sort2;
use super::IntSentinelLesser;

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

pub struct MergeSort2;

impl MergeSort2 {
    fn merge(sub: &mut [i32], l: Vec<i32>, r: Vec<i32>) {
        // println!("merge target: {:?} with l: {:?} and r: {:?}", sub, l, r);
        let l = l
            .into_iter()
            .map(|e| IntSentinelLesser::Int(e))
            .chain(std::iter::once(IntSentinelLesser::Guard))
            .collect::<Vec<_>>();
        let r = r
            .into_iter()
            .map(|e| IntSentinelLesser::Int(e))
            .chain(std::iter::once(IntSentinelLesser::Guard))
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

pub struct MergeInsertSort {
    pub insert_sort_size: usize,
}

impl MergeInsertSort {
    fn merge(sub: &mut [i32], l: Vec<i32>, r: Vec<i32>) {
        // println!("merge target: {:?} with l: {:?} and r: {:?}", sub, l, r);
        let l = l
            .into_iter()
            .map(|e| IntSentinelLesser::Int(e))
            .chain(std::iter::once(IntSentinelLesser::Guard))
            .collect::<Vec<_>>();
        let r = r
            .into_iter()
            .map(|e| IntSentinelLesser::Int(e))
            .chain(std::iter::once(IntSentinelLesser::Guard))
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

    fn merge_priv(&self, a: &mut [i32], p: usize, r: usize) {
        // println!(
        //     "run merge_sort_priv: {:?} {} {}",
        //     a[p..r].iter().collect::<Vec<_>>(),
        //     p,
        //     r
        // );
        if p + 1 < r {
            if r - p <= self.insert_sort_size {
                insertion_sort2(&mut a[p..r]);
                return;
            }
            let q = (r + p) / 2;
            self.merge_priv(a, p, q);
            self.merge_priv(a, q, r);
            let l = a[p..q].into();
            let rr = a[q..r].into();
            MergeInsertSort::merge(&mut a[p..r], l, rr);
        }
    }

    pub fn merge_sort(&self, a: &mut [i32]) {
        self.merge_priv(a, 0, a.len());
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
    fn merge_sort_check3() {
        let mut a = vec![6, 3];
        MergeInsertSort {
            insert_sort_size: 256,
        }
        .merge_sort(&mut a);
        assert_eq!(a, &[3, 6]);

        let mut a = vec![6, 3, 5];
        MergeInsertSort {
            insert_sort_size: 256,
        }
        .merge_sort(&mut a);
        assert_eq!(a, &[3, 5, 6]);

        let mut a = vec![6, 3, 5, 4];
        MergeInsertSort {
            insert_sort_size: 256,
        }
        .merge_sort(&mut a);
        assert_eq!(a, &[3, 4, 5, 6]);

        let mut a = vec![4, 5, 3, 10, 11];
        MergeInsertSort {
            insert_sort_size: 256,
        }
        .merge_sort(&mut a);
        assert_eq!(a, &[3, 4, 5, 10, 11]);
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
