/// 2.1-3
pub fn linear_search<T: Ord>(v: T, a: &[T]) -> Option<usize> {
    // initialization: not explicit. Before loop we assume that we
    // will return NIL
    for (idx, i) in a.iter().enumerate() {
        // iteration: at every iteration, if next element equals to v
        // it means that we found what we need and we return it's index
        if v == *i {
            return Some(idx);
        }
        // termination: because we iteration over given elements by iterator
        // we can guarantee correct termination even in case empty slice
    }
    None
}

/// 2.1-4
/// Given: array `A` with length `N` <a1, a2, ..., an> and array `B` with length N <b1, b2, ..., bn> where every element
/// equals one bit.
/// Output: array with length `N+1` <c1, c2, ..., c(n+1)> which equals sum of A and B.
/// cant return `[i8; N+1]` for 1.51
pub fn sum<const N: usize>(a: [i8; N], b: [i8; N]) -> Vec<i8> {
    // initialization: N+1 vec with zero-initialized elements
    // so in case we sum to arrays with zero elements we will return (N+1) == 1 length vec with zeros.
    // Probably I need to emphasize that first element is zero so in case of summation zeros from A and B
    // or values other than zero we will do correct sum.
    let mut result = Vec::with_capacity(N + 1);
    result.resize(N + 1, 0b0);

    // we assume that every element is 0 or 1
    for ((idx, an), bn) in a.iter().enumerate().zip(b.iter()) {
        // iteration: for every element from A and B we also take element from resulting array
        // because it can have some value from previous iteration.
        // After we sum everything and and check if we need to move `overvalue` to next index
        // Example corner cases: 0 + 0 + 0 == 0 -> dont move
        // 1 + 1 + 1 == 3 (or 11 in 2-base) -> need to move `1` to next index and write `1` to current index
        // We calculate moving value as VAL / BASE:
        //      for 0: 0 / 2 == 0
        //      for 3: 3 / 2 == 1
        // and currenct value as VAL % BASE:
        //      for 0: 0 % 2 == 0
        //      for 3: 3 % 2 == 1
        // not really obvious but from `/` operation we get division, from '%' we get remainder
        let t = *an + *bn + result[idx];
        let m = t / 2;
        let t = t % 2;

        result[idx] = t;
        result[idx + 1] = m;
    }

    result
}

fn abs_sub(a: usize, b: usize) -> usize {
    if a > b {
        a - b
    } else {
        b - a
    }
}

pub fn binary_search(a: &[i32], val: i32) -> Option<usize> {
    if a.is_empty() {
        return None;
    }
    let mut i = a.len() / 2;
    let mut i_prev = a.len();
    while a[i] != val {
        if a[i] > val {
            i -= abs_sub(i, i_prev) / 2;
        } else {
            i += abs_sub(i, i_prev) / 2;
        }

        i_prev = i;

        if i == 0 || i == a.len() - 1 {
            return None;
        }
    }
    Some(i)
}

/// 2.3-7
/// should be sorted
pub fn two_sum(a: &[i32], x: i32) -> Option<(usize, usize)> {
    if a.is_empty() {
        return None;
    }
    for i in 0..a.len() - 1 {
        let y = a[i];
        if let Some(k) = binary_search(a, x - y) {
            return Some((i, k));
        }
    }
    None
}

/// 2.4 d
pub struct Inversions;

use super::sorts::IntSentinelGreater;

type IntSentinel = IntSentinelGreater;

impl Inversions {
    fn count(sub: &mut [i32], l: Vec<i32>, r: Vec<i32>) -> usize {
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
        let mut count = 0usize;

        let mut i = l.iter().peekable();
        let mut j = r.iter().peekable();
        let mut k = sub.iter_mut();

        while let Some(k) = k.next() {
            let i_val = i.peek().unwrap();
            let j_val = j.peek().unwrap();

            if **i_val > **j_val {
                *k = i_val.int();
                i.next();
                count += j.len()-1;
            } else {
                *k = j_val.int();
                j.next();
            }
        }
        // println!("target result: {:?} with count: {}", sub, count);
        count
    }

    fn merge_priv(a: &mut [i32], p: usize, r: usize) -> usize {
        // println!(
        //     "run merge_sort_priv: {:?} {} {}",
        //     a[p..r].iter().collect::<Vec<_>>(),
        //     p,
        //     r
        // );
        if p + 1 < r {
            let q = (r + p) / 2;
            let aa = Inversions::merge_priv(a, p, q);
            let b = Inversions::merge_priv(a, q, r);
            let l = a[p..q].into();
            let rr = a[q..r].into();
            return Inversions::count(&mut a[p..r], l, rr) + aa + b;
        }
        0usize
    }

    pub fn merge_sort(a: &mut [i32]) -> usize {
        Inversions::merge_priv(a, 0, a.len())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn inversions_count_check() {
        let mut a = vec![4, 3, 2, 1];
        assert_eq!(Inversions::merge_sort(&mut a), 6);
        let mut a = vec![2, 3, 8, 6, 1];
        assert_eq!(Inversions::merge_sort(&mut a), 5);
    }

    #[test]
    fn two_sum_check() {
        assert_eq!(two_sum(&[], 1), None);
        assert_eq!(two_sum(&[1, 2, 3, 4, 5], 5), Some((0, 3)));
    }

    #[test]
    fn binary_search_test() {
        assert_eq!(binary_search(&[1, 2, 4, 5, 7, 9, 10], 5), Some(3));
        assert_eq!(binary_search(&[], 10), None);
        assert_eq!(binary_search(&[1, 2, 3], 4), None);
        assert_eq!(binary_search(&[2, 3, 4], 1), None);
    }

    #[test]
    fn linear_search_test() {
        assert_eq!(linear_search(10, &[1, 2, 3, 10]), Some(3));
        assert_eq!(linear_search(1, &[1, 2, 3, 10]), Some(0));
        assert_eq!(linear_search(2, &[1, 2, 3, 10]), Some(1));
        assert_eq!(linear_search(-1, &[1, 2, 3]), None);
        assert_eq!(linear_search(10, &[]), None);
    }

    #[test]
    fn bit_sum() {
        assert_eq!(sum([1], [1]), &[0, 1]);
        assert_eq!(sum([], []), &[0]);
        assert_eq!(sum([1, 0], [0, 0]), &[1, 0, 0]);
        assert_eq!(sum([1, 1, 1], [1, 1, 1]), &[0, 1, 1, 1]);
        // assert_eq!(sum([1, 0], [1]), &[0, 1]);
    }
}
