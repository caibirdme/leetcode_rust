use std::collections::BinaryHeap;
use std::cmp::Reverse;

impl Solution {
    pub fn connect_sticks(sticks: Vec<i32>) -> i32 {
        if sticks.len() == 1 {
            return 0;
        }
        let mut pq: BinaryHeap<_> = sticks.into_iter().map(|v| Reverse(v)).collect();
        let mut ans = 0;
        while pq.len() > 1 {
            let a = pq.pop().unwrap().0;
            let b = pq.pop().unwrap().0;
            let t = a+b;
            ans += t;
            pq.push(Reverse(t));
        }
        ans
    }
}

struct Solution;