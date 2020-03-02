impl Solution {
    pub fn length_of_longest_substring_two_distinct(s: String) -> i32 {
        let n = s.len();
        if n < 3 {
            return n as i32;
        }
        let bs = s.as_bytes();
        let mut l = 0;
        let mut r = 0;
        let mut ans = 2;
        let mut a = None;
        let mut b = None;
        while r < n {
            if a.is_none() || bs[r] == bs[a.unwrap()] {
                a = Some(r);
            } else if b.is_none() || bs[r] == bs[b.unwrap()] {
                b = Some(r);
            } else {
                let pa = a.unwrap();
                let pb = b.unwrap();
                if pa < pb {
                    l = pa+1;
                    a = Some(r);
                } else {
                    l = pb+1;
                    b = Some(r);
                }
            }
            ans = ans.max(r-l+1);
            r += 1;
        }
        ans as i32
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_length_of_longest_substring_two_distinct() {
        let test_cases = vec![
            ("aac", 3),
            ("abcdcdceee", 5),
            ("abcdef", 2),
            ("eceba", 3),
            ("ccaabbb", 5),
        ];
        for (s, expect) in test_cases {
            assert_eq!(Solution::length_of_longest_substring_two_distinct(s.to_string()), expect, "s:{}", s);
        }
    }
}