macro_rules! idx {
    ($e: expr) => {
        ($e-b'a') as usize
    };
}

impl Solution {
    pub fn find_substring_in_wrapround_string(p: String) -> i32 {
        let n = p.len();
        if n <= 1 {
            return n as i32;
        }
        let mut f = vec![0; 26];
        let mut count = vec![0; 26];
        let s = p.as_bytes();
        let mut pos = idx!(s[0]);
        f[pos] = 1;
        count[pos] = 1;
        for i in 1..n {
            let mut t = 1;
            if s[i] == s[i-1]+1 || (s[i-1]==b'z' && s[i]==b'a') {
                t += f[idx!(s[i-1])];
            }
            pos = idx!(s[i]);
            f[pos] = t;
            count[pos] = count[pos].max(t);
        }
        count.into_iter().sum()
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_find_substring_in_wrapround_string() {
        let test_cases = vec![
            ("zabtbc", 9),
            ("a", 1),
            ("cac", 2),
            ("zab", 6),
            ("abzab", 6),
            ("abczab", 9),
        ];
        for (s, expect) in test_cases {
            assert_eq!(Solution::find_substring_in_wrapround_string(s.to_string()), expect, "s: {}", s);
        }
    }
}