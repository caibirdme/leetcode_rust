use std::cmp::{max,min};

impl Solution {
    pub fn split_array(nums: Vec<i32>, m: i32) -> i32 {
        if m == 1 {
            return nums.iter().sum();
        }
        let nums_i64: Vec<i64> = nums.into_iter().map(|v| v as i64).collect();
        let (mut l, mut r) = (0, std::i32::MAX as i64);
        let mut ans = r;
        while l <= r {
            let mid = (l+r) >> 1;
            if let Some(v) = Self::check(&nums_i64, mid, m) {
                ans = min(ans, v);
                r = v-1;
            } else {
                l = mid+1;
            }
        }
        ans as i32
    }
    fn check(nums: &Vec<i64>, t: i64, m: i32) -> Option<i64> {
        let n = nums.len()-1;
        let mut sum = 0;
        let mut group = 1;
        let mut cur_max = 0;
        for (i, &v) in nums.iter().enumerate() {
            if v > t {
                return None;
            }
            sum += v;
            if sum == t {
                cur_max = t;
                sum = 0;
                if i < n {
                    group += 1;
                }
            } else if sum > t {
                group += 1;
                cur_max = max(cur_max, sum-v);
                sum = v;
            }
            if group > m {
                return None;
            }
        }
        Some(max(cur_max, sum))
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use super::Solution;
    #[test]
    fn test_split_array() {
        let test_cases = vec![
            (vec![1, std::i32::MAX], 2, std::i32::MAX),
            (vec![1,2147483646], 1, std::i32::MAX),
            (vec![7,2,5,10,8], 3, 14),
            (vec![7,2,5,10,8], 2, 18),
            (vec![1,2,6,4,5], 5, 6),
        ];
        for (num,m,expect) in test_cases {
            assert_eq!(expect, Solution::split_array(num.clone(), m), "num: {:?}, m: {}",num,m);
        }
    }
}