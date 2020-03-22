impl Solution {
    pub fn repeated_substring_pattern(s: String) -> bool {
        let n = s.len();
        if n == 1 {
            return false;
        }
        let p = s.as_bytes();
        let mut next = vec![-1; n];
        let mut i = 0;
        let mut j = -1;
        while i+1 < n {
            if p[i as usize+1] == p[(j+1) as usize] {
                i+=1;
                j+=1;
                next[i as usize] = j;
            } else if j == -1 {
                i += 1;
            } else {
                j = next[j as usize];
            }
        }
        next[n-1] >= 0 && n as i32 % (n as i32-next[n-1]-1) == 0
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use super::Solution;
    #[test]
    fn test_repeated_substring_pattern() {
        let test_cases = vec![
            ("aa", true),
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