impl Solution {
    pub fn unique_paths(m: i32, n: i32) -> i32 {
        let (m, n) = (m as usize, n as usize);
        let mut f = vec![vec![0; m]; n];
        f[0][0] = 1;
        for i in 0..m {f[0][i] = 1;}
        for i in 0..n {f[i][0] = 1;}
        for i in 1..n {
            for j in 1..m {
                f[i][j] = f[i-1][j] + f[i][j-1];
            }
        }
        f[n-1][m-1]
    }
}

struct Solution;


#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_unique_paths() {
        let test_cases = vec![
            (2,2,2),
            (1,1,1),
            (7, 3, 28),
            (3,2, 3),
        ];
        for (m,n,expect) in test_cases {
            assert_eq!(Solution::unique_paths(m,n), expect, "m: {}, n: {}", m, n);
        }
    }
}