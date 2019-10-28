use std::collections::HashMap;
use std::cmp::{max, min};

impl Solution {
    pub fn length_of_longest_substring_k_distinct(s: String, k: i32) -> i32 {
        if s.is_empty() {
            return 0;
        }
        if k == 0 {
            return 0;
        }
        let uk = k as usize;
        let mut hash: HashMap<u8, i32> = HashMap::new();
        let (mut l, mut r) = (0usize, uk);
        let n = s.len();
        let chars = s.as_bytes();
        for i in 0..min(uk, n) {
            let p = hash.entry(chars[i]).or_insert(0);
            *p += 1;
        }
        let mut ans = min(n,uk);
        while r < n {
            *hash.entry(chars[r]).or_insert(0) += 1;
            if hash.len() > uk && l < r {
                let c = chars[l];
                *hash.get_mut(&c).unwrap() -= 1;
                if *hash.get(&c).unwrap() == 0 {
                    hash.remove(&c);
                }
                l+=1;
            } else {
                ans = max(ans, r-l+1);
            }
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
    fn test_length_of_longest_substring_k_distinct() {
        let test_cases = vec![
            ("a", 26, 1),
            ("aa", 1, 2),
            ("eceba", 2, 3),
            ("aaaaabaaa", 1, 5),
            ("aaaaabaaa", 2, 9),
            ("aaaaabaaacb", 2, 9),


        ];
        for (s, k, expect) in test_cases {
            assert_eq!(Solution::length_of_longest_substring_k_distinct(s.to_string(), k), expect, "s: {}, k: {}", s, k);
        }
    }
}