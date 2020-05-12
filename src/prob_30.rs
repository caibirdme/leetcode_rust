use std::collections::HashMap;

impl Solution {
    pub fn find_substring(s: String, words: Vec<String>) -> Vec<i32> {
        if words.is_empty() || s.is_empty() {
            return vec![];
        }
        let word_len = words[0].len();
        let target_len = words.len();
        if s.len() < target_len * word_len {
            return vec![];
        }
        let mut ans = vec![];
        let mut count = HashMap::new();
        for w in &words {
            *count.entry(w.as_str()).or_insert(0) += 1;
        }
        let sb = s.as_bytes();
        for i in 0..word_len {
            Self::find(&sb[i..], i, word_len, &count, target_len, &mut ans);
        }
        ans
    }
    fn find(s: &[u8], idx: usize, len: usize, words: &HashMap<&str, i32>, total: usize, ans: &mut Vec<i32>) {
        let mut i = 0;
        let mut j = 0;
        let mut cur = HashMap::new();
        let mut acc = 0;
        while j+len <= s.len() {
            let t = unsafe {std::str::from_utf8_unchecked(&s[j..j+len])};
            j += len;
            if !words.contains_key(t) {
                i = j;
                cur.clear();
                acc = 0;
                continue;
            }
            *cur.entry(t).or_insert(0) += 1;
            acc += 1;
            while *cur.get(&t).unwrap() > *words.get(t).unwrap() {
                let p = unsafe {std::str::from_utf8_unchecked(&s[i..i+len])};
                i += len;
                *cur.get_mut(&p).unwrap() -= 1;
                acc -= 1;
            }
            if acc == total {
                ans.push((i+idx) as i32);
            }
        }
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_find_substring() {
        let test_cases = vec![
            ("barfoothefoobarman", vec!["foo","bar"], vec![0,9]),
            ("a", vec!["a", "a"], vec![]),
            ("lingmindraboofooowingdingbarrwingmonkeypoundcake", vec!["fooo","barr","wing","ding","wing"], vec![13]),
            ("wordgoodgoodgoodbestword", vec!["word","good","best","word"], vec![]),
            ("wordgoodwordgoodbestwordword", vec!["word","good","best","word"], vec![8, 12]),

        ];
        for (s, words, expect) in test_cases {
            let w = words.iter().map(|s| s.to_string()).collect();
            assert_eq!(Solution::find_substring(s.to_string(), w), expect, "s: {}, words: {:?}", s, words);
        }
    }
}