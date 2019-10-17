use std::collections::HashMap;
use std::cmp::max;

impl Solution {
    pub fn max_sub_array_len(nums: Vec<i32>, k: i32) -> i32 {
        let mut total = 0;
        let mut hash = HashMap::new();
        let n = nums.len();
        let mut ans = 0;
        for (i, v) in nums.into_iter().enumerate() {
            total += v;
            let q = total-k;
            if q == 0 {
                ans = i+1;
            } else if let Some(&idx) = hash.get(&q) {
                ans = max(ans, i-idx);
            }
            hash.entry(total).or_insert(i);
        }
        ans as i32
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_max_sub_array_len() {
        let test_cases = vec![
            (vec![1, -1, 5, -2, 3], 3, 4),
            (vec![-2, -1, 2, 1], 1, 2),
            (vec![-2, -1, 2, -1,-1, 2, 1], 1, 5),
        ];
        for (nums, k, expect) in test_cases {
            assert_eq!(Solution::max_sub_array_len(nums.clone(), k), expect, "nums: {:?}, k: {}", nums, k);
        }
    }
}