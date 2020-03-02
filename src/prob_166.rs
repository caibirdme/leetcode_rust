use std::collections::HashMap;
use std::str::from_utf8_unchecked;

impl Solution {
    pub fn fraction_to_decimal(numerator: i32, denominator: i32) -> String {
        let numerator = numerator as i64;
        let denominator = denominator as i64;
        if numerator == 0 {
            return "0".to_string();
        }
        if numerator  % denominator == 0 {
            return (numerator/denominator).to_string();
        }
        let flag = if (numerator > 0 && denominator > 0) || (numerator < 0 && denominator < 0) {String::new()} else {"-".to_string()};
        let z = Self::abs(numerator/denominator);
        let d = Self::abs(denominator);
        let mut cur = (Self::abs(numerator) % d)*10;
        let mut q: Vec<u8> = vec![];
        let mut hash = HashMap::new();
        while cur != 0 {
            if let Some(&pos) = hash.get(&cur) {
                return flag + z.to_string().as_str() + "." + unsafe {from_utf8_unchecked(&q[..pos])} + "(" + unsafe {from_utf8_unchecked(&q[pos..])} + ")";
            }
            q.push((cur / d) as u8 + b'0');
            hash.insert(cur, q.len()-1);
            cur = (cur % d) * 10;
        }
        flag + z.to_string().as_str() + "." + unsafe {from_utf8_unchecked(&q)}
    }
    fn abs(t: i64) -> i64 {
        if t > 0 {
            t
        } else {
            -t
        }
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_fraction_to_decimal() {
        let test_cases = vec![
            (-1,-2147483648,"0.0000000004656612873077392578125"),
            (-50, -8, "6.25"),
            (-50, 8, "-6.25"),
            (2,7000,"0.000(285714)"),
            (1, 300, "0.00(3)"),
            (1,2,"0.5"),
            (1,3,"0.(3)"),
            (23,15, "1.5(3)"),
            (2,3,"0.(6)"),
        ];
        for (a,b,expect) in test_cases {
            assert_eq!(Solution::fraction_to_decimal(a,b), expect, "a:{}, b:{}",a,b);
        }
    }
}