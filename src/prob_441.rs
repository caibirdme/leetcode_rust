impl Solution {
    pub fn arrange_coins(n: i32) -> i32 {
        if n <= 0 {
            return 0;
        }
        let n = n as i64;
        let t = ((2*n) as f64).sqrt().floor() as i64;
        if t*(t+1)/2 <= n as i64 {
            t as i32
        } else {
            (t-1) as i32
        }
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use super::Solution;
    #[test]
    fn test_arrange_coins() {
        let test_cases = vec![
            (1804289383, 60070),
        ];
        for (n,e) in test_cases {
            assert_eq!(e, Solution::arrange_coins(n), "n:{}", n);
        }
    }
}