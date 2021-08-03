pub mod insert;
pub mod merge;

pub use insert::*;
pub use merge::*;

pub fn bubble_sort(arr: &mut [i32]) {
    for i in (0..arr.len()).rev() {
        for j in 0..i - 1 {
            if arr[j] > arr[j + 1] {
                arr.swap(j, j + 1);
            }
        }
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

// TODO: can I somehow remove generic parameter?
pub enum IntSentinel<T> {
    Int(i32),
    Guard,
    #[allow(non_camel_case_types)]
    __data(PhantomData<T>),
}

impl<T> IntSentinel<T> {
    pub fn int(&self) -> i32 {
        if let IntSentinel::Int(a) = self {
            *a
        } else {
            panic!("cant get value from IntSentinel::Guard");
        }
    }

    pub fn is_int(&self) -> bool {
        if let IntSentinel::Int(_) = &self {
            true
        } else {
            false
        }
    }
}

impl<T> PartialEq for IntSentinel<T> {
    fn eq(&self, other: &IntSentinel<T>) -> bool {
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

impl<T> PartialEq<i32> for IntSentinel<T> {
    fn eq(&self, other: &i32) -> bool {
        if let IntSentinel::Int(a) = &self {
            return a == other;
        }
        false
    }
}

use std::{cmp::Ordering, marker::PhantomData};
pub trait Comparer {
    fn less() -> Ordering;
    fn greater() -> Ordering;
}

/// Ascending order
pub struct Lesser;

impl Comparer for Lesser {
    fn less() -> Ordering {
        Ordering::Less
    }
    fn greater() -> Ordering {
        Ordering::Greater
    }
}

/// Descending order
pub struct Greater;

impl Comparer for Greater {
    fn greater() -> Ordering {
        Ordering::Less
    }

    fn less() -> Ordering {
        Ordering::Greater
    }
}

impl<T> PartialOrd for IntSentinel<T>
where
    T: Comparer,
{
    fn partial_cmp(&self, other: &IntSentinel<T>) -> Option<std::cmp::Ordering> {
        if let IntSentinel::Int(a) = &self {
            if let IntSentinel::Int(b) = &other {
                return a.partial_cmp(b);
            } else {
                Some(T::less())
            }
        } else {
            Some(T::greater())
        }
    }
}

impl<T> PartialOrd<i32> for IntSentinel<T>
where
    T: Comparer,
{
    fn partial_cmp(&self, other: &i32) -> Option<Ordering> {
        if let IntSentinel::Int(a) = &self {
            a.partial_cmp(other)
        } else {
            Some(T::less())
        }
    }
}

impl<T> std::ops::Add for IntSentinel<T>
{
    type Output = IntSentinel<T>;

    fn add(self, rhs: IntSentinel<T>) -> Self::Output {
        if let IntSentinel::Int(a) = &self {
            if let IntSentinel::Int(b) = rhs {
                IntSentinel::Int(*a + b)
            } else {
                self
            }
        } else {
            if rhs.is_int() {
                rhs
            } else {
                IntSentinel::Guard
            }
        }
    }
}

pub type IntSentinelLesser = IntSentinel<Lesser>;
pub type IntSentinelGreater = IntSentinel<Greater>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn selection_sort_check() {
        let mut a = vec![5, 2, 4, 6, 1, 3];
        selection_sort(&mut a);
        assert_eq!(a, &[1, 2, 3, 4, 5, 6]);
    }
}
