use std::collections::HashMap;

struct TrieNode {
    child: HashMap<u8, Option<Box<TrieNode>>>,
    term: bool,
}

impl TrieNode {
    fn new() -> Self {
        Self{
            child: HashMap::new(),
            term: false,
        }
    }
    fn insert(&mut self, s: &[u8]) {
        if s.is_empty() {
            self.term = true;
            return;
        }
        self.child.entry(s[0]).or_insert(Some(Box::new(TrieNode{
            child: HashMap::new(),
            term: false,
        }))).as_mut().unwrap().insert(&s[1..]);
    }
    fn find(&self, s: &[u8], dep: usize) -> (usize, bool) {
        if s.is_empty() {
            return (0, false);
        }
        match self.child.get(&s[0]) {
            Some(v) => {
                if v.as_ref().unwrap().term {
                    (dep, true)
                } else {
                    v.as_ref().unwrap().find(&s[1..], dep+1)
                }
            },
            None => (0, false),
        }
    }
}

struct Trie {
    root: Option<Box<TrieNode>>
}

impl Trie {
    fn new() -> Self {
        Self{
            root: Some(Box::new(TrieNode::new())),
        }
    }
    fn insert(&mut self, s: &[u8]) {
        self.root.as_mut().unwrap().insert(s);
    }
    fn find(&self, s: &[u8]) -> Option<usize> {
        let (idx,ok) = self.root.as_ref().unwrap().find(s, 1);
        if ok {
            Some(idx)
        } else {
            None
        }
    }
}

impl Solution {
    pub fn replace_words(dict: Vec<String>, sentence: String) -> String {
        let mut trie = Trie::new();
        for s in &dict {
            trie.insert(s.as_bytes());
        }
        let words: Vec<&str> = sentence.split(' ').collect();
        let mut ans = vec![];
        for &word in &words {
            let bs = word.as_bytes();
            if let Some(idx) = trie.find(bs) {
                ans.push(unsafe{std::str::from_utf8_unchecked(&bs[..idx]).to_string()});
            } else {
                ans.push(word.to_string());
            }
        }
        ans.join(" ")
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_replace_words() {
        let test_cases = vec![
            (vec!["cat", "bat", "rat"], "the cattle was rattled by the battery", "the cat was rat by the bat"),
        ];
        for (dict, sentence, expect) in test_cases {
            assert_eq!(Solution::replace_words(
                dict.iter().map(|v| v.to_string()).collect(),
                sentence.to_string(),
            ), expect.to_string(), "dict: {:?}, sentence: {}", dict, sentence);
        }
    }
}