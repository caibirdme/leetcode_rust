use std::cmp::{max, min};
macro_rules! pos {
    ($e:expr) => {
        (($e-b'A') as usize)
    };
}

impl Solution {
    pub fn character_replacement(s: String, k: i32) -> i32 {
        let n = s.len();
        let k = k as usize;
        if k+1 >= n {
            return n as i32;
        }
        let none = n+10;
        let mut prec = [none; 26];
        let mut pre_idx = vec![none; n];
        let mut ans = k+1;
        let bytes = s.as_bytes();
        prec[pos!(bytes[0])] = 0;
        let mut last = 1;
        for i in 1..n {
            let c = bytes[i];
            let t = pos!(c);
            if prec[t] != none {
                pre_idx[i] = prec[t];
            }
            prec[t] = i;
            if i<=k {
                last = i+1;
                continue;
            }
            if c == bytes[i-1] {
                last += 1;
                ans = max(ans, last);
                continue;
            }
            let mut cur = i;
            let mut rest = k;
            while pre_idx[cur] != none && rest >= 0 {
                let replaced = cur - pre_idx[cur]-1;
                if replaced > rest {
                    break;
                }
                cur = pre_idx[cur];
                rest -= replaced;
            }
            last = i-cur+1+rest;
            ans = max(ans, min(last,i+1));
        }
        ans as i32
    }
}

struct Solution;