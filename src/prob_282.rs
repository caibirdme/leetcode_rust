impl Solution {
    pub fn add_operators(num: String, target: i32) -> Vec<String> {
        let mut res = vec![];
        let mut ans = vec![];
        Self::dfs(num.as_bytes(), 0, 0, 0, target as i64, &mut res, &mut ans);
        ans
    }
    fn dfs(s: &[u8], pre: i64, cur: i64, val:i64, target: i64, res: &mut Vec<String>, ans: &mut Vec<String>) {
        if cur > std::i32::MAX as i64 {
            return;
        }
        if s.is_empty() {
            if val == target && cur == 0{
                ans.push(res[1..].join(""));
            }
            return;
        }
        let current = cur * 10 + (s[0]-b'0') as i64;
        if current > 0 {
            Self::dfs(&s[1..], pre, current, val, target, res, ans);
        }
        res.push("+".to_string());
        res.push(current.to_string());
        Self::dfs(&s[1..], current, 0, val+current, target, res, ans);
        res.pop();
        res.pop();
        if res.len() > 0 {
            res.push("-".to_string());
            res.push(current.to_string());
            Self::dfs(&s[1..], -current, 0, val-current, target, res, ans);
            res.pop();
            res.pop();

            res.push("*".to_string());
            res.push(current.to_string());
            Self::dfs(&s[1..], pre*current, 0, val-pre+pre*current, target, res, ans);
            res.pop();
            res.pop();
        }
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use super::Solution;
    #[test]
    fn test_add_operators() {
        let mut test_cases = vec![
            ("105", 5, vec!["1*0+5".to_string(), "10-5".to_string()]),
            ("232", 8, vec!["2*3+2".to_string(), "2+3*2".to_string()]),
            ("00", 0, vec!["0+0".to_string(), "0-0".to_string(), "0*0".to_string()]),
            ("3456237490", 9191, vec![]),
            ("12", 12, vec!["12".to_string()]),
            ("112", 12, vec!["1*12".to_string()]),
            ("2147483647", 2147483647, vec!["2147483647".to_string()]),
        ];
        for (s,t,mut expect) in test_cases {
            let mut actual = Solution::add_operators(s.to_string(), t,);
            actual.sort();
            expect.sort();
            assert_eq!(actual,expect);
        }
    }
}