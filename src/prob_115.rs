use std::collections::HashMap;

impl Solution {
    pub fn num_distinct(s: String, t: String) -> i32 {
        let n = s.len();
        let m = t.len();
        if n == 0 {
            return 0;
        }
        if m == 0 {
            return 0;
        }
        let mut f = vec![0; m];
        let ss = s.as_bytes();
        let ts = t.as_bytes();
        if ss[0] == ts[0] {
            f[0] = 1;
        }
        let t0 = ts[0];
        let mut table = HashMap::new();
        for j in (1..m).rev() {
            table.entry(ts[j]).or_insert(vec![]).push(j);
        }
        for i in 1..n {
            let c = ss[i];
            if let Some(arr) = table.get(&c) {
                for &pos in arr {
                    f[pos] += f[pos-1]
                }
            }
            if c == t0 {
                f[0] += 1;
            }
        }
        f[m-1]
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_num_distinct() {
        let test_cases = vec![
            ("aaaa", "aa", 6),
            ("aaaa", "a", 4),
            ("babag", "bag", 3),
            ("babgbag", "bag", 5),
            ("rabbbit", "rabbit", 3),
        ];
        for (s,t,expect) in test_cases {
            assert_eq!(Solution::num_distinct(s.to_string(), t.to_string()), expect, "s: {}, t: {}", s, t);
        }
    }
}