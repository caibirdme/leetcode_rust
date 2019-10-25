use std::collections::HashMap;
use std::str;

impl Solution {
    pub fn palindrome_pairs(words: Vec<String>) -> Vec<Vec<i32>> {
        if words.is_empty() {
            return vec![];
        }
        let mut empty = None;
        let mut dict = HashMap::new();
        for (i, word) in words.iter().enumerate() {
            if word.is_empty() {
                empty = Some(i);
            } else {
                dict.insert(word, i);
            }
        }
        let mut ans = vec![];
        for (i, word) in words.iter().enumerate() {
            if word.is_empty() {
                continue;
            }
            let wc = word.as_bytes();
            match (empty, Self::is_palindrome(wc)) {
                (Some(j), true) => {
                    ans.push(vec![i as i32, j as i32]);
                    ans.push(vec![j as i32, i as i32]);
                },
                _ => {},
            }
            let w = word.chars().rev().collect::<String>();
            if let Some(&j) = dict.get(&w) {
                if j != i {
                    ans.push(vec![i as i32,j as i32]);
                }
            }
            for k in 1..wc.len() {
                let pre = unsafe {
                    str::from_utf8_unchecked(&wc[..k])
                };

                match (dict.get(&pre.chars().rev().collect::<String>()), Self::is_palindrome(&wc[k..])) {
                    (Some(&j), true) => {
                        ans.push(vec![i as i32, j as i32]);
                    },
                    _ => {}
                }
                let rest = unsafe {
                    str::from_utf8_unchecked(&wc[k..])
                };
                match (dict.get(&rest.chars().rev().collect::<String>()), Self::is_palindrome(&wc[..k])) {
                    (Some(&j), true) => {
                        ans.push(vec![j as i32, i as i32]);
                    },
                    _ => {}
                }
            }
        }
        ans
    }
    fn is_palindrome(word: &[u8]) -> bool {
        let n = word.len();
        if n == 0 {
            return false;
        }
        if n == 1 {
            return true;
        }
        let (mut head, mut tail) = (0, n-1);
        while head < tail {
            if word[head] != word[tail] {
                return false;
            }
            head += 1;
            tail -= 1;
        }
        true
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_palindrome_pairs() {
        let test_cases = vec![
            (vec!["abcd","dcba","lls","s","sssll"], vec![vec![0,1],vec![1,0],vec![3,2],vec![2,4]]),
            (vec!["abcd","dcba","lls","s","sssll", ""], vec![vec![0,1],vec![1,0],vec![3,2],vec![2,4], vec![3,5], vec![5,3]]),
            (vec!["bat","tab","cat"], vec![vec![0,1],vec![1,0]]),
        ];
        for (words, mut expect) in test_cases {
            let mut ans = Solution::palindrome_pairs(words.iter().map(|v| v.to_string()).collect());
            ans.sort();
            expect.sort();
            assert_eq!(ans, expect, "words: {:?}", words);
        }
    }
}