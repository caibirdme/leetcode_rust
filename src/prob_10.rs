impl Solution {
    pub fn is_match(s: String, p: String) -> bool {
        if s.is_empty() {
            return p.is_empty() || Self::check(p.as_bytes());
        }
        if p.is_empty() {
            return false;
        }
        let n = s.len();
        let m = p.len();
        let mut f = vec![vec![None; m+1]; n+1];
        Self::do_match(s.as_bytes(), p.as_bytes(), n, m, &mut f)
    }
    fn check(p: &[u8]) -> bool {
        let n = p.len();
        if n < 2 {
            return false;
        }
        if p[1] != b'*' {
            return false;
        }
        n == 2 || Self::check(&p[2..])
    }
    fn do_match(s: &[u8], p: &[u8], i: usize, j: usize, f: &mut Vec<Vec<Option<bool>>>) -> bool {
        if let Some(ok) = f[i][j] {
            return ok;
        }
        if i == 0 {
            if j == 0 {
                return true;
            }
            let ok = Self::check(&p[..j]);
            f[0][j] = Some(ok);
            return ok;
        }
        if j == 0 {
            return false;
        }
        let mut ok = false;
        if p[j-1] == b'*' {
            ok = j>=2 && ((p[j-2] == b'.' || p[j-2] == s[i-1]) && Self::do_match(s,p,i-1,j,f) || Self::do_match(s,p,i,j-2,f));
        } else {
            ok = (p[j-1]==b'.' || s[i-1] == p[j-1]) && Self::do_match(s,p,i-1,j-1,f);
        }
        f[i][j] = Some(ok);
        ok
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_is_match() {
        let test_cases = vec![
            ("mississippi", "mis*is*ip*.", true),
            ("a", ".*..a*", false),
            ("", "a*b*c*c*.*", true),
            ("bbbba", ".*a*a", true),
        ];
        for (s, p, ok) in test_cases {
            assert_eq!(Solution::is_match(s.to_string(), p.to_string()), ok, "s: {}, p: {}",s,p);
        }
    }
}