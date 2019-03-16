impl Solution {
    pub fn super_pow(a: i32, b: Vec<i32>) -> i32 {
        const c: i32 = 1337;
        const phi: i32 = 1140;  // phi(1337) = 1140 according to euler
        let n = b.len();
        if n == 0 {
            return 1;
        }
        let mut exp = 0;
        for digit in b {
            exp = (10*exp+digit) % phi;
        }
        Self::quick_pow(a as i64,exp as i64,c as i64) as i32
    }
    fn quick_pow(mut a: i64, mut b: i64, c: i64) -> i64 {
        let mut res = 1;
        while b > 0 {
            if b & 1 == 1 {
                res = (res*a)%c;
            }
            b >>=1;
            a = (a*a)%c;
        }
        res
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use super::Solution;
    #[test]
    fn test_super_pow() {
        let test_cases = vec![
            (2, vec![4], 16),
            (2, vec![3], 8),
            (2, vec![16], 65536%1337),
            (2, vec![10], 1024),
            (8, vec![0], 1),
        ];
        for (a,b,expect) in test_cases {
            let actual = Solution::super_pow(a,b.clone());
            assert_eq!(expect, actual, "a:{}, b:{:?}", a,b);
        }
    }
}