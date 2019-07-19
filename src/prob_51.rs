impl Solution {
    pub fn solve_n_queens(n: i32) -> Vec<Vec<String>> {
        if n == 1 {
            return vec![vec!["Q".to_string()]]
        }
        if n <= 3 {
            return vec![];
        }
        let n = n as usize;
        let mut y = vec![false; n];
        let mut sin45 = vec![false; n*2+1];
        let mut sin135 = vec![false; n*2+1];
        let mut ans = vec![];
        let mut cur = vec![0; n];
        Self::dfs(0,  n-1, &mut y, &mut sin45, &mut sin135, &mut cur, &mut ans);
        ans
    }
    fn dfs(pos: usize, n: usize, y: &mut Vec<bool>, sin45: &mut Vec<bool>, sin135: &mut Vec<bool>, cur: &mut Vec<usize>, ans: &mut Vec<Vec<String>>) {
        if pos > n {
            let mut v:Vec<String> = vec![];
            for &p in cur.iter() {
                let mut t = vec!['.'; n+1];
                t[p] = 'Q';
                v.push(t.into_iter().collect());
            }
            ans.push(v);
            return;
        }
        for i in 0..=n {
            if !y[i] && !sin45[pos+i] && !sin135[n+pos-i] {
                cur[pos] = i;
                y[i] = true;
                sin45[pos+i] = true;
                sin135[n+pos-i] = true;
                Self::dfs(pos+1, n,y,sin45,sin135, cur, ans);
                y[i] = false;
                sin45[pos+i] = false;
                sin135[n+pos-i] = false;
            }
        }
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_solve_n_queens() {
        let test_cases = vec![
            (1, vec![vec!["Q"]]),
            (3, vec![]),
            (4, vec![
                vec![
                    ".Q..",
                    "...Q",
                    "Q...",
                    "..Q.",
                ],
                vec![
                    "..Q.",
                    "Q...",
                    "...Q",
                    ".Q.."
                ],
            ]),
        ];
        for (n, expect) in test_cases {
            assert_eq!(Solution::solve_n_queens(n), expect, "n: {}", n);
        }
    }
}