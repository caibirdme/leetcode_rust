use std::collections::BinaryHeap;
use std::cmp::{Ordering, Reverse};

impl Solution {
    pub fn kth_smallest(matrix: Vec<Vec<i32>>, mut k: i32) -> i32 {
        let n = matrix.len();
        if n == 0 {
            return 0;
        }
        let mut pq = BinaryHeap::with_capacity(k as usize);
        for i in 0..n {
            pq.push(Reverse(Pair::new(matrix[i][0], i, 0)));
        }
        let mut ans = Pair::new(matrix[0][0],0,0);
        for _ in 0..k {
            ans = pq.pop().unwrap().0;
            let next_idx = ans.2+1;
            if next_idx < n {
                pq.push(Reverse(Pair::new(matrix[ans.1][next_idx], ans.1, next_idx)));
            }
        }
        ans.0
    }
}

#[derive(Eq, PartialEq, Debug)]
struct Pair(i32,usize,usize);

impl Pair {
    pub fn new(a:i32,x:usize,y:usize) -> Self {
        Self(a,x,y)
    }
}
impl Ord for Pair {
    fn cmp(&self, other: &Self) -> Ordering {
        self.0.cmp(&other.0)
    }
}

impl PartialOrd for Pair {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use super::Solution;
    #[test]
    fn test_kth_smallest() {
        let test_cases = vec![
            (vec![vec![1,5,9],vec![10,11,13],vec![12,13,15]], 8, 13),
            (vec![vec![8]], 1, 8),
            (vec![vec![1,3], vec![2,4]], 1, 1),
            (vec![vec![1,3], vec![2,4]], 2, 2),
            (vec![vec![1,3], vec![2,4]], 3, 3),
            (vec![vec![1,3], vec![2,4]], 4, 4),
        ];
        for (matrix, k, expect) in test_cases {
            assert_eq!(expect, Solution::kth_smallest(matrix.clone(), k), "matrix: {:?}, k: {}", matrix, k);
        }
    }
}