impl Solution {
    pub fn permutation(mut s: String) -> Vec<String> {
        let n = s.len();
        if n == 0 {
            return vec![];
        }
        if n == 1 {
            return vec![s];
        }
        let bs = unsafe{s.as_bytes_mut()};
        bs.sort();
        let mut ans = vec![];
        let mut used = vec![false; n];
        let mut temp = Vec::with_capacity(n);
        Self::dfs(bs, &mut used, &mut temp, &mut ans);
        ans
    }
    fn dfs(s: &[u8], used: &mut Vec<bool>, temp: &mut Vec<u8>, ans: &mut Vec<String>) {
        if temp.len() == s.len() {
            ans.push(unsafe{std::str::from_utf8_unchecked(&temp).to_string()});
            return;
        }
        for i in 0..s.len() {
            if used[i] {continue;}
            if i > 0 && s[i] == s[i-1] && !used[i-1] {continue;}
            used[i] = true;
            temp.push(s[i]);
            Self::dfs(s, used, temp, ans);
            used[i] = false;
            temp.pop();
        }
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_permutation() {
        let test_cases = vec![
            ("z", vec!["z"]),
            ("abb", vec!["abb", "bab", "bba"]),
            ("abc", vec!["abc","acb","bac","bca","cab","cba"]),
        ];
        for (s, expect) in test_cases {
            assert_eq!(Solution::permutation(s.to_string()), expect, "s: {}", s);
        }
    }
}