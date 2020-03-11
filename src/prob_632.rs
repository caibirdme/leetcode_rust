use std::collections::BinaryHeap;
use std::cmp::Ordering;

#[derive(Eq, PartialEq)]
struct MinElem {
    val: i32,
    which: usize,
}

impl Ord for MinElem {
    fn cmp(&self, other: &Self) -> Ordering {
        other.val.cmp(&self.val)
    }
}

impl PartialOrd for MinElem {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Solution {
    pub fn smallest_range(nums: Vec<Vec<i32>>) -> Vec<i32> {
        let mut maxHeap = BinaryHeap::new();
        let mut minHeap = BinaryHeap::new();
        let k = nums.len();
        let mut ptr = vec![0; k];
        for i in 0..k {
            maxHeap.push(nums[i][0]);
            minHeap.push(MinElem{val: nums[i][0], which:i});
        }
        let mut min_len = std::i32::MAX;
        let mut start = 0;
        loop {
            let MinElem{val: minimal, which} = minHeap.pop().unwrap();
            let &maximum = maxHeap.peek().unwrap();
            if maximum-minimal < min_len {
                min_len = maximum-minimal;
                start = minimal;
            }
            ptr[which] += 1;
            if ptr[which] >= nums[which].len() {
                break;
            }
            let t = nums[which][ptr[which]];
            minHeap.push(MinElem{val: t, which});
            maxHeap.push(t);
        }
        vec![start, start+min_len]
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_smallest_range() {
        let test_cases = vec![
            (vec![
                vec![4,10,15,24,26],
                vec![0,9,12,20],
                vec![5,18,22,30],
            ], vec![20, 24]),
            (vec![vec![1]], vec![1,1]),
            (vec![vec![1], vec![2]], vec![1,2]),

        ];
        for (nums, expect) in test_cases {
            assert_eq!(Solution::smallest_range(nums.clone()), expect, "nums: {:?}", nums);
        }
    }
}