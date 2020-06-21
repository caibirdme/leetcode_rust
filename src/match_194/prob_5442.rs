use std::collections::{HashMap, BTreeSet};

impl Solution {
    pub fn avoid_flood(mut rains: Vec<i32>) -> Vec<i32> {
        let mut water = HashMap::new();
        let mut no_rain = BTreeSet::new();
        let n = rains.len();
        let mut ans = vec![-1; n];
        for i in 0..n {
            let t = rains[i];
            if t == 0 {
                no_rain.insert(i);
                continue;
            }
            if let Some(pos) = water.insert(t, i) {
                if let Some(&j) = no_rain.range(pos+1..).nth(0) {
                    rains[j] = -1;
                    ans[j] = t;
                    no_rain.remove(&j);
                } else {
                    return vec![];
                }
            }
        }
        for i in 0..n {
            if rains[i] == 0 {
                ans[i] = 1;
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
    fn test_avoid_flood() {
        let test_cases = vec![
            (vec![1,2,0,0,2,1], vec![-1,-1,2,1,-1,-1]),
            (vec![0,1,1], vec![]),
            (vec![1,2,3,4], vec![-1,-1,-1,-1]),
            (vec![1,2,0,1,2], vec![]),
            (vec![69,0,0,0,69], vec![-1,69,1,1,-1]),
            (vec![10,20,20], vec![]),
        ];
        for (rains, expect) in test_cases {
            assert_eq!(Solution::avoid_flood(rains.clone()), expect, "rains: {:?}", rains);
        }
    }
}