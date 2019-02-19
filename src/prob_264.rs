use std::collections::{BinaryHeap,HashSet};
use std::cmp::Reverse;
impl Solution {
    pub fn nth_ugly_number(mut n: i32) -> i32 {
        let mut i:i64 = 0;
        let mut heap = BinaryHeap::new();
        let mut inHeap: HashSet<i64>= HashSet::new();
        heap.push(Reverse(1));
        let candidate:[i64; 3] = [2,3,5];
        loop {
            let t = heap.pop().unwrap();
            let q = t.0;
            for &i in candidate.iter() {
                let p = q*i;
                if !inHeap.contains(&p) {
                    heap.push(Reverse(p));
                    inHeap.insert(p);
                }
            }
            n-=1;
            if n == 0 {
                return t.0 as i32;
            }
        }
        unreachable!();
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use super::Solution;
    #[test]
    fn test_nth_ugly_number() {
        assert_eq!(
            Solution::nth_ugly_number(10),
            12
        );
        assert_eq!(
            Solution::nth_ugly_number(1),
            1
        );
        assert_eq!(
            Solution::nth_ugly_number(2),
            2
        );
        assert_eq!(
            Solution::nth_ugly_number(4),
            4
        );
        assert_eq!(
            Solution::nth_ugly_number(9),
            10
        );
        assert_eq!(
            Solution::nth_ugly_number(1407),
            536870912
        );
    }
}