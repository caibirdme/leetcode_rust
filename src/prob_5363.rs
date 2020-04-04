use std::cmp::max;

impl Solution {
    pub fn max_satisfaction(mut satisfaction: Vec<i32>) -> i32 {
        let n = satisfaction.len();
        if n == 0 {
            return 0;
        }
        if n == 1 {
            if satisfaction[0] < 0 {
                return 0;
            }
            return satisfaction[0];
        }
        satisfaction.sort();
        let mut s = 0;
        let mut w = 0;
        let mut ans = 0;
        for i in (0..n).rev() {
            let val = satisfaction[i]+w+s;
            if val < 0 {
                break;
            }
            s += satisfaction[i];
            w = val;
            ans = ans.max(val);
        }
        ans
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_max_satisfaction() {
        let test_cases = vec![
            (vec![-2,5,-1,0,3,-3], 35),
            (vec![4,3,2], 20),
            (vec![-1,-8,0,5,-9], 14),
            (vec![-1,-4,-5], 0),
        ];
        for (sat, expect) in test_cases {
            assert_eq!(Solution::max_satisfaction(sat.clone()), expect, "sat: {:?}", sat);
        }
    }
}