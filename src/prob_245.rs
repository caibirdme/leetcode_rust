use std::cmp::min;

impl Solution {
    pub fn shortest_word_distance(words: Vec<String>, word1: String, word2: String) -> i32 {
        if word1 == word2 {
            return Self::find_same(words, word1);
        }
        let mut ans = words.len();
        let mut one = vec![];
        let mut two = vec![];
        for (i,w) in words.into_iter().enumerate() {
            if w == word1 {
                one.push(i);
            } else if w == word2 {
                two.push(i);
            }
        }
        for n_1 in one {
            for &n_2 in &two {
                if n_2 > n_1 {
                    ans = min(ans, n_2-n_1);
                } else {
                    ans = min(ans, n_1-n_2);
                }
            }
        }
        ans as i32
    }
    fn find_same(words: Vec<String>, word: String) -> i32 {
        let mut ans = words.len();
        let mut pre = None;
        let mut cur = None;
        for (i,w) in words.into_iter().enumerate() {
            if w == word {
                if pre.is_none() {
                    pre = Some(i);
                } else if cur.is_none() {
                    ans = min(ans, i-pre.unwrap());
                    cur = Some(i);
                } else {
                    pre = cur;
                    ans = min(ans, i-pre.unwrap());
                    cur = Some(i);
                }
            }
        }
        ans as i32
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_shortest_word_distance() {
        let test_cases = vec![
            (vec!["practice", "makes", "perfect", "coding", "makes"], "makes", "coding", 1),
            (vec!["practice", "makes", "perfect", "coding", "makes"], "makes", "makes", 3),
        ];
        for (words, a,b, expect) in test_cases {
            assert_eq!(Solution::shortest_word_distance(words.into_iter().map(|v| v.to_string()).collect(), a.to_string(), b.to_string()), expect);
        }
    }
}