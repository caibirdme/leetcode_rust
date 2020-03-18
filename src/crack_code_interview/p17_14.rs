use std::collections::BinaryHeap;

impl Solution {
    pub fn smallest_k(arr: Vec<i32>, k: i32) -> Vec<i32> {
        let k = k as usize;
        if k == arr.len() {
            return arr;
        }
        let mut h = BinaryHeap::new();
        for i in 0..k {
            h.push(arr[i]);
        }
        for i in k..arr.len() {
            if let Some(&top) = h.peek() {
                if arr[i] >= top {
                    continue;
                }
                h.pop();
                h.push(arr[i]);
            }
        }
        let mut ans = vec![0; k as usize];
        (0..k).rev().into_iter().for_each(|i| ans[i] = h.pop().unwrap());
        ans
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_smallest_k() {
        let test_cases = vec![
            (vec![1,3,5,7,2,4,6,8], 4, vec![1,2,3,4]),
        ];
        for (nums, k, expect) in test_cases {
            assert_eq!(Solution::smallest_k(nums.clone(), k), expect, "nums: {:?}, k:{}", nums, k);
        }
    }
}