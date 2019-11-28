impl Solution {
    pub fn divide(dividend: i32, divisor: i32) -> i32 {
        if dividend == 0 {
            return 0;
        }
        /*
        if dividend == divisor {
            return 1;
        }
        */
        let mut positive = false;
        if (dividend > 0 && divisor > 0) || (dividend < 0 && divisor < 0) {
            positive = true;
        }
        if dividend == std::i32::MIN && divisor == -1 {
            return std::i32::MAX;
        }
        let mut dividend = Self::convert(dividend);
        let mut divisor = Self::convert(divisor);
        if dividend < divisor {
            return 0;
        }
        if divisor == 1 {
            if positive {
                return dividend as i32;
            }
            return -(dividend as i32);
        }
        let mut ans = 0;
        let mut pd = 31;
        for i in 1..=31 {
            if 1<<i > divisor {
                pd = i-1;
                break;
            }
        }
        pd = 31 - pd;
        while dividend >= divisor {
            let mut t = 0;
            while t<pd && divisor << t <= dividend {
                t+=1;
            }
            if divisor << t > dividend {
                t -= 1;
            }
            dividend -= divisor << t;
            ans |= 1 << t;
        }
        if positive {
           ans as i32
        } else {
            -ans as i32
        }
    }
    fn convert(x: i32) -> u32 {
        if x > 0 {
            x as u32
        } else {
            let t = 1u32<<31;
            let ux = x as u32;
            let nt = ux - t;
            t - nt
        }
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_divide() {
        let test_cases = vec![
            (-2147483648, -2147483648),
            (-2147483648, -2),
            (7, -3),
            (120, 4),
            (20, 4),
            (91, 13),
            (124890, 2),
            (123121238, 24),
            (-10, 2),
        ];
        for (a,b) in test_cases {
            assert_eq!(Solution::divide(a,b), a/b, "a: {}, b: {}", a,b);
        }
        assert_eq!(Solution::divide(-2147483648, -1), 2147483647);
    }
}