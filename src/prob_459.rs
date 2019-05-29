impl Solution {
    pub fn repeated_substring_pattern(s: String) -> bool {
        let n = s.len();
        if n == 1 {
            return false;
        }
        let bytes = s.as_bytes();
        for i in 0..n/2 {
            let length = i-0+1;
            if n%length != 0 {
                continue;
            }
            let mut ok = true;
            for j in i+1..n {
                if bytes[j] != bytes[j-length] {
                    ok = false;
                    break;
                }
            }
            if ok {
                return true;
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
    fn test_repeated_substring_pattern() {
        let test_cases = vec![
            ("ababcababcababdc", false),
            ("ababcababc", true),
            ("ababccab", false),
            ("ababba", false),
            ("a", false),
            ("abab", true),
            ("abcab", false),
            ("abcabcabc", true),
            ("aba", false),
            ("abcabcabcabc", true),
        ];
        for (s, ok) in test_cases {
            assert_eq!(Solution::repeated_substring_pattern(s.clone().to_string()), ok, "{}", s);
        }
    }
}