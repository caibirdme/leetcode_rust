use std::collections::BinaryHeap;
use std::cmp::Ordering;

struct Solution;
impl Solution {
    pub fn find_kth_largest(nums: Vec<i32>, k: i32) -> i32 {
        if nums.len() == 0 {
            return 0;
        }
        let mut min_heap = BinaryHeap::with_capacity(k as usize);
        for i in 0..k as usize {
            min_heap.push(reverseNum(nums[i]));
        }
        for i in k as usize..nums.len() {
            if nums[i] < min_heap.peek().unwrap().0 {
                continue;
            }
            min_heap.push(reverseNum(nums[i]));
            min_heap.pop();
        }
        min_heap.peek().unwrap().0
    }
}

#[derive(Eq, PartialEq)]
struct reverseNum(i32);

impl Ord for reverseNum {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.0 > other.0 {
            Ordering::Less
        } else if self.0 < other.0 {
            Ordering::Greater
        } else {
            Ordering::Equal
        }
    }
}

impl PartialOrd for reverseNum {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;
    #[test]
    fn test_find_kth_largest() {
        assert_eq!(
            Solution::find_kth_largest(vec![3,2,1,5,6,4], 2),
            5
        );
        assert_eq!(
            Solution::find_kth_largest(vec![3,2,3,1,2,4,5,5,6], 4),
            4
        );
        assert_eq!(
            Solution::find_kth_largest(vec![0], 1),
            0
        );
    }
}