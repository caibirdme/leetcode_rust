use std::cmp::min;

impl Solution {
    pub fn is_one_edit_distance(s: String, t: String) -> bool {
        let n = s.len();
        let m = t.len();
        if n == 0 && m == 0 {
            return false;
        }
        if Self::diff(n,m) > 1 {
            return false;
        }
        let ss = s.as_bytes();
        let ts = t.as_bytes();
        for i in 0..min(n,m) {
            if ss[i] != ts[i] {
                if n < m {
                    return ss[i..].eq(&ts[i+1..]);
                }
                if n > m {
                    return ss[i+1..].eq(&ts[i..]);
                }
                return ss[i+1..].eq(&ts[i+1..]);
            }
        }
        n != m
    }

    fn diff(n:usize, m: usize) -> usize {
        if n < m {
            m-n
        } else {
            n-m
        }
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_is_one_edit_distance() {
        let test_cases = vec![
            ("aaa", "aaa", false),
            ("ab","acb", true),
            ("cab", "ad", false),
            ("1203", "1213", true),
            ("asdasp", "adaspc", false),
        ];
        for (a,b,ok) in test_cases {
            assert_eq!(Solution::is_one_edit_distance(a.to_string(), b.to_string()), ok, "a:{}, b:{}",a,b);
        }
    }
}