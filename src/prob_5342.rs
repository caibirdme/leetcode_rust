use std::collections::{BinaryHeap, HashMap};
use std::cmp::Reverse;

impl Solution {
    pub fn max_events(mut events:  Vec<Vec<i32>>) -> i32 {
        let mut hash = HashMap::new();
        let mut last_day = 0;
        for v in events {
            last_day = last_day.max(v[1]);
            hash.entry(v[0]).or_insert(vec![]).push(v[1]);
        }
        let mut ans = 0;
        let mut heap = BinaryHeap::new();
        for i in 1..=last_day {
            if let Some(arr) = hash.get(&i) {
                for &v in arr {
                    heap.push(Reverse(v));
                }
            }
            while !heap.is_empty() {
                let d = heap.pop().unwrap().0;
                if d >= i {
                    ans += 1;
                    break;
                }
            }
        }
        ans
    }
}

struct Solution;

