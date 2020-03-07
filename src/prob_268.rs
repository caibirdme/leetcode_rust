use std::collections::HashMap;

impl Solution {
    pub fn max_product(words: Vec<String>) -> i32 {
        let mut hash = HashMap::new();
        for word in &words {
            let mut t = 0;
            for &c in word.as_bytes() {
                t |= 1<<((c-b'a') as i32);
            }
            let p = hash.entry(t).or_insert(0);
            *p = (*p).max(word.len() as i32);
        }
        let mut ans = 0;
        for (k1,v1) in hash.iter() {
            for (k2, v2) in hash.iter() {
                if *k1 & *k2 == 0 {
                    ans = ans.max((*v1) * (*v2));
                }
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
    fn test_max_product() {
        let test_cases = vec![
            (vec!["abcw","baz","foo","bar","xtfn","abcdef"], 16),
            (vec!["a","ab","abc","d","cd","bcd","abcd"], 4),
            (vec!["a","aa","aaa","aaaa"], 0),
        ];
        for (words, expect) in test_cases {
            assert_eq!(Solution::max_product(words.iter().map(|v| v.to_string()).collect()), expect, "words: {:?}", words);
        }
    }
}