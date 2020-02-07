use std::collections::HashMap;

impl Solution {
    pub fn find_target_sum_ways(nums: Vec<i32>, s: i32) -> i32 {
        let n = nums.len();
        if n == 0 {
            return 0;
        }
        let mut f = HashMap::new();
        let zero = nums.iter().filter(|&&v| v == 0).count() as i32;
        let t = 2f64.powi(zero) as i32;
        f.insert(nums[0], t);
        f.insert(-nums[0], t);
        for i in 1..n {
            let v = nums[i];
            if v == 0 {
                continue;
            }
            let mut cur = HashMap::new();
            for (&p, &q) in &f {
                *cur.entry(p+v).or_insert(0) += q;
                *cur.entry(p-v).or_insert(0) += q;
            }
            f = cur;
        }
        match f.get(&s) {
            Some(&v) => v,
            _ => 0,
        }
    }
    pub fn find_target_sum_ways_dp(nums: Vec<i32>, s: i32) -> i32 {
        if nums.is_empty() {
            return 0;
        }
        let sum: i32 = nums.iter().sum();
        if sum < s {
            return 0;
        }
        let us = sum as usize;
        let dus = us*2;
        let zero= nums.iter().filter(|&&v| v == 0).count();
        let mut f = [vec![0; dus+1], vec![0; dus+1]];
        f[0][us] = 2f64.powi(zero as i32) as i32;;
        let mut idx = 1;
        nums.iter().filter(|&&v| v != 0).for_each(|&v| {
            let ni = idx ^ 1;
            for i in v as usize..=dus as usize {
                f[idx][i] = f[ni][i-v as usize];
            }
            for i in 0..=dus-v as usize {
                f[idx][i] += f[ni][i+v as usize];
            }
            idx = ni;
        });
        f[idx^1][(us as i32+s) as usize]
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_find_target_sum_ways() {
        let test_cases = vec![
            (vec![0,0,0,0,0,0,1,0,0], 1, 256),
            (vec![0,0,0,0,0,0,0,0,1], 1, 256),
            (vec![1,2,1], 2, 2),
            (vec![5], -5, 1),
            (vec![1,1,1,1,1], 3, 5),
        ];
        for (nums, s, expect) in test_cases {
            assert_eq!(Solution::find_target_sum_ways_dp(nums.clone(), s), expect, "nums: {:?}, s:{}", nums, s);
        }
    }
}