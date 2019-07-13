impl Solution {
    pub fn three_sum_closest(mut nums: Vec<i32>, target: i32) -> i32 {
        let n = nums.len();
        if n < 3 {
            return 0;
        }
        nums.sort();
        let mut ans = nums[0]+nums[1]+nums[2];
        let mut res: i32 = (ans-target).abs();
        for i in 0..n-2 {
            let mut l = i+1;
            let mut r = n-1;
            while l < r {
                let t = nums[i]+nums[l]+nums[r];
                let d:i32 = (t-target).abs();
                if d < res {
                    res = d;
                    ans = t;
                    if res == 0 {
                        return ans;
                    }
                }
                if t > target {
                    r-=1;
                } else {
                    l+=1;
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
    fn test_three_sum_closest() {
        let test_cases = vec![
            (vec![-1, 2, 1, -4], 1, 2),
            (vec![-4, -1, 2, 1], 1, 2),
            (vec![1,2,3], 1, 6),
            (vec![7,5,3,1], 2, 9),
            (vec![1,2,3], 6, 6),
        ];
        for (nums, target, expect) in test_cases {
            assert_eq!(Solution::three_sum_closest(nums.clone(), target), expect, "nums: {:?}, target: {}", nums, target);
        }
    }
}