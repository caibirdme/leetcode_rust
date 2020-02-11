use std::collections::{HashSet, HashMap};

impl Solution {
    pub fn ladder_length(begin_word: String, end_word: String, word_list: Vec<String>) -> i32 {
        let mut hash = [HashSet::new(), HashSet::new()];
        hash[0].insert(begin_word.clone());
        hash[1].insert(end_word.clone());
        let mut q = [vec![&begin_word], vec![&end_word]];
        let mut steps = [HashMap::new(), HashMap::new()];
        steps[0].insert(begin_word.clone(), 1);
        steps[1].insert(end_word.clone(), 1);

        let mut mapping = HashMap::new();
        let mut ok = false;
        for list in &word_list {
            if list.eq(&end_word) {
                ok = true;
            }
            for s in Self::split(list) {
                mapping.entry(s).or_insert(vec![]).push(list);
            }
        }
        if !ok {
            return 0;
        }
        let mut head = [0,0];
        while head[0] < q[0].len() || head[1] < q[1].len() {
            for i in 0..=1usize {
                if head[i] < q[i].len() {
                    let cur = q[i][head[i]];
                    head[i]+=1;
                    let next_step = *steps[i].get(cur).unwrap() + 1;
                    for s in Self::split(cur) {
                        if let Some(arr) = mapping.get(&s) {
                            for &next_s in arr {
                                if let Some(&v) = steps[i^1].get(next_s) {
                                    return next_step+v-1;
                                }
                                if !hash[i].contains(next_s) {
                                    hash[i].insert(next_s.clone());
                                    steps[i].insert(next_s.clone(), next_step);
                                    q[i].push(next_s);
                                }
                            }
                        }
                    }
                }
            }
        }
        0
    }
    fn split(s: &String) -> Vec<String> {
        let n = s.len();
        let mut ans = Vec::with_capacity(n);
        for i in 0..n {
            let mut tmp: Vec<u8> = s.as_bytes().into();
            tmp[i] = b'_';
            ans.push(unsafe {std::str::from_utf8_unchecked(&tmp).to_string()});
        }
        ans
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_ladder_length() {
        let test_cases = vec![
            ("hit","cog", vec!["hot","dot","dog","lot","log"], 0),
            ("hit","cog", vec!["hot","dot","dog","lot","log","cog"], 5),
        ];
        for (begin, end, list, expect) in test_cases {
            assert_eq!(Solution::ladder_length(
                begin.to_string(),
                end.to_string(),
                list.iter().map(|v| v.to_string()).collect(),
            ), expect, "begin:{}, end:{}, list:{:?}", begin, end, list);
        }
    }
}