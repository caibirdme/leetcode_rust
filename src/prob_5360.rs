use std::collections::HashMap;

impl Solution {
    pub fn count_largest_group(n: i32) -> i32 {
        let mut hash = HashMap::new();
        let mut m = 0;
        for i in 1..=n {
            let mut t = i;
            let mut c = 0;
            while t > 0 {
                c += t%10;
                t /= 10;
            }
            let p = hash.entry(c).or_insert(0);
            *p += 1;
            m = m.max(*p);
        }
        let mut ans = 0;
        for &v in hash.values() {
            if v == m {
                ans += 1;
            }
        }
        ans
    }
}

struct Solution;