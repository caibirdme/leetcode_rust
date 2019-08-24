impl Solution {
    pub fn get_factors(n: i32) -> Vec<Vec<i32>> {
        let mut ans = vec![];
        let mut cur = vec![];
        Self::dfs(n, 2, &mut cur, &mut ans);
        ans
    }
    fn dfs(n: i32, start: i32, cur: &mut Vec<i32>, ans: &mut Vec<Vec<i32>>) {
        if n == 1 {
            if cur.len() > 1 {
                ans.push(cur.clone());
            }
            return;
        }
        if n < start {
            return;
        }
        let bound = (n as f64).sqrt().floor() as i32;
        for i in start..=bound {
            if n % i == 0 {
                cur.push(i);
                Self::dfs(n/i, i, cur, ans);
                cur.pop();
            }
        }
        if n >= start {
            cur.push(n);
            Self::dfs(1, n, cur, ans);
            cur.pop();
        }
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_get_factors() {
        let test_cases = vec![
            (8, vec![
                vec![2,2,2],
                vec![2,4],
            ]),
            (1, vec![]),
            (2, vec![]),
            (37, vec![]),
            (12, vec![
                vec![2,2,3],
                vec![2,6],
                vec![3,4],
            ]),
            (32, vec![
                vec![2,2,2,2,2],
                vec![2,2,2,4],
                vec![2,2,8],
                vec![2,4,4],
                vec![2,16],
                vec![4,8],
            ]),
        ];
        for (n, expect) in test_cases {
            assert_eq!(Solution::get_factors(n), expect, "n: {}", n);
        }
    }
}