impl Solution {
    pub fn parse_ternary(expression: String) -> String {
        let (ans, _) = Self::calc(expression.as_bytes());
        let v = [ans];
        unsafe {std::str::from_utf8_unchecked(&v).to_string()}
    }
    fn calc(s: &[u8]) -> (u8, usize) {
        if s.len() == 1 {
            return (s[0], 1);
        }
        if s[1] == b'?' {
            let (lv, ll) = Self::calc(&s[2..]);
            let (rv, rl) = Self::calc(&s[2+ll+1..]);
            if s[0] == b'T' {
                (lv, ll+rl+3)
            } else {
                (rv, ll+rl+3)
            }
        } else {
            (s[0], 1)
        }
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_parse_ternary() {
        let test_cases = vec![
            ("F?T?F:T?1:2:5", "5"),
            ("T?2:3", "2"),
            ("F?1:T?4:5", "4"),
            ("T?T?F:5:3", "F"),
        ];
        for (expr, expect) in test_cases {
            assert_eq!(Solution::parse_ternary(expr.to_string()), expect, "expr: {}", expr);
        }
    }
}