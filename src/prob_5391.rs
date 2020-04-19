impl Solution {
    pub fn num_of_arrays(n: i32, m: i32, k: i32) -> i32 {
        const MOD: i64 = 1000000007;
        if k == 0 {
            return 0;
        }
        let n = n as usize;
        let m = m as usize;
        let K = k as usize;
        let mut f = vec![vec![vec![0; K+1]; m+1]; n];
        f[0][1][1] = 1;
        for j in 1..=m {
            f[0][j][1] = 1 as i64;
        }
        for i in 1..n {
            for j in 1..=m {
                for k in 1..=K {
                    let mut acc = 0;
                    for w in 1..j {
                        acc += f[i-1][w][k-1];
                    }
                    acc %= MOD;
                    acc += f[i-1][j][k] * (j as i64);
                    f[i][j][k] = acc % MOD;
                }
            }
        }
        let mut ans = 0;
        for j in 1..=m {
            ans += f[n-1][j][K];
            ans %= MOD;
        }
        ans as i32
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_num_of_arrays() {
        let test_cases = vec![
            (2,3,1,6),
            (5,2,3,0),
            (37, 17, 7, 418930126),
            (9,1,1,1),
            (50, 100, 25, 34549172),
        ];
        for (n,m,k, expect) in test_cases {
            assert_eq!(Solution::num_of_arrays(n,m,k), expect, "n:{},m:{},k:{}",n,m,k);
        }
    }
}
