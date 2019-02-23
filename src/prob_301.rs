use std::collections::HashSet;

impl Solution {
    pub fn remove_invalid_parentheses(s: String) -> Vec<String> {
        let n = s.len();
        if n == 0 {
            return vec!["".to_string()];
        }
        let mut left = vec![0; n];
        let mut right = vec![0; n];
        let (l,r) = Self::get_mismatch(s.as_bytes());
        if l == 0 && r == 0 {
            return vec![s];
        }
        let mut ans: HashSet<String> = HashSet::new();
        let mut builder = String::new();
        //let mut f = vec![vec![n as i32; n]; n];
        Self::build(s.as_bytes(), 0, &mut builder, &mut ans, l,r,0);
        ans.into_iter().collect()
    }
    fn build(s: &[u8], idx: usize, cur: &mut String, ans: &mut HashSet<String>, l: i32, r: i32, open: i32) {
        if l < 0 || r < 0 || open < 0 {
            return;
        }
        if idx == s.len() {
            if open == 0 {
                ans.insert(cur.clone());
            }
            return;
        }
        let n = cur.len();
        let c = s[idx];
        if c == b'(' {
            Self::build(s, idx+1, cur, ans, l-1, r, open);
            cur.push('(');
            Self::build(s, idx+1, cur, ans, l, r, open+1);
        } else if c == b')' {
            Self::build(s, idx+1, cur, ans, l, r-1, open);
            cur.push(')');
            Self::build(s, idx+1, cur, ans, l, r, open-1);
        } else {
            cur.push(c as char);
            Self::build(s, idx+1, cur, ans, l, r, open);
        }
        cur.truncate(n);
    }
    pub fn get_mismatch(s: &[u8]) -> (i32, i32) {
        let (mut left, mut right) = (0,0);
        for (i,&c) in s.iter().enumerate() {
            if c != b'(' && c!=b')'{
                continue;
            }
            if c == b'(' {
                left += 1;
            } else {
                if left > 0 {
                    left -= 1;
                } else {
                    right += 1;
                }
            }
        }
        (left, right)
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use super::Solution;
    #[test]
    fn test_get_mismatch() {
        assert_eq!(Solution::get_mismatch("()))((()".as_bytes()), (2,2));
        assert_eq!(Solution::get_mismatch("n".as_bytes()), (0,0));
    }
    #[test]
    fn test_remove_invalid_parentheses() {
        let test_cases = vec![
            ("n", vec!["n".to_string()]),
            (")(", vec!["".to_string()]),
            ("()", vec!["()".to_string()]),
            ("()())()", vec!["()()()".to_string(), "(())()".to_string()]),
        ];
        for (s, mut expect) in test_cases {
            let mut actual = Solution::remove_invalid_parentheses(s.to_string());
            actual.sort();
            expect.sort();
            assert_eq!(actual, expect);
        }
    }
}