use std::collections::HashSet;

impl Solution {
    pub fn find_maximum_xor(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        if n <= 1 {
            return 0;
        }
        if n == 2 {
            return nums[0]^nums[1];
        }
        let mut ans = 0;
        let mut temp = 0;
        let mut set = HashSet::new();
        for i in (0..=30).rev() {
            temp = temp | (1<<i);
            set.clear();
            for &v in nums.iter() {
                set.insert(v&temp);
            }
            let w = ans | (1<<i);
            for &v in set.iter() {
                if set.contains(&(w^v)) {
                    ans = w;
                    break;
                }
            }
        }
        ans
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use super::Solution;
    #[test]
    fn test_find_maximum_xor() {
        let test_cases = vec![
            (vec![3, 10, 5, 25, 2, 8], 28),
        ];
        for (nums, expect) in test_cases {
            assert_eq!(expect, Solution::find_maximum_xor(nums.clone()), "nums: {:?}", nums);
        }
    }
}