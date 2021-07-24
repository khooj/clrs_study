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
    let mut result = Vec::with_capacity(N+1);
    result.resize(N+1, 0b0);

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
        result[idx+1] = m;
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

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