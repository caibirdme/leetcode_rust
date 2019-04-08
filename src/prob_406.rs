use std::collections::BinaryHeap;
use std::cmp::{Ordering, Reverse};

impl Solution {
    pub fn reconstruct_queue(people: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        if people.len() <= 1 {
            return people;
        }
        let mut heap = BinaryHeap::new();
        for item in people {
            heap.push(Reverse(Pair(item[0], item[1])));
        }
        let n = heap.len();
        let mut ans: Vec<Option<Vec<i32>>> = vec![None; n];
        while !heap.is_empty() {
            let cur = heap.pop().unwrap().0;
            let mut count = 0;
            let mut idx = 0;
            while cur.1 > count {
                if ans[idx].is_none() {
                    count+=1;
                }
                idx+=1;
            }
            while idx < n && ans[idx].is_some() {idx+=1;}
            ans[idx] = Some(vec![cur.0, cur.1]);
        }
        ans.into_iter().map(|x| x.unwrap()).collect()
    }
}

#[derive(Eq, PartialEq, Copy, Clone)]
struct Pair(i32, i32);

impl Ord for Pair {
    fn cmp(&self, other: &Self) -> Ordering {
        let t = self.0.cmp(&other.0);
        match t {
            Ordering::Equal => other.1.cmp(&self.1),
            _ => t,
        }
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
    fn test_reconstruct_queue() {
        let test_cases = vec![
            (vec![vec![7,0], vec![4,4], vec![7,1], vec![5,0], vec![6,1], vec![5,2]], vec![vec![5,0], vec![7,0], vec![5,2], vec![6,1], vec![4,4], vec![7,1]]),
            (vec![vec![1,1], vec![2,0]], vec![vec![2,0], vec![1,1]]),
        ];
        for (num, expect) in test_cases {
            assert_eq!(expect, Solution::reconstruct_queue(num.clone()), "num: {:?}", num);
        }
    }
}