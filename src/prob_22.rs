impl Solution {
    pub fn generate_parenthesis(n: i32) -> Vec<String> {
        let mut tmp = vec![];
        let mut ans = vec![];
        Self::dfs(n, 0, &mut tmp, &mut ans);
        ans
    }
    fn dfs(n: i32, open: i32, tmp: &mut Vec<u8>, ans: &mut Vec<String>) {
        if n == 0 {
            for _ in 0..open {
                tmp.push(b')');
            }
            ans.push(unsafe {std::str::from_utf8_unchecked(&tmp).to_string()});
            tmp.truncate(tmp.len()-(open as usize));
            return;
        }
        if open > 0 {
            tmp.push(b')');
            Self::dfs(n, open-1, tmp, ans);
            tmp.pop();
        }
        tmp.push(b'(');
        Self::dfs(n-1, open+1, tmp, ans);
        tmp.pop();
    }
}

struct Solution;