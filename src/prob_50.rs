impl Solution {
    pub fn my_pow(mut x: f64, n: i32) -> f64 {
        let mut nn = n as i64;
        let mut minus = false;
        if nn < 0 {
            minus = true;
            nn = -nn;
        }
        let mut res = 1f64;
        while nn > 0 {
            if nn&1==1 {
                res *= x;
            }
            x*=x;
            nn>>=1;
        }
        if minus {
            1f64/res
        } else {
            res
        }
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_my_pow() {
        let test_cases = vec![
            (2.00000f64, 10),
            (2.10000, 3),
            (2.00000, -2),
        ];
        for (x,n) in test_cases {
            assert!((x.powi(n) - Solution::my_pow(x,n)).abs() < 1e-8, "x:{}, n:{}",x,n);
        }
    }
}