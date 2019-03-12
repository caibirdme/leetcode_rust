impl Solution {
    pub fn is_perfect_square(num: i32) -> bool {
        // given that num is positive
        if num < 2 {
            return true
        }
        let num = num as i64;
        let (mut l, mut r) = (2, num/2);
        while l<=r {
            let mid = (l+r)/2;
            let t = mid*mid;
            if t== num {
                return true;
            } else if t<num {
                l = mid+1;
            } else {
                r = mid-1;
            }
        }
        false
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use super::Solution;
    #[test]
    fn test_is_perfect_square() {
        for i in 1..=808201 {
            assert_eq!(Solution::is_perfect_square(i), super::tests::is_sqrt(i as f64));
        }
    }
    fn is_sqrt(n: f64) -> bool {
        use std::f64;
        let q = n.sqrt().floor();
        (q*q-n).abs() <= f64::EPSILON
    }
}