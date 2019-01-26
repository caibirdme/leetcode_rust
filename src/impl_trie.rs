use std::collections::HashMap;

pub struct Trie {
    root: HashMap<u8, Box<node>>,
}

struct node {
    stop: bool,
    child: HashMap<u8, Box<node>>,
}


/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl Trie {

    /** Initialize your data structure here. */
    pub fn new() -> Self {
        Self{root: HashMap::new()}
    }

    /** Inserts a word into the trie. */
    pub fn insert(&mut self, word: String) {
        if word.len() == 0 {
            return;
        }
        self.insert_u8(word.as_bytes());
    }

    fn insert_u8(&mut self, word: &[u8]) {
        let mut res = &word[..];
        let mut cur = self.root.entry(res[0]).or_insert(Box::new(node{
            stop: false,
            child: HashMap::new(),
        }));
        if res.len() == 1 {
            cur.stop = true;
            return;
        }
        res = &res[1..];
        while res.len() > 0 {
            cur = cur.child.entry(res[0]).or_insert(Box::new(node{
                stop: false,
                child: HashMap::new(),
            }));
            if res.len() == 1 {
                cur.stop = true;
                return;
            }
            res = &res[1..];
        }
    }

    /** Returns if the word is in the trie. */
    pub fn search(&self, word: String) -> bool {
        if word.len() == 0 {
            return false;
        }
        self.search_u8(word.as_bytes())
    }

    fn search_u8(&self, word: &[u8]) -> bool {
        let mut res = &word[..];
        let mut cur = self.root.get(&res[0]);
        while res.len() > 0 {
            match cur.take() {
                Some(ch) => {
                    if res.len() == 1 {
                        return ch.stop;
                    }
                    res = &res[1..];
                    cur = ch.child.get(&res[0]);
                },
                None => {
                  return false;
                },
            }
        }
        false
    }

    /** Returns if there is any word in the trie that starts with the given prefix. */
    pub fn starts_with(&self, prefix: String) -> bool {
        self.starts_with_u8(prefix.as_bytes())
    }

    fn starts_with_u8(&self, prefix: &[u8]) -> bool {
        let mut res = &prefix[..];
        let mut cur = self.root.get(&res[0]);
        while res.len() > 0 {
            match cur.take() {
                Some(ch) => {
                    if res.len() == 1 {
                        return true;
                    }
                    res = &res[1..];
                    cur = ch.child.get(&res[0]);
                },
                None => {
                    return false;
                },
            }
        }
        false
    }
}

/**
 * Your Trie object will be instantiated and called as such:
 * let obj = Trie::new();
 * obj.insert(word);
 * let ret_2: bool = obj.search(word);
 * let ret_3: bool = obj.starts_with(prefix);
 */

#[cfg(test)]
mod tests {
    use super::Trie;

    #[test]
    fn test_trie() {
        let mut tree = Trie::new();
        tree.insert("apple".to_string());
        assert_eq!(tree.search("apple".to_string()), true);
        assert_eq!(tree.search("app".to_string()), false);
        assert_eq!(tree.starts_with("app".to_string()), true);
        tree.insert("app".to_string());
        assert_eq!(tree.search("app".to_string()), true);
        assert_eq!(tree.search("foo".to_string()), false);
        tree.insert("foo".to_string());
        assert_eq!(tree.search("foo".to_string()), true);
        assert_eq!(tree.starts_with("f".to_string()), true);
        assert_eq!(tree.starts_with("fo".to_string()), true);
        assert_eq!(tree.starts_with("foo".to_string()), true);
    }
}