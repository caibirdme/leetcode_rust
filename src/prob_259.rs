impl Solution {
    pub fn three_sum_smaller(mut nums: Vec<i32>, target: i32) -> i32 {
        let n = nums.len();;
        if n <= 2 {
            return 0;
        }
        nums.sort();
        let mut ans = 0;
        for i in 0..n-2 {
            for j in i+1..n-1 {
                if nums[i] + 2 * nums[j] >= target {
                    break;
                }
                let t = target - nums[i] - nums[j];
                match nums[j+1..].binary_search(&t) {
                    Ok(mut idx) => {
                        while idx > 0 && nums[j+1+idx] == t {
                            idx -= 1;
                        }
                        if nums[j+1+idx] != t {
                            ans += idx+1;
                        }
                    },
                    Err(idx) => {
                        if idx != 0 {
                            ans += idx;
                        }
                    }
                }
            }
        }
        ans as i32
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_three_sum_smaller() {
        let test_cases = vec![
            (vec![-1,-1,1,1], -1, 0),
            (vec![-2,0,1,3], 2, 2),
        ];
        for (nums,target,expect) in test_cases {
            assert_eq!(Solution::three_sum_smaller(nums.clone(), target), expect, "nums: {:?}, target: {}", nums, target);
        }
    }
}