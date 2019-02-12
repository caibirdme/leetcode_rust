use std::cmp;
struct Solution;

impl Solution {
    pub fn rob(nums: Vec<i32>) -> i32 {
        let mut ans: i32;
        let length = nums.len();
        if length == 0 {
            return 0;
        }
        if length == 1 {
            return unsafe { *nums.get_unchecked(0) };
        }
        cmp::max(
            Self::sub_rob(&nums[1..]),
            Self::sub_rob(&nums[..nums.len() - 1]),
        )
    }

    fn sub_rob(nums: &[i32]) -> i32 {
        let length = nums.len();
        if length == 0 {
            return 0;
        }
        if length == 1 {
            return unsafe { *nums.get_unchecked(0) };
        }
        let mut f = vec![0; length];
        unsafe {
            let (n0, n1) = (*nums.get_unchecked(0), *nums.get_unchecked(1));
            *f.get_unchecked_mut(0) = n0;
            *f.get_unchecked_mut(1) = {
                if n0 > n1 {
                    n0
                } else {
                    n1
                }
            };
        }
        for i in 2..length {
            let (p, pp) = unsafe { (*f.get_unchecked(i - 1), *f.get_unchecked(i - 2)) };
            unsafe {
                *f.get_unchecked_mut(i) = cmp::max(p, pp + nums[i]);
            }
        }
        *f.last().unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;
    #[test]
    fn test_rob() {
        assert_eq!(Solution::rob(vec![2, 3, 2]), 3);
        assert_eq!(Solution::rob(vec![1, 2, 3, 1]), 4);
    }
}
