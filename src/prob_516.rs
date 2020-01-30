use std::cmp::max;

impl Solution {
    pub fn longest_palindrome_subseq(s: String) -> i32 {
        let n = s.len();
        if n <= 1 {
            return n as i32;
        }
        let mut f = vec![vec![1; n]; n];
        let bs = s.as_bytes();
        for i in 0..n-1 {
            if bs[i] == bs[i+1] {
                f[i][i+1] = 2;
            }
        }
        for len in 3..=n {
            for i in 0..=n-len {
                let j = i+len-1;
                if bs[i] == bs[j] {
                    f[i][j] = f[i+1][j-1]+2;
                } else {
                    f[i][j] = max(f[i][j-1], f[i+1][j]);
                }
            }
        }
        f[0][n-1]
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_longest_palindrome_subseq() {
        let test_cases = vec![
            ("aa", 2),
            ("bbbab", 4),
            ("cbbd", 2),
            ("abccbda", 6),
            ("abcdefg", 1),
            ("abcdefgda", 5),
            ("abcdefgdbqa", 7),
            ("abcdeegdbqa", 8),


        ];
        for (s, expect) in test_cases {
            assert_eq!(Solution::longest_palindrome_subseq(s.to_string()), expect, "s: {}", s);
        }
    }
}