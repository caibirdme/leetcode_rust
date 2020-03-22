impl Solution {
    pub fn longest_prefix(s: String) -> String {
        let n = s.len();
        if n < 2 {
            return "".to_string();
        }
        let bs = s.as_bytes();
        let mut next: Vec<i32> = vec![-1; n];
        let mut i = 0;
        let mut j = -1;
        while i+1 < n {
            if bs[i as usize+1] == bs[(j+1) as usize] {
                i+=1;
                j+=1;
                next[i as usize] = j;
            } else if j == -1 {
                i+=1;
            } else {
                j = next[j as usize];
            }
        }
        unsafe {std::str::from_utf8_unchecked(&bs[..(next[n-1]+1) as usize]).to_string()}
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_longest_prefix() {
        let test_cases = vec![
            ("ababab", "abab"),
            ("level", "l"),
            ("leetcodeleet", "leet"),
            ("a", ""),
            ("abc", ""),
        ];
        for (s, expect) in test_cases {
            assert_eq!(Solution::longest_prefix(s.to_string()), expect, "s: {}", s);
        }
    }
}