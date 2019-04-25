use std::collections::HashSet;

macro_rules! abs {
    ($e:expr) => {
        {
            if $e < 0 {
                -$e
            } else {
                $e
            }
        }
    };
}

impl Solution {
    pub fn find_duplicates(mut nums: Vec<i32>) -> Vec<i32> {
        let mut ans = HashSet::new();
        let n = nums.len();
        let mut i = 0;
        for i in 0..n {
            let c = nums[i];
            let pos_c = abs!(c);
            if nums[(pos_c-1) as usize] < 0 {
                ans.insert(pos_c);
            } else {
                nums[(pos_c-1) as usize] *= -1;
            }
        }
        ans.into_iter().map(|x| x).collect::<Vec<i32>>()
    }
}

struct Solution;


#[cfg(test)]
mod tests {
    use super::Solution;
    #[test]
    fn test_find_duplicates() {
        let test_cases = vec![
            (vec![4,3,2,7,8,2,3,1], vec![2,3]),
            (vec![2,2,1,3], vec![2]),
            (vec![1], vec![]),
            (vec![1,2,2,2,2,2], vec![2]),
        ];
        for (nums, exp) in test_cases {
            let mut actual  = Solution::find_duplicates(nums.clone());
            actual.sort();
            assert_eq!(exp, actual, "nums: {:?}", nums);
        }
    }
}