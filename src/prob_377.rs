impl Solution {
    pub fn combination_sum4(nums: Vec<i32>, target: i32) -> i32 {
        let target = target as usize;
        let mut f = vec![0; target+1];
        f[0] = 1;
        for i in 1..=target {
            for &j in nums.iter() {
                if j as usize<=i {
                    f[i] += f[i-j as usize];
                }
            }
        }
        f[target]
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use super::Solution;
    #[test]
    fn test_combination_sum4() {
        let test_cases = vec![
            (vec![1], 3, 1),
            (vec![1,2], 2, 2),
            (vec![1,2,3], 4, 7),
        ];
        for (nums, target, expect) in test_cases {
            assert_eq!(expect, Solution::combination_sum4(nums.clone(), target), "nums: {:?}", nums);
        }
    }
}