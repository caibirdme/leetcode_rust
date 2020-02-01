impl Solution {
    pub fn min_cut(s: String) -> i32 {
        let n = s.len();
        if n <= 1 {
            return 0;
        }
        let ws = s.as_bytes();
        let mut mem = vec![vec![None; n]; n];
        let mut f: Vec<i32> = vec![0; n];
        for i in 1..n {
            if Self::is_palindrome(ws, 0, i, &mut mem) {
                f[i] = 0;
                continue;
            }
            let mut t: i32 = std::i32::MAX;
            for j in 1..i {
                if Self::is_palindrome(ws, j, i, &mut mem) {
                    t = t.min(f[j-1]+1);
                }
            }
            f[i] = t.min(f[i-1]+1);
        }
        f[n-1]
    }
    fn is_palindrome(s: &[u8], x: usize, y: usize, f: &mut Vec<Vec<Option<bool>>>) -> bool {
        if x >= y {
            return true;
        }
        if s[x] != s[y] {
            return false;
        }
        if let Some(ok) = f[x][y] {
            return ok;
        }
        let t = Self::is_palindrome(s, x+1, y-1, f);
        f[x][y] = Some(t);
        t
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_min_cut() {
        let test_cases = vec![
            ("aab", 1),
            ("abcecbaaa", 1),
            ("abcecbaaeab", 2),
            ("bacdedcatg", 3),
        ];
        for (s, expect) in test_cases {
            assert_eq!(Solution::min_cut(s.to_string()), expect, "s: {}", s);
        }
    }
}