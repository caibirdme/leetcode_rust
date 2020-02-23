impl Solution {
    pub fn jump(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        if n == 1 {
            return 0;
        }
        let mut ans = 0;
        let mut i = 0;
        let mut pre = 0;
        while i != n-1 {
            let s = nums[i] as usize;
            if i + s >= n - 1 {
                return ans + 1;
            }
            let mut t = 0;
            let mut pos = 0;
            for j in pre+1..=i+s {
                let q = j + nums[j] as usize;
                if q > t {
                    t = q;
                    pos = j;
                }
            }
            ans += 1;
            pre = i + s;
            i = pos;
        }
        ans
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_jump() {
        let test_cases = vec![
            (vec![5,9,3,2,1,0,2,3,3,1,0,0], 3),
        ];
        for (nums, expect) in test_cases {
            assert_eq!(Solution::jump(nums.clone()), expect, "nums: {:?}", nums);
        }
    }
}