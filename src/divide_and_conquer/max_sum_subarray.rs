use crate::sorts::IntSentinelLesser;

pub struct MaxSumSubarray;

impl MaxSumSubarray {
    fn find_max_crossing_subarray(&self, l: &[i32], r: &[i32]) -> (usize, usize, i32) {
        let mut left_sum = IntSentinelLesser::Guard;
        let mut sum = 0i32;
        let mut max_left = 0usize;
        l.iter().enumerate().rev().for_each(|(idx, el)| {
            sum += *el;
            if left_sum < sum {
                left_sum = IntSentinelLesser::Int(sum);
                max_left = idx;
            }
        });

        let mut right_sum = IntSentinelLesser::Guard;
        let mut max_right = 0usize;
        sum = 0i32;
        r.iter().enumerate().for_each(|(idx, el)| {
            sum += *el;
            if right_sum < sum {
                right_sum = IntSentinelLesser::Int(sum);
                max_right = idx;
            }
        });

        (max_left, max_right, (left_sum + right_sum).int())
    }

    fn find_max(&self, a: &[i32], low: usize, high: usize) -> (usize, usize, i32) {
        if high == low {
            return (low, high, a[low]);
        }

        let mid = (low + high) / 2;

        let (left_low, left_high, left_sum) = self.find_max(a, low, mid);
        let (right_low, right_high, right_sum) = self.find_max(a, mid + 1, high);
        let (mut cross_low, mut cross_high, cross_sum) =
            self.find_max_crossing_subarray(&a[low..mid + 1], &a[mid + 1..high + 1]);

        cross_low += low;
        cross_high += mid + 1;

        if left_sum >= right_sum && left_sum >= cross_sum {
            (left_low, left_high, left_sum)
        } else if right_sum >= left_sum && right_sum >= cross_sum {
            (right_low, right_high, right_sum)
        } else {
            (cross_low, cross_high, cross_sum)
        }
    }

    pub fn find(&self, a: &[i32]) -> Option<(usize, usize, i32)> {
        if a.is_empty() {
            None
        } else {
            Some(self.find_max(a, 0, a.len() - 1))
        }
    }
}

/// 4.1-5
pub fn find_max_subarray(a: &[i32]) -> Option<(usize, usize, i32)> {
    if a.is_empty() {
        return None;
    }

    let mut m = IntSentinelLesser::Guard;
    let mut low_m = 0;
    let mut high_m = 0;
    let mut mr = 0;
    let mut low_r = 0;

    for i in 0..a.len() {
        mr += a[i];
        if m < mr {
            low_m = low_r;
            high_m = i;
            m = IntSentinelLesser::Int(mr);
        }
        if mr < 0 {
            mr = 0;
            low_r = i + 1;
        }
    }


    Some((low_m, high_m, m.int()))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn max_sum_subarray() {
        let sum = MaxSumSubarray {};
        assert_eq!(
            sum.find(&[13, -3, -25, 20, -3, -16, -23, 18, 20, -7, 12, -5, -22, 15, -4, 7]),
            Some((7, 10, 43))
        );
        assert_eq!(sum.find(&[1, 2, -10, 4, 5]), Some((3, 4, 9)));
        assert_eq!(sum.find(&[1, 2, -10, -11]), Some((0, 1, 3)));
        assert_eq!(sum.find(&[]), None);
        assert_eq!(sum.find(&[-1, -2, -3]), Some((0, 0, -1)));
    }

    #[test]
    fn max_sum_subarray_linear() {
        assert_eq!(
            find_max_subarray(&[13, -3, -25, 20, -3, -16, -23, 18, 20, -7, 12, -5, -22, 15, -4, 7]),
            Some((7, 10, 43))
        );
        assert_eq!(find_max_subarray(&[1, 2, -10, 4, 5]), Some((3, 4, 9)));
        assert_eq!(find_max_subarray(&[1, 2, -10, -11]), Some((0, 1, 3)));
        assert_eq!(find_max_subarray(&[]), None);
        assert_eq!(find_max_subarray(&[-1, -2, -3]), Some((0, 0, -1)));
        assert_eq!(find_max_subarray(&[-10, -2, -1, -3, -10]), Some((2, 2, -1)));
    }
}
