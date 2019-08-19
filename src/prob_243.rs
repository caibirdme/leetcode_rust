use std::collections::HashMap;
use std::cmp::min;

impl Solution {
    pub fn shortest_distance(words: Vec<String>, word1: String, word2: String) -> i32 {
        if word1 == word2 {
            return 0;
        }
        let n = words.len();
        let mut mp = HashMap::new();
        for (idx,word) in words.into_iter().enumerate() {
            let list = mp.entry(word).or_insert(vec![]);
            list.push(idx);
        }
        let pos_1 = mp.get(&word1).unwrap();
        let pos_2 = mp.get(&word2).unwrap();
        let mut ans = n;
        for p1 in pos_1 {
            for p2 in pos_2 {
                if *p1 > *p2 {
                    ans = min(ans, *p1-*p2);
                } else {
                    ans = min(ans, *p2-*p1);
                }
            }
        }
        ans as i32
    }

}

struct Solution;