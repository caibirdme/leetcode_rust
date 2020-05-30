use std::collections::HashSet;

impl Solution {
    pub fn has_all_codes(s: String, k: i32) -> bool {
        if s.is_empty() {
            return false;
        }
        if k >= s.len() as i32 {
            return false;
        }
        let bs = s.as_bytes();
        if k == 1 {
            let mut one = false ;
            let mut zero = false;
            for &c in bs {
                if c == b'0' {
                    zero = true;
                } else {
                    one = true;
                }
                if zero && one {
                    return true;
                }
            }
            return false;
        }
        let mut v = 0;
        for i in 0..k as usize {
            v = (v << 1) | ((bs[i]-b'0') as i32);
        }
        let mut hash = HashSet::new();
        hash.insert(v);
        let mask = (1 << (k-1)) - 1;
        for i in k as usize..bs.len() {
            v = ((v & mask) << 1) | ((bs[i]-b'0') as i32);
            hash.insert(v);
        }
        for i in 0..=(1<<k)-1 {
            if !hash.contains(&i) {
                return false;
            }
        }
        true
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_has_all_codes() {
        let test_cases = vec![
            ("00110", 2, true),
            ("0110", 1, true),
            ("0000000001011100", 4, false),
            ("00110110", 2, true),
        ];
        for (s, k, ok) in test_cases {
            assert_eq!(Solution::has_all_codes(s.to_string(), k), ok, "s:{}, k:{}", s,k);
        }
    }
}