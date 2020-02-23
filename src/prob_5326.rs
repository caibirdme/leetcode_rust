const MOD: i64 = 1000000007;

impl Solution {
    pub fn count_orders(n: i32) -> i32 {
        let mut f = vec![0; n as usize+1];
        Self::calc(n as i64, &mut f) as i32
    }
    fn calc(n: i64, f: &mut Vec<i64>) -> i64 {
        if n == 1 {
            return 1;
        }
        if n == 2 {
            return 6;
        }
        if f[n as usize] != 0 {
            return f[n as usize];
        }
        let ans = ((n*(2*n-1)) * Self::calc(n-1, f)) % MOD;
        f[n as usize] = ans;
        ans
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_count_orders() {
        let test_cases = vec![
            (6, 7484400),
            (3,90),
            (1,1),
            (2, 6),
        ];
        for (n,expect) in test_cases {
            assert_eq!(Solution::count_orders(n), expect, "n:{}", n);
        }
    }
}