impl Solution {
    pub fn create_target_array(nums: Vec<i32>, index: Vec<i32>) -> Vec<i32> {
        let mut ans = vec![];
        for (v, idx) in nums.into_iter().zip(index.into_iter()) {
            let idx = idx as usize;
            if idx >= ans.len() {
                ans.push(v);
            } else {
                let n = ans.len();
                ans.push(0);
                for i in (idx..n).rev() {
                    ans[i+1] = ans[i];
                }
                ans[idx] = v;
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
    fn test_create_target_array() {
        let test_cases = vec![
            (vec![0,1,2,3,4], vec![0,1,2,2,1], vec![0,4,1,3,2]),
        ];
        for (nums, idx, expect) in test_cases {
            assert_eq!(Solution::create_target_array(nums.clone(), idx.clone()), expect, "nums: {:?}, idx: {:?}", nums, idx);
        }
    }
}