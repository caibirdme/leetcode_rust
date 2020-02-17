impl Solution {
    pub fn next_greater_elements(nums: Vec<i32>) -> Vec<i32> {
        let n = nums.len();
        if n == 0 {
            return vec![];
        }
        if n == 1 {
            return vec![-1];
        }
        let mut stack = vec![];
        for &v in nums.iter().rev() {
            while !stack.is_empty() && *stack.last().unwrap() <= v {
                stack.pop();
            }
            stack.push(v);
        }
        let mut ans = vec![-1; n];
        for i in (0..n).rev() {
            let v = nums[i];
            while !stack.is_empty() && *stack.last().unwrap() <= v {
                stack.pop();
            }
            if let Some(&last) = stack.last() {
                ans[i] = last;
            } else {
                ans[i] = -1;
            }
            stack.push(v);
        }
        ans
    }
}

struct Solution;