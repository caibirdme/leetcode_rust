use std::collections::HashMap;

impl Solution {
    pub fn find_substring(s: String, words: Vec<String>) -> Vec<i32> {
        if words.is_empty() || s.is_empty() {
            return vec![];
        }
        let word_len = words[0].len();
        let target_len = words.len();
        if s.len() < target_len*word_len {
            return vec![];
        }
        let mut words_map = HashMap::new();
        for word in words.iter() {
            *words_map.entry(word.clone()).or_insert(0) += 1;
        }
        let s_len = s.len();
        let last_idx = s_len-target_len*word_len;
        let bytes = s.as_bytes();
        let mut ans = vec![];
        let mut current = HashMap::new();
        let mut count = 0;
        for i in 0..=last_idx {
            let mut j = i;
            let mut success = true;
            current.clear();
            for _ in 0..target_len {
                let next_j = j+word_len;
                let cs = unsafe {std::str::from_utf8_unchecked(&bytes[j..next_j]).to_string()};
                j = next_j;
                if !words_map.contains_key(&cs) {
                    success = false;
                    break;
                }
                let p = current.entry(cs.clone()).or_insert(0);
                *p += 1;
                if *p > *words_map.get(&cs).unwrap() {
                    success = false;
                    break;
                }
            }
            if success && current == words_map {
                ans.push(i as i32);
            }
        }
        ans
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_find_substring() {
        let test_cases = vec![
            ("a", vec!["a", "a"], vec![]),
            ("lingmindraboofooowingdingbarrwingmonkeypoundcake", vec!["fooo","barr","wing","ding","wing"], vec![13]),
            ("barfoothefoobarman", vec!["foo","bar"], vec![0,9]),
            ("wordgoodgoodgoodbestword", vec!["word","good","best","word"], vec![]),
            ("wordgoodwordgoodbestwordword", vec!["word","good","best","word"], vec![8, 12]),

        ];
        for (s, words, expect) in test_cases {
            let w = words.iter().map(|s| s.to_string()).collect();
            assert_eq!(Solution::find_substring(s.to_string(), w), expect, "s: {}, words: {:?}", s, words);
        }
    }
}