use std::cmp::max;

impl Solution {
    pub fn max_sum_div_three(nums: Vec<i32>) -> i32 {
        if nums.is_empty() {
            return 0;
        }
        let mut dp = vec![vec![0,-std::i32::MAX,-std::i32::MAX]; 2];
        let mut idx = 0;
        for v in nums {
            let m = (v % 3) as usize;
            let nidx = idx ^ 1;
            for i in 0..3usize {
                let p = (3+i-m)%3;
                dp[nidx][i] = max(dp[idx][i], dp[idx][p]+v);
            }
            idx = nidx;
        }
        dp[idx][0]
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_max_sum_div_three() {
        let test_cases = vec![
            (vec![5,2,2,2], 9),
            (vec![4,11,11,4,11,4, 101], 138),
            (vec![11,11,4,11,4], 33),
            (vec![4,11,11,4,11,4], 45),
            (vec![1,2,3,4,4,], 12),
            (vec![3,6,5,1,8], 18),
            (vec![4], 0),
        ];
        for (nums, expect) in test_cases {
            assert_eq!(Solution::max_sum_div_three(nums.clone()), expect, "nums: {:?}", nums);
        }
    }
}