impl Solution {
    pub fn num_squares(mut n: i32) -> i32 {
        while n%4 == 0 {n>>=2;}
        if n%8==7 {
            return 4;
        }
        let un = n as f64;
        if Self::is_sqrt(un) {
            return 1;
        }
        for i in 1..=un.sqrt() as usize {
            let r = n as usize - i*i;
            if Self::is_sqrt(r as f64) {
                return 2;
            }
        }
        3
    }
    fn is_sqrt(f: f64) -> bool {
        use std::f64;
        let t = f.sqrt().floor();
        (t*t-f).abs() <= f64::EPSILON
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use super::Solution;
    #[test]
    fn test_num_squares() {
        assert_eq!(
            Solution::num_squares(11),
            3
        );
    }
}