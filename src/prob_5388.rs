impl Solution {
    pub fn reformat(s: String) -> String {
        let mut d = vec![];
        let mut chars = vec![];
        for &c in s.as_bytes() {
            if c >= b'0' && c <= b'9' {
                d.push(c);
            } else {
                chars.push(c);
            }
        }
        let mut ans = vec![];
        if d.len() > chars.len() {
            if d.len()-chars.len() > 1 {
                return "".to_string();
            }
            let n = chars.len();
            for i in 0..n {
                ans.push(d[i]);
                ans.push(chars[i]);
            }
            ans.push(d[n]);
        } else {
            if chars.len()-d.len() > 1 {
                return "".to_string();
            }
            let n = d.len();
            for i in 0..n {
                ans.push(chars[i]);
                ans.push(d[i]);
            }
            if chars.len() > n {
                ans.push(chars[n]);
            }
        }
        unsafe {std::str::from_utf8_unchecked(&ans).to_string()}
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_reformat() {
        let test_cases = vec![
            ("a0b1c2", "a0b1c2"),
            ("leetcode", ""),
            ("1229857369", ""),
            ("covid2019", "c2o0v1i9d"),
        ];
        for (s,expect) in test_cases {
            assert_eq!(Solution::reformat(s.to_string()), expect, "s: {}", s);
        }
    }
}