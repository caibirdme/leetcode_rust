impl Solution {
    pub fn partition(s: String) -> Vec<Vec<String>> {
        let n = s.len();
        if n == 0 {
            return vec![];
        }
        if n == 1 {
            return vec![vec![s]];
        }
        let mut mem: Vec<Vec<Option<bool>>> = vec![vec![None; n]; n];
        Self::do_partition(s.as_bytes(), &mut mem, 0, n-1)
    }
    fn do_partition(s: &[u8], mem: &mut Vec<Vec<Option<bool>>>, l:usize,r:usize) -> Vec<Vec<String>> {
        let mut cur = vec![];
        for i in l..r {
            if Self::is_palindrome(s, l, i, mem) {
                let part = &s[l..i+1];
                let st = String::from_utf8_lossy(part).to_string();
                let rest = Self::do_partition(s, mem, i+1, r);
                for mut others in rest {
                    let mut new_vec = vec![st.clone()];
                    new_vec.append(&mut others);
                    cur.push(new_vec);
                }
            }
        }
        if Self::is_palindrome(s, l, r, mem) {
            let part = &s[l..r+1];
            let st = String::from_utf8_lossy(part).to_string();
            cur.push(vec![st]);
        }
        cur
    }
    fn is_palindrome(s: &[u8], l:usize,r:usize, mem: &mut Vec<Vec<Option<bool>>>) -> bool {
        if l == r {
            return true;
        }
        if l+1 == r && s[l] == s[r] {
            mem[l][r] = Some(true);
            return true;
        }
        if let Some(ok) = mem[l][r] {
            return ok;
        }
        if s[l] != s[r] {
            mem[l][r] = Some(false);
            return false;
        }
        let ok = Self::is_palindrome(s, l+1, r-1, mem);
        mem[l][r] = Some(ok);
        ok
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use super::Solution;
    #[test]
    fn test_partition() {
        let test_cases = vec![
            ("aab", vec![vec!["aa", "b"], vec!["a","a","b"]]),
            ("aa", vec![vec!["aa"], vec!["a", "a"]]),
            ("aaa", vec![vec!["aaa"], vec!["a", "a", "a"], vec!["aa", "a"]]),

        ];
        for (s, expect) in test_cases {
            let mut actual = Solution::partition(s.to_string());
            let n = actual.len();
            println!("{:?}", actual);
            assert_eq!(n, expect.len());
        }
    }
}