use std::collections::HashMap;

impl Solution {
    pub fn find_rotate_steps(ring: String, key: String) -> i32 {
        if ring.is_empty() || key.is_empty() {
            return 0;
        }
        let n = ring.len();
        let m = key.len();
        const MAX: usize = std::i32::MAX as usize;
        let mut f = vec![vec![MAX; n]; 2];
        let mut pos = HashMap::new();
        for (i, &c) in ring.as_bytes().iter().enumerate() {
            pos.entry(c).or_insert(vec![]).push(i);
        }
        let rs = ring.as_bytes();
        let ks = key.as_bytes();
        if let Some(arr) = pos.get(&ks[0]) {
            for &i in arr {
                f[1][i] = Self::calc(0, i, n) + 1;
            }
        }
        let mut idx = 0;
        for i in 1..m {
            let nidx = idx ^ 1;
            for j in 0..n {
                f[idx][j] = MAX;
                if rs[j] != ks[i] {
                    continue;
                }
                if let Some(pre) = pos.get(&ks[i-1]) {
                    for &k in pre {
                        f[idx][j] = f[idx][j].min(f[nidx][k] + Self::calc(j, k, n) + 1);
                    }
                }
            }
            idx = nidx;
        }
        let mut ans = std::i32::MAX as usize;
        for &v in f[idx^1].iter() {
            ans = ans.min(v);
        }
        ans as i32
    }
    fn calc(a: usize, b: usize, n: usize) -> usize {
        if a < b {
            (b-a).min(n-b+a)
        } else {
            (a-b).min(n-a+b)
        }
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_find_rotate_steps() {
        let test_cases = vec![
            ("aaa", "aaa", 3),
            ("godding", "godding", 13),
            ("godding","gd", 4),
        ];
        for (s,k,expect) in test_cases {
            assert_eq!(Solution::find_rotate_steps(s.to_string(), k.to_string()), expect, "s:{},k:{},expect:{}",s,k,expect);
        }
    }
}