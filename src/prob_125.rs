impl Solution {
    pub fn is_palindrome(s: String) -> bool {
        let n = s.len();
        if n <= 1 {
            return true;
        }
        let s = s.as_bytes();
        let (mut l, mut r) = (0, n-1);
        while l<r {
            while l<r && !Self::is_char(s[l]) {
                l+=1;
            }
            while r > l && !Self::is_char(s[r]) {
                r-=1;
            }
            if (s[l] as char).to_lowercase().ne((s[r] as char).to_lowercase()) {
                return false;
            }
            l+=1;
            if r > 0 {
                r-=1;
            }
        }
        true
    }
    fn is_char(c: u8) -> bool {
        (c >= b'a' && c <= b'z') || (c >= b'A' && c<=b'Z') || (c>=b'0' && c<=b'9')
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_is_palindrome() {
        let test_cases = vec![
            ("0P", false),
            (".,", true),
            ("   A a,.   ", true),
            ("A man, a plan, a canal: Panama", true),
            ("race a car", false),
        ];
        for (s, ok) in test_cases {
            assert_eq!(Solution::is_palindrome(s.to_string()), ok, "s: {}", s);
        }
    }
}