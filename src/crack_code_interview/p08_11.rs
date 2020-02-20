use std::cmp::min;

impl Solution {
    pub fn ways_to_change(n: i32) -> i32 {
        let mut f = vec![vec![0;n as usize+1]; 4];
        let un = n as usize;
        const m : i64 = 1000000007;
        let d = [5,10,25];
        for i in 0..3 {
            f[i][0] = 1;
        }
        for j in 1..5 {
            f[0][j] = 1;
        }
        for j in 5..=un {
            f[0][j] = 1+f[0][j-5];
        }
        for i in 1..3 {
            for j in 1..=min(d[i]-1 as usize, un) {
                f[i][j] = f[i-1][j];
            }
            for j in d[i] as usize..=un {
                f[i][j] = (f[i-1][j]+f[i][j-d[i] as usize])%m;
            }
        }
        f[2][un] as i32
    }
}

struct Solution;
#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_ways_to_change() {
        let test_cases = vec![
            (6, 2),
            (10, 4),
            (5, 2),
            (15, 6),
        ];
        for (n, expect) in test_cases {
            assert_eq!(Solution::ways_to_change(n), expect, "n: {}", n);
        }
    }
}