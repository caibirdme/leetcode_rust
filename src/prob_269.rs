use std::collections::{HashMap, HashSet};
use std::cmp::min;

impl Solution {
    pub fn alien_order(words: Vec<String>) -> String {
        let mut graph = HashMap::new();
        let mut count = HashMap::new();
        let mut alphabet = HashSet::new();
        if words.is_empty() {
            return "".to_string();
        }
        if words.len() == 1 {
            let mut used = HashSet::new();
            let mut ans = vec![];
            words[0].chars().into_iter().for_each(|c| {
                let ok = used.contains(&c);
                if !ok {
                    used.insert(c);
                    ans.push(c);
                }
            });
            return ans.into_iter().collect();
        }
        words[0].as_bytes().iter().for_each(|&c| { alphabet.insert(c as char); });
        for i in 1..words.len() {
            let pre = &words[i-1];
            let cur = &words[i];
            let pre = pre.as_bytes();
            let cur = cur.as_bytes();
            cur.iter().for_each(|&c| {alphabet.insert(c as char);});
            if pre.is_empty() || cur.is_empty() {
                return "".to_string();
            }
            for i in 0..min(pre.len(), cur.len()) {
                let p = pre[i] as char;
                let c = cur[i] as char;
                if p != c {
                    let p = graph.entry(p).or_insert(HashSet::new());
                    let before = p.len();
                    p.insert(c);
                    if p.len() > before {
                        let ct = count.entry(c).or_insert(0);
                        *ct += 1;
                    }
                    break;
                }
            }
        }
        let mut ans = vec![];
        loop {
            let mut cur = vec![];
            for c in alphabet.iter() {
                match count.get(c) {
                    Some(v) => {
                        continue;
                    },
                    _ => cur.push(*c),
                }
            }
            if cur.is_empty() {
                break;
            }
            for c in cur.iter() {
                if let Some(g) = graph.remove(c) {
                    for nc in g {
                        let mut should_delete = false;
                        if let Some(v) = count.get_mut(&nc) {
                            *v -= 1;
                            if *v == 0 {
                                should_delete = true;
                            }
                        }
                        if should_delete {
                            count.remove(&nc);
                        }
                    }
                }
                alphabet.remove(c);
            }
            ans.push(cur);
        }
        if count.len() > 0 {
            return "".to_string();
        }
        ans.into_iter().flatten().collect()
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_alien_order() {
        let test_cases = vec![
            //(vec!["za","zb","ca","cb"], "abzc"),
            (vec!["aaabbbccc"], "abc"),
            (vec![
                "wrt",
                "wrf",
                "er",
                "ett",
                "rftt",
            ], "wertf"),
            (vec!["z","x"], "zx"),
            (vec!["z","x","z"], ""),

        ];
        for (words, expect) in test_cases {
            assert_eq!(Solution::alien_order(words.iter().map(|c| c.to_string()).collect()), expect, "words: {:?}", words);
        }
    }
}