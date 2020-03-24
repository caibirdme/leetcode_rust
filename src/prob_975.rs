use std::collections::BTreeMap;
use std::ops::Bound::{Included, Unbounded};

impl Solution {
    pub fn odd_even_jumps(a: Vec<i32>) -> i32 {
        let n = a.len();
        if n == 1 {
            return 1;
        }
        let mut ans = 1;
        let mut bt = BTreeMap::new();
        let mut odd = vec![false; n];
        let mut even = vec![false; n];
        bt.insert(a[n-1], n-1);
        for i in (0..n-1).rev() {
            let cur = a[i];
            if let Some((_, &pos)) = bt.range((Included(&cur), Unbounded)).next() {
                if pos == n-1 || even[pos] {
                    odd[i] = true;
                    ans += 1
                }
            }
            if let Some((_, &pos)) = bt.range((Unbounded, Included(&cur))).rev().next() {
                if pos == n-1 || odd[pos] {
                    even[i] = true;
                }
            }
            bt.insert(cur, i);
        }
        ans
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_odd_even_jumps() {
        let test_cases = vec![
            (vec![81,54,96,60,58], 2),
            (vec![10,13,12,14,15], 2),
            (vec![2,3,1,1,4], 3),
            (vec![5,1,3,4,2], 3),
        ];
        for (arr, expect) in test_cases {
            assert_eq!(Solution::odd_even_jumps(arr.clone()), expect, "arr: {:?}", arr);
        }
    }
}