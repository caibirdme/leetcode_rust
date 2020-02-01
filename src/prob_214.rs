use std::str;

impl Solution {
    pub fn shortest_palindrome(s: String) -> String {
        let mut i = 0;
        let n = s.len();
        if n <= 1 {
            return s;
        }
        let ws = s.as_bytes();
        Self::dfs(ws)
    }
    fn dfs(s: &[u8]) -> String {
        let mut i = 0;
        let n = s.len();
        for j in (0..n).rev() {
            if s[i] == s[j] {
                i+=1;
            }
        }
        if i == n {
            return str::from_utf8(s).unwrap().to_string();
        }
        let mut rest = vec![0; n-i];
        rest.clone_from_slice(&s[i..]);
        rest.reverse();
        str::from_utf8(&rest).unwrap().to_string() + Self::dfs(&s[..i]).as_str() + str::from_utf8(&s[i..]).unwrap()
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_shortest_palindrome() {
        let test_cases = vec![
            ("aacecaaa", "aaacecaaa"),
            ("abcd", "dcbabcd"),
        ];
        for (s, expect) in test_cases {
            assert_eq!(Solution::shortest_palindrome(s.to_string()), expect.to_string(), "s: {}", s);
        }
    }
}