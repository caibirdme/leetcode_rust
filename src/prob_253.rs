use std::collections::BinaryHeap;
use std::cmp::{max, Reverse};

impl Solution {
    pub fn min_meeting_rooms(mut intervals: Vec<Vec<i32>>) -> i32 {
        if intervals.is_empty() {
            return 0;
        }
        if intervals.len() == 1 {
            return 1;
        }
        intervals.sort_by(|a,b| a[0].cmp(&b[0]));
        let mut heap: BinaryHeap<Reverse<i32>> = BinaryHeap::new();
        let mut ans = 0;
        for interval in intervals {
            let (s,e) = (interval[0], interval[1]);
            if let Some(&end) = heap.peek() {
                if end.0 <= s {
                    heap.pop();
                }
            }
            heap.push(Reverse(e));
            ans = max(ans, heap.len());
        }
        ans as i32
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_min_meeting_rooms() {
        let test_cases = vec![
            (vec![vec![1,5], vec![8,9], vec![8,9]], 2),
            (vec![vec![0,30],vec![5,10],vec![15,20]], 2),
            (vec![vec![7,10],vec![2,4]], 1),
        ];
        for (intervals, num) in test_cases {
            assert_eq!(Solution::min_meeting_rooms(intervals.clone()), num, "intervals: {:?}", intervals);
        }
    }
}