use std::collections::HashMap;

impl Solution {
    pub fn can_win(mut s: String) -> bool {
        let n = s.len();
        if n == 0 {
            return false;
        }
        let mut hash = HashMap::new();
        Self::dfs(unsafe {s.as_bytes_mut()}, &mut hash)
    }
    fn dfs(s: &mut [u8], f: &mut HashMap<String, bool>) -> bool {
        if s.len() < 2 {
            return false;
        }
        let st = unsafe {std::str::from_utf8_unchecked_mut(s)}.to_string();
        if let Some(&ok) = f.get(&st) {
            return ok;
        } else {
            for i in 0..s.len()-1 {
                if s[i] == b'+' && s[i+1] == b'+' {
                    s[i] = b'-';
                    s[i+1] = b'-';
                    let p2 = Self::dfs(s, f);
                    s[i] = b'+';
                    s[i+1] = b'+';
                    if !p2 {
                        f.insert(st, true);
                        return true;
                    }
                }
            }
        }
        f.insert(st, false);
        false
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_can_win() {
        let test_cases = vec![
            ("+++++++++", false),
            ("++", true),
            ("", false),
            ("---+++--", true),
            ("++++", true),
            ("----", false),
            ("++-++", false),
            ("+++++", false),
            ("++++++", true),
        ];
        for (s, ok) in test_cases {
            assert_eq!(Solution::can_win(s.to_string()), ok, "s: {}", s);
        }
    }
}