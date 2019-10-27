use std::cmp::{max, min};

impl Solution {
    pub fn tiling_rectangle(n: i32, m: i32) -> i32 {
        let n = n as usize;
        let m = m as usize;
        if n == m {
            return 1;
        }
        let mut f = vec![vec![0; max(n,m)+1]; max(n,m)+1];
        Self::fill(n, m, &mut f)
    }
    fn fill(n:usize, m: usize, f: &mut Vec<Vec<i32>>) -> i32 {
        if n == 0 || m == 0 {
            return 0;
        }
        // 保证n比较大 方便处理
        if m > n {
            return Self::fill(m,n, f);
        }
        if m == 1 {
            return n as i32;
        }
        if f[n][m] != 0 {
            return f[n][m];
        }
        let mut ans = (m*n) as i32;
        for i in (1..=m).rev() {
            //竖着切
            ans = min(ans, Self::fill(n-i, i, f)+Self::fill(n, m-i, f)+1);

            // 横着切
            ans = min(ans, Self::fill(n-i, m, f)+Self::fill(i, m-i, f)+1);

            // 挖个洞 枚举洞的大小
            for j in 1..min(m-i, i) {
                let t = Self::fill(n-i, i+j, f) + Self::fill(i-j, m-i, f) + Self::fill(n-i+j, m-i-j, f);
                ans = min(ans, t+2);
            }
        }
        f[n][m] = ans;
        f[m][n] = ans;
        ans
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_tiling_rectangle() {
        let test_cases = vec![
            (2,3,3),
            (5,8,5),
            (11,13,6),
        ];
        for (n,m,expect) in test_cases {
            assert_eq!(Solution::tiling_rectangle(n,m), expect, "n:{}, m:{}",n,m);
        }
    }

}