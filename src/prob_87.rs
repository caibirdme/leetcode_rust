impl Solution {
    pub fn is_scramble(s1: String, s2: String) -> bool {
        Self::dfs(s1.as_bytes(), s2.as_bytes())
    }
    fn dfs(s1: &[u8], s2: &[u8]) -> bool {
        if s1.len() != s2.len() {
            return false;
        }
        if s1 == s2 {
            return true;
        }
        let n = s1.len();
        if n == 1 {
            return false;
        }
        let mut count = [0; 26];
        for &c in s1 {
            count[(c- b'a') as usize] += 1;
        }
        for &c in s2 {
            count[(c- b'a') as usize] -= 1;
        }
        for &v in count.iter() {
            if v != 0 {
                return false;
            }
        }
        for i in 1..=n-1 {
            if Self::dfs(&s1[..i], &s2[..i]) && Self::dfs(&s1[i..], &s2[i..]) {
                return true;
            }
            if Self::dfs(&s1[..i], &s2[n-i..]) && Self::dfs(&s1[i..], &s2[..n-i]) {
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
    fn test_is_scramble() {
        let test_cases = vec![
            ("great", "rgeat", true),
            ("great", "rgtae", true),
            ("abcde", "caebd", false),
        ];
        for (s1, s2, ok) in test_cases {
            assert_eq!(Solution::is_scramble(s1.to_string(), s2.to_string()), ok, "s1: {}, s2: {}",s1, s2);
        }
    }
}