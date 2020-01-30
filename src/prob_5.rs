impl Solution {
    pub fn longest_palindrome(s: String) -> String {
        let n = s.len();
        if n <= 1 {
            return s;
        }
        let bs = s.as_bytes();
        let mut f = vec![vec![false; n]; n];
        for i in 0..n {
            f[i][i] = true;
        }
        let mut ans = 1;
        let mut pos = 0;
        for i in 0..n-1 {
            if bs[i] == bs[i+1] {
                f[i][i+1] = true;
                ans = 2;
                pos = i;
            }
        }
        for len in 3..=n {
            for i in 0..=n-len {
                let j = i+len-1;
                if bs[i] == bs[j] {
                    f[i][j] = f[i+1][j-1];
                    if f[i][j] {
                        pos = i;
                        ans = len;
                    }
                }
            }
        }
        unsafe {
            std::str::from_utf8_unchecked(&bs[pos..pos+ans]).to_string()
        }
    }
    pub fn longest_palindrome_expand(s: String) -> String {
        let n = s.len();
        if n <= 1 {
            return s;
        }
        let bs = s.as_bytes();
        let mut ans = 1;
        let mut pos = 0;
        for i in 0..n-1 {
            let x = Self::expand(bs, i, i);
            let y = Self::expand(bs, i, i+1);
            if x >= ans {
                ans = x;
                pos = i-x/2;
            }
            if y >= ans {
                ans = y;
                pos = i+1-y/2;
            }
        }
        unsafe {
            std::str::from_utf8_unchecked(&bs[pos..pos+ans]).to_string()
        }
    }
    fn expand(s: &[u8], mut a: usize, mut b: usize) -> usize {
        if s[a] != s[b] {
            return 0;
        }
        let n = s.len();
        while a > 0 && b+1 < n && s[a-1] == s[b+1] {
            a-=1;
            b+=1;
        }
        b-a+1
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_longest_palindrome() {
        let test_cases = vec![
            ("babad", "aba"),
            ("cbbd", "bb"),
            ("cabace", "cabac"),
            ("cabacecabac", "cabacecabac"),
        ];
        for (s, expect) in test_cases {
            assert_eq!(Solution::longest_palindrome_expand(s.to_string()), Solution::longest_palindrome(s.to_string()), "s: {}", s);
        }
    }
}