use std::collections::HashMap;

struct AutocompleteSystem {
    content: Vec<u8>,
    trie: Trie,
    hash: HashMap<String, i32>,
    empty: bool,
}

impl AutocompleteSystem {

    fn new(sentences: Vec<String>, times: Vec<i32>) -> Self {
        let mut hash = HashMap::new();
        let mut trie = Trie::new();
        for (s,t) in sentences.into_iter().zip(times) {
            trie.insert(s.as_bytes());
            hash.insert(s, t);
        }
        Self{
            content: vec![],
            trie,
            hash,
            empty: false,
        }
    }

    fn input(&mut self, c: char) -> Vec<String> {
        let mut ans = vec![];
        if c == '#' {
            *self.hash.entry(unsafe {std::str::from_utf8_unchecked(&self.content).to_string()}).or_insert(0) += 1;
            self.empty = false;
            self.trie.insert(&self.content);
            self.content.clear();
            return vec![];
        } else {
            self.content.push(c as u8);
            if self.empty {
                return vec![];
            }
            ans = self.trie.find_prefix(&self.content, 0)
        }
        if ans.is_empty() {
            self.empty = true;
        }
        if self.empty {
            return vec![];
        }
        ans.sort_by(|a,b| {
            let ta = *self.hash.get(a).unwrap();
            let tb = *self.hash.get(b).unwrap();
            if ta == tb {
                a.cmp(b)
            } else {
                tb.cmp(&ta)
            }
        });

        (0..ans.len().min(3)).into_iter().map(|i| ans[i].clone()).collect()
    }
}

struct Trie {
    data: HashMap<u8, Option<Box<Trie>>>,
    term: bool,
}

impl Trie {
    fn new() -> Self {
        Self {
            data: HashMap::new(),
            term: false,
        }
    }
    fn insert(&mut self, s: &[u8]) {
        if s.is_empty() {
            return;
        }
        if let Some(node) = self.data.get_mut(&s[0]) {
            if s.len() == 1 {
                node.as_mut().unwrap().term = true;
                return;
            }
            node.as_mut().unwrap().insert(&s[1..]);
        } else {
            let mut node = Self::new();
            if s.len() == 1 {
                node.term = true;
            } else {
                node.insert(&s[1..]);
            }
            self.data.insert(s[0], Some(Box::new(node)));
        }
    }
    fn find_prefix(&self, s: &[u8], i: usize) -> Vec<String> {
        if i == s.len() {
            let mut prefix = Vec::from(s);
            return self.find_all(&mut prefix);
        }
        if let Some(node) = self.data.get(&s[i]) {
            return node.as_ref().unwrap().find_prefix(s, i+1);
        }
        return vec![]
    }
    fn find_all(&self, prefix: &mut Vec<u8>) -> Vec<String> {
        let mut ans = vec![];
        if self.term {
            ans.push(unsafe{std::str::from_utf8_unchecked(&prefix).to_string()});
        }
        for (&c, node) in self.data.iter() {
            prefix.push(c);
            let mut t = node.as_ref().unwrap().find_all(prefix);
            if !t.is_empty() {
                ans.append(&mut t);
            }
            prefix.pop();
        }
        ans
    }
}

#[cfg(test)]
mod tests {
    use super::AutocompleteSystem;

    #[test]
    fn test_input() {
        let test_cases = vec![
            (
                vec!["i love you", "island", "ironman", "i love leetcode"],
                vec![5,3,2,2],
                vec![
                    ('i', vec!["i love you", "island","i love leetcode"]),
                    (' ', vec!["i love you","i love leetcode"]),
                    ('a', vec![]),
                    ('#', vec![]),
                    ('i', vec!["i love you", "island","i love leetcode"]),
                    (' ', vec!["i love you","i love leetcode", "i a"]),
                    ('a', vec!["i a"]),
                    ('#', vec![]),
                    ('i', vec!["i love you", "island","i a"]),
                    (' ', vec!["i love you","i a", "i love leetcode"]),
                    ('a', vec!["i a"]),
                    ('#', vec![]),
                ],
            ),
            (
                vec!["i love you", "island", "ironman", "i love leetcode"],
                vec![5,3,2,2],
                vec![
                    ('i', vec!["i love you", "island","i love leetcode"]),
                    (' ', vec!["i love you","i love leetcode"]),
                    ('l', vec!["i love you","i love leetcode"]),
                    ('o', vec!["i love you","i love leetcode"]),
                    ('v', vec!["i love you","i love leetcode"]),
                    ('e', vec!["i love you","i love leetcode"]),
                    (' ', vec!["i love you","i love leetcode"]),
                    ('l', vec!["i love leetcode"]),
                    ('c', vec![]),
                    ('#', vec![]),
                    ('i', vec!["i love you", "island","i love leetcode"]),
                    (' ', vec!["i love you","i love leetcode", "i love lc"]),
                    ('l', vec!["i love you","i love leetcode", "i love lc"]),
                    ('o', vec!["i love you","i love leetcode", "i love lc"]),
                    ('v', vec!["i love you","i love leetcode", "i love lc"]),
                    ('e', vec!["i love you","i love leetcode", "i love lc"]),
                    (' ', vec!["i love you","i love leetcode", "i love lc"]),
                    ('y', vec!["i love you"]),
                    ('#', vec![]),
                ],
            ),
        ];
        for (s, t, tc) in test_cases {
            let mut obj = AutocompleteSystem::new(s.iter().map(|v| v.to_string()).collect(), t);
            for (c, expect) in tc {
                assert_eq!(obj.input(c), expect, "c:{}", c);
            }
        }
    }
}