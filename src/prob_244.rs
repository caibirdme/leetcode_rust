use std::collections::HashMap;
use std::cmp::min;

struct WordDistance {
    dis: HashMap<String, Vec<usize>>,
    n: usize,
}


/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl WordDistance {

    fn new(words: Vec<String>) -> Self {
        let n = words.len();
        let mut dis = HashMap::new();
        for (idx, word) in words.into_iter().enumerate() {
            let list = dis.entry(word).or_insert(vec![]);
            list.push(idx);
        }
        Self{
            dis,
            n,
        }
    }

    fn shortest(&self, word1: String, word2: String) -> i32 {
        let pos_1 = self.dis.get(&word1).unwrap();
        let pos_2 = self.dis.get(&word2).unwrap();
        let mut ans = self.n;
        for p1 in pos_1 {
            for p2 in pos_2 {
                if *p1 > *p2 {
                    ans = min(ans, *p1-*p2);
                } else {
                    ans = min(ans, *p2-*p1);
                }
            }
        }
        ans as i32
    }
}