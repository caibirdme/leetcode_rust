use std::cmp::max;
use std::str::from_utf8_unchecked;

impl Solution {
    pub fn shortest_common_supersequence(str1: String, str2: String) -> String {
        let lcs = Self::get_lcs(str1.as_bytes(), str2.as_bytes());
        if lcs.is_empty() {
            return str1 + str2.as_str();
        }
        let up = Self::split(&lcs, str1.as_bytes());
        let down = Self::split(&lcs, str2.as_bytes());
        let mut ans = vec![];
        for &c in &up[0] {
            ans.push(c);
        }
        for &c in &down[0] {
            ans.push(c);
        }
        for (i, c) in lcs.into_iter().enumerate() {
            ans.push(c);
            let li = i+1;
            for &cc in &up[li] {
                ans.push(cc);
            }
            for &cc in &down[li] {
                ans.push(cc);
            }
        }
        unsafe { from_utf8_unchecked(&ans).to_string() }
    }

    fn split(lcs: &Vec<u8>, st: &[u8]) -> Vec<Vec<u8>> {
        let n = lcs.len();
        let mut ans = vec![vec![]; n+1];
        let mut idx = 0;
        for (i, &c) in st.iter().enumerate() {
            if idx == n {
                ans[idx].push(c);
                continue;
            }
            if c != lcs[idx] {
                ans[idx].push(c);
            } else {
                idx += 1;
            }
        }
        ans
    }

    fn get_lcs(str1: &[u8], str2: &[u8]) -> Vec<u8> {
        let (n,m) = (str1.len(), str2.len());
        if n == 0 || m == 0 {
            return vec![];
        }
        let mut f = vec![vec![0; m+1]; n+1];
        for i in 0..n {
            let c1 = str1[i];
            for j in 0..m {
                let c2 = str2[j];
                if c1 == c2 {
                    f[i+1][j+1] = f[i][j]+1;
                } else {
                    f[i+1][j+1] = max(f[i][j+1], f[i+1][j]);
                }
            }
        }
        let mut idx = f[n][m];
        let (mut i, mut j) = (n, m);
        let mut ans = vec![0; idx];
        while i > 0 && j > 0 {
            if str1[i-1] == str2[j-1] {
                idx -= 1;
                ans[idx] = str1[i-1];
                i -= 1;
                j -= 1;
            } else if f[i][j-1] > f[i-1][j] {
                j-=1;
            } else {
                i-=1;
            }
        }
        ans
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_shortest_common_supersequence() {
        let test_cases = vec![
            ("abac", "cab", "cabac"),
            ("abcd", "axbecd", "axbecd"),
            ("abcwd", "axbecd", "axbecwd"),
        ];
        for (s1,s2,expect) in test_cases {
            let ss1 = s1.to_string();
            let ss2 = s2.to_string();
            assert_eq!(Solution::shortest_common_supersequence(ss1, ss2), expect.to_string());
        }
    }
}