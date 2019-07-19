use std::collections::HashMap;

impl Solution {
    pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
        let mut groups: HashMap<[usize; 26], Vec<String>> = HashMap::new();
        for s in strs {
            let mut cmp = [0; 26];
            for c in s.as_bytes() {
                cmp[(c-b'a') as usize] += 1;
            }
            groups.entry(cmp).or_insert(vec![]).push(s);
        }
        groups.into_iter().map(|(_,v)| v).collect()
    }
}

struct Solution;