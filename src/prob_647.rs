impl Solution {
    pub fn count_substrings(s: String) -> i32 {
        let n = s.len();
        let bs = s.as_bytes();
        if n < 2 {
            return n as i32;
        }
        let mut f = vec![vec![false; n]; n];
        let mut count = 0;
        for i in 0..n {
            f[i][i] = true;
            count+=1;
        }
        for i in 0..n-1 {
            if bs[i] == bs[i+1] {
                f[i][i+1] = true;
                count+=1;
            }
        }
        for i in 3..=n {
            for j in 0..=n-i {
                let r = j+i-1;
                if bs[j] == bs[r] && f[j+1][r-1] {
                    f[j][r] = true;
                    count+=1;
                }
            }
        }
        count
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_count_substrings() {
        let test_cases = vec![
            ("aaa", 6),
            ("abc", 3),
        ];
        for (s, expect) in test_cases {
            assert_eq!(Solution::count_substrings(s.to_string()), expect, "s: {}", s);
        }
    }
}