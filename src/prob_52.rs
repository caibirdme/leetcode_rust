impl Solution {
    pub fn total_n_queens(n: i32) -> i32 {
        if n == 1 {
            return 1;
        }
        if n <= 3 {
            return 0;
        }
        let n = n as usize;
        let mut ans= 0;
        Self::dfs(0,  (n-1)/2, n-1, 0,0,0, &mut ans);
        ans<<1
    }
    fn dfs(pos: usize, k:usize, n: usize, y:usize, sin45: usize, sin135: usize, ans: &mut i32) {
        if pos > n {
            *ans += 1;
            return;
        }
        for i in 0..=k {
                if (y>>i)&1 == 0 && (sin45>>(pos+i))&1 == 0 && (sin135>>(n+i-pos)) & 1 == 0 {
                // 如果总数是奇数(n是偶数)，且第一排放中间
                if n&1 == 0 && pos == 0 && i == k {
                    Self::dfs(1, k, n, y|(1<<i), sin45|(1<<(pos+i)), sin135|(1<<(n+i-pos)), ans);
                } else {
                    Self::dfs(pos+1, n, n, y|(1<<i), sin45|(1<<(pos+i)), sin135|(1<<(n+i-pos)), ans);
                }
            }
        }
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_total_n_queens() {
        let test_cases = vec![
            (0, 0),
            (1,1),
            (2, 0),
            (3,0),
            (4,2),
            (5,10),
            (6, 4),
            (7, 40),
            (8, 92),
            (9, 352),
        ];
        for (n, expect) in test_cases {
            assert_eq!(Solution::total_n_queens(n), expect, "n: {}", n);
        }
    }

}