struct WordDictionary {
    data: Trie
}


/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl WordDictionary {

    /** Initialize your data structure here. */
    pub fn new() -> Self {
        Self{
            data: Trie::new(),
        }
    }

    /** Adds a word into the data structure. */
    pub fn add_word(&mut self, word: String) {
        if word.len() == 0 {
            return;
        }
        self.data.insert(word.as_bytes());
    }

    /** Returns if the word is in the data structure. A word could contain the dot character '.' to represent any one letter. */
    pub fn search(&self, word: String) -> bool {
        if word.len() == 0 {
            return false;
        }
        self.data.search(word.as_bytes())
    }
}

/**
 * Your WordDictionary object will be instantiated and called as such:
 * let obj = WordDictionary::new();
 * obj.add_word(word);
 * let ret_2: bool = obj.search(word);
 */

struct Trie {
    child: ([Option<Box<Trie>>; 26], bool),
}

impl Trie {
    pub fn new() -> Self {
        Self{
            child: Default::default(),
        }
    }
    pub fn insert(&mut self, word: &[u8]) {
        let mut cur = &mut self.child;
        for &i in word {
            let idx = (i - b'a') as usize;
            let next: &mut Option<Box<Trie>> = unsafe { cur.0.get_unchecked_mut(idx) };
            if next.is_none() {
                *next = Some(Box::new(Trie::new()));
            }
            cur = &mut next.as_mut().unwrap().child;
        }
        cur.1 = true;
    }
    pub fn search(&self, word: &[u8]) -> bool {
        let mut cur = &self.child;
        let mut res = word;
        while res.len() > 0 {
            if res[0] == b'.' {
                for i in 0..26 as usize {
                    let next: &Option<Box<Trie>> = unsafe {cur.0.get_unchecked(i)};
                    if let Some(ch) = next.as_ref() {
                        if ch.search(&res[1..]) {
                            return true;
                        }
                    }
                }
                return false;
            }
            let idx = (res[0]-b'a') as usize;
            let next: &Option<Box<Trie>> = unsafe {cur.0.get_unchecked(idx)};
            if next.is_none() {
                return false;
            }
            cur = &next.as_ref().unwrap().child;
            res = &res[1..];
        }
        cur.1
    }
}

#[cfg(test)]
mod tests {
    use super::WordDictionary;
    #[test]
    fn test_word_dict() {
        let mut dict = WordDictionary::new();
        dict.add_word("bad".to_string());
        dict.add_word("dad".to_string());
        dict.add_word("mad".to_string());
        assert_eq!(
            dict.search("pad".to_string()),
            false
        );
        assert_eq!(
            dict.search("bad".to_string()),
            true
        );
        assert_eq!(
            dict.search(".ad".to_string()),
            true
        );
        assert_eq!(
            dict.search("b..".to_string()),
            true
        );
        assert_eq!(
            dict.search(".a.".to_string()),
            true
        );
        assert_eq!(
            dict.search(".a".to_string()),
            false
        );
        dict.add_word("foobar".to_string());
        assert_eq!(
            dict.search("f....r".to_string()),
            true
        );
    }
}