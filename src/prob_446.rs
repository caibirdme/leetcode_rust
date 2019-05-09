use std::collections::HashMap;

impl Solution {
    pub fn number_of_arithmetic_slices(a: Vec<i32>) -> i32 {
        let mut ans = 0;
        let n = a.len();
        let mut dp:HashMap<usize, HashMap<i64, i32>> = HashMap::new();
        for i in 1..n {
            for j in 0..i {
                let d = a[i] as i64 - a[j] as i64;
                let mut pre = 0;
                if let Some(v) = dp.get(&j) {
                    if let Some(&v) = v.get(&d) {
                        ans += v;
                        pre = v;
                    }
                }
                let t = dp.entry(i)
                    .or_insert(HashMap::new()).entry(d)
                    .or_insert(0);
                *t += 1+pre;
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
    fn test_number_of_arithmetic_slices() {
        let test_cases = vec![
            (vec![2,1,1,2,3,4], 6),
            (vec![0,2000000000,-294967296], 0),
            (vec![2,2,3,4], 2),
            (vec![2, 4, 6, 8, 10], 7),
            (vec![1,2,3,5,5,8], 5),
        ];
        for (nums, e) in test_cases {
            assert_eq!(e, Solution::number_of_arithmetic_slices(nums.clone()), "nums: {:?}", nums);
        }
    }
}