use std::collections::HashMap;

impl Solution {
    pub fn four_sum_count(a: Vec<i32>, b: Vec<i32>, c: Vec<i32>,d: Vec<i32>) -> i32 {
        let mut ans = 0;
        let mut hash = HashMap::new();
        for &i in a.iter() {
            for &j in b.iter() {
                *hash.entry(i+j).or_insert(0) += 1;
            }
        }
        for &i in c.iter() {
            for &j in d.iter() {
                let t = -(i+j);
                if let Some(&t) = hash.get(&t) {
                    ans += t;
                }
            }
        }
        ans
    }
}

struct Solution;