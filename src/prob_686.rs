impl Solution {
    pub fn repeated_string_match(a: String, b: String) -> i32 {
        let n = a.len();
        if b.len() < n {
            if a.contains(b.as_str()) {
                return 1;
            }
            if a.repeat(2).contains(b.as_str()) {
                return 2;
            }
            return -1;
        }
        let bs = b.as_bytes();
        for i in n..b.len() {
            if bs[i] != bs[i-n] {
                return -1;
            }
        }
        if let Some(pos) = a.repeat(2).find(unsafe{std::str::from_utf8_unchecked(&bs[..n])}) {
            let t = (b.len()/n) as i32;
            let rest = b.len()%n;
            if pos == 0 {
                if rest > 0 {
                    t+1
                } else {
                    t
                }
            } else {
                if rest <= n-pos {
                    t+1
                } else {
                    t+2
                }
            }
        } else {
            -1
        }
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_repeated_string_match() {
        let test_cases = vec![
            ("abc", "cabcabca", 4),
            ("abc", "cabc", 2),
            ("abc", "cab", 2),
            ("abc", "ca", 2),
            ("abc", "b", 1),
            ("abc", "d", -1),
            ("abc", "abcabc", 2),
            ("abcd", "cdabcdab", 3),
            ("abcd", "bcda", 2),
        ];
        for (a,b,expect) in test_cases {
            assert_eq!(Solution::repeated_string_match(a.to_string(), b.to_string()), expect, "a:{}, b:{}",a,b);
        }
    }
}