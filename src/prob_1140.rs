impl Solution {
    pub fn stone_game_ii(piles: Vec<i32>) -> i32 {
        let n = piles.len();
        if n <= 2 {
            return piles.iter().sum();
        }
        let mut sum = vec![0; n+1];
        for i in 0..n {
            sum[i+1] = sum[i]+piles[i];
        }
        let mut f = vec![vec![0; n]; n];
        Self::dfs(0, 1, &sum, &mut f)
    }
    fn dfs(idx: usize, m: usize, sum: &Vec<i32>, f: &mut Vec<Vec<i32>>) -> i32 {
        let n = f.len();
        if idx >= n {
            return 0;
        }
        if f[idx][m] > 0 {
            return f[idx][m];
        }
        let mut ans = 0;
        for i in idx..n.min(idx+2*m) {
            let v = sum[n]-sum[idx] - Self::dfs(i+1, m.max(i-idx+1), sum, f);
            ans = ans.max(v);
        }
        f[idx][m] = ans;
        ans
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_stone_game_ii() {
        let test_cases = vec![
            (vec![2,7,9,4,4], 10),
        ];
        for (piles, expect) in test_cases {
            assert_eq!(Solution::stone_game_ii(piles.clone()), expect, "piles: {:?}", piles);
        }
    }
}