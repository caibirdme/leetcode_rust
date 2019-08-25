use std::collections::HashMap;

impl Solution {
    pub fn can_permute_palindrome(s: String) -> bool {
        if s.is_empty() {
            return true;
        }
        let mut count = HashMap::new();
        for c in s.chars() {
            let p = count.entry(c).or_insert(0);
            *p += 1;
        }
        let mut odd = 0;
        for &v in count.values() {
            if v & 1 == 1 {
                odd+=1;
                if odd > 1 {
                    return false;
                }
            }
        }
        true
    }
}

struct Solution;