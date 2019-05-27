use std::collections::HashSet;

macro_rules! dec_n {
    ($e: ident, $k: expr, $n: ident) => {
        {
            let kk = $k % $n;
            if $e >= kk {
                $e - kk
            } else {
                $e+$n-kk
            }
        }
    };
}

impl Solution {
    pub fn circular_array_loop(nums: Vec<i32>) -> bool {
        let mut hash = HashSet::new();
        let n = nums.len();
        for i in 0..n {
            if hash.contains(&i) {
                continue;
            }
            if nums[i] > 0 {
                if Self::backward(&nums, i, &mut hash) {
                    return true;
                }
            } else {
                if Self::forward(&nums, i, &mut hash) {
                    return true;
                }
            }
        }
        false
    }
    fn backward(nums: &Vec<i32>, start: usize, hash: &mut HashSet<usize>) -> bool {
        let mut fast = start;
        let mut slow = start;
        let n = nums.len();
        hash.insert(start);
        loop {
            fast = (nums[fast] as usize +fast)%n;
            if nums[fast] < 0 {
                return false;
            }
            hash.insert(fast);
            fast = (nums[fast] as usize +fast)%n;
            if nums[fast] < 0 {
                return false;
            }
            hash.insert(fast);
            slow = (nums[slow] as usize + slow)%n;
            if fast == slow {
                return slow != (nums[fast] as usize +fast)%n;
            }
        }
        unreachable!()
    }
    fn forward(nums: &Vec<i32>, start: usize, hash: &mut HashSet<usize>) -> bool {
        let mut fast = start;
        let mut slow = start;
        let n = nums.len();
        loop {
            fast = dec_n!(fast, (-nums[fast]) as usize, n);
            if nums[fast] > 0 {
                return false;
            }
            hash.insert(fast);
            fast = dec_n!(fast, (-nums[fast]) as usize, n);
            if nums[fast] > 0 {
                return false;
            }
            hash.insert(fast);
            slow = dec_n!(slow, (-nums[slow]) as usize, n);
            if fast == slow {
                return slow != dec_n!(fast, (-nums[fast]) as usize, n);
            }
        }
        unreachable!()
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use super::Solution;
    #[test]
    fn test_circular_array_loop() {
        let test_cases = vec![
            (vec![-1,-1], true),
            (vec![1,2,10], true),
            (vec![1,2,3,10], true),
            (vec![1,2,-1,11], false),
            (vec![2,-1,1,2,2], true),
            (vec![-1,2], false),
            (vec![-2,1,-1,-2,-2], false),
        ];
        for (nums, ok) in test_cases {
            assert_eq!(ok, Solution::circular_array_loop(nums.clone()), "nums {:?}", nums);
        }
    }
}