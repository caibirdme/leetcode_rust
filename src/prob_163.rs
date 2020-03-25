impl Solution {
    pub fn find_missing_ranges(nums: Vec<i32>, lower: i32, upper: i32) -> Vec<String> {
        let n = nums.len();
        if n == 0 {
            if lower == upper {
                return vec![lower.to_string()];
            } else {
                return vec![format!("{}->{}", lower, upper)];
            }
        }
        let mut ans = vec![];
        if nums[0] > lower {
            ans.push((lower, nums[0]-1));
        }
        let mut i = 0;
        for i in 0..n-1 {
            if let Some(delta) = nums[i+1].checked_sub(nums[i]) {
                if delta > 1 {
                    ans.push((nums[i]+1, nums[i+1]-1));
                }
            } else {
                ans.push((nums[i]+1, nums[i+1]-1));
            }
        }
        if nums[n-1] < upper {
            ans.push((nums[n-1]+1, upper));
        }
        ans.into_iter().map(|(x,y)| {
            if x == y {
                x.to_string()
            } else {
                format!("{}->{}",x,y)
            }
        }).collect()
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_find_missing_ranges() {
        let test_cases = vec![
            (vec![-2147483648,2147483647],-2147483648,2147483647,vec!["-2147483647->2147483646"]),
            (vec![0, 1, 3, 50, 75], 0, 99, vec!["2", "4->49", "51->74", "76->99"]),
        ];
        for (nums, lower, upper, expect) in test_cases {
            assert_eq!(Solution::find_missing_ranges(nums.clone(), lower, upper), expect, "nums: {:?}, lower: {}, upper: {}", nums, lower, upper);
        }
    }
}