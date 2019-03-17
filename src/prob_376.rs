impl Solution {
    pub fn wiggle_max_length(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        if n < 2 {
            return n as i32;
        }
        let mut l = 0;
        while l<n-1 && nums[l] == nums[l+1] {l+=1;}
        if l == n-1 {
            return 1;
        }
        let mut ans = 2;
        let mut state = {
            if nums[l] > nums[l+1] {
                true // 应该寻找一个大的
            } else {
                false // 应该寻找一个小的
            }
        };
        let mut pre = nums[l+1];
        for i in l+2..n {
            let t = nums[i];
            if state {
                if t > pre {
                    ans += 1;
                    state = false;
                }
                pre = t;
            } else {
                if t < pre {
                    ans += 1;
                    state = true;
                }
                pre = t;
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
    fn test_wiggle_max_length() {
        let test_cases = vec![
            (vec![2,1,1,1,2,1,2], 5),
            (vec![1], 1),
            (vec![2,2,2,2,2,2], 1),
            (vec![1,2,2,2,1,2], 4),
            (vec![2,2,2,2,1,2], 3),
            (vec![1,7,4,9,2,5], 6),
            (vec![1,17,5,10,13,15,10,5,16,8], 7),
            (vec![1,2,3,4,5,6,7,8,9], 2),
        ];
        for (nums,expect) in test_cases {
            let actual = Solution::wiggle_max_length(nums.clone());
            assert_eq!(expect, actual, "nums: {:?}", nums);
        }
    }
}