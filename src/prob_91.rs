macro_rules! can_compose {
    ($pre:expr, $c:expr) => {
        ($pre == b'1' || ($pre == b'2' && $c <= b'6'))
    };
}

impl Solution {
    pub fn num_decodings(s: String) -> i32 {
        if s == "0".to_string() {
            return 0;
        }
        let n = s.len();
        if n <= 1 {
            return n as i32;
        }
        let s = s.as_bytes();
        if s[0] == b'0' {
            return 0;
        }
        let (mut a, mut b) = (1,1);
        if s[1] == b'0' {
            if s[0] != b'1' && s[0] != b'2' {
                return 0;
            }
            b = 1;
        } else if can_compose!(s[0], s[1]) {
            b = 2;
        }
        for i in 2..n {
            let mut c = b;
            if s[i] == b'0' {
                if s[i-1] != b'1' && s[i-1] != b'2' {
                    return 0;
                }
                b = a;
                a = 0;
                continue;
            }
            if can_compose!(s[i-1], s[i]) {
                c+=a;
            }
            a = b;
            b = c;
        }
        b
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_num_decodings() {
        let test_cases = vec![
            ("102020126", 3),
            ("1361", 2),
            ("110262019", 4),
            ("11026209", 2),
            ("11026", 2),
            ("110260", 0),
            ("10", 1),
            ("0", 0),
            ("100", 0),
            ("1010", 1),
            ("101", 1),
            ("26", 2),
            ("12", 2),
            ("226", 3),
            ("227", 2),
            ("2261", 3),
            ("2619", 4),
        ];
        for (s, expect) in test_cases {
            assert_eq!(Solution::num_decodings(s.to_string()), expect, "s: {}", s);
        }
    }

}