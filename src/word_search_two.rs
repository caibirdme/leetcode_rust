use std::collections::HashSet;

struct Solution;
impl Solution {
    pub fn find_words(board: Vec<Vec<char>>, words: Vec<String>) -> Vec<String> {
        if board.len() == 0 {
            return vec![];
        }
        let length = unsafe {
            let row: &Vec<char> = board.get_unchecked(0);
            row.len()
        };
        let mut tree = Trie::new();
        for st in &words {
            tree.insert(st.as_bytes());
        }
        let mut visited = d2_graph::new(board.len(), length);
        let mut ans = HashSet::new();
        for (i, row) in board.iter().enumerate() {
            for (j, &ch) in row.iter().enumerate() {
                visited.set_val(i,j,true);
                ans.extend(Solution::find(i,j,"", &tree, &board, &mut visited, length));
                visited.set_val(i,j,false);
            }
        }
        ans.into_iter().collect()
    }

    fn find(i: usize, j: usize, prefix: &str, tree: &Trie, board: &Vec<Vec<char>>, visited: &mut d2_graph, length: usize) -> HashSet<String> {
        let ch: &char = unsafe {
            let row: &Vec<char> = board.get_unchecked(i);
            row.get_unchecked(j)
        };
        let mut ns = prefix.to_string();
        ns.push(*ch);
        if !tree.starts_with(ns.as_bytes()) {
            return HashSet::new();
        }
        let mut ans = HashSet::new();
        if tree.search(ns.as_bytes()) {
            ans.insert(ns.clone());
        }
        if i > 0 && visited.can(i-1, j) {
            visited.set_val(i-1,j, true);
            ans.extend(Solution::find(i-1, j, &ns, tree, board, visited, length));
            visited.set_val(i-1, j, false);
        }
        if j > 0 && visited.can(i, j-1) {
            visited.set_val(i,j-1, true);
            ans.extend(Solution::find(i, j-1, &ns, tree, board, visited,length));
            visited.set_val(i,j-1, false);
        }
        if i+1 < board.len() && visited.can(i+1,j) {
            visited.set_val(i+1,j, true);
            ans.extend(Solution::find(i+1, j, &ns, tree, board, visited,length));
            visited.set_val(i+1,j, false);
        }
        if j+1 < length && visited.can(i,j+1) {
            visited.set_val(i,j+1, true);
            ans.extend(Solution::find(i, j+1, &ns, tree, board, visited,length));
            visited.set_val(i,j+1, false);
        }
        ans
    }
}

struct d2_graph {
    col: usize,
    row: usize,
    data: Vec<Vec<bool>>
}

impl d2_graph {
    pub fn new(col: usize, row: usize) -> Self {
        let mut data = Vec::with_capacity(col);
        for _ in 0..col {
            data.push(vec![false; row]);
        }
        Self{
            col,
            row,
            data,
        }
    }
    pub fn clear(&mut self) {
        for i in 0..self.col {
            unsafe {
                let vals: &mut Vec<bool> = self.data.get_unchecked_mut(i);
                for j in 0..self.row {
                    let val: &mut bool = vals.get_unchecked_mut(j);
                    *val = false;
                }
            }
        }
    }
    pub fn set_val(&mut self, i: usize, j: usize, v: bool) {
        unsafe {
            let vals: &mut Vec<bool> = self.data.get_unchecked_mut(i);
            let val: &mut bool = vals.get_unchecked_mut(j);
            *val = v;
        }
    }
    pub fn can(&mut self, i: usize, j: usize) -> bool {
        unsafe {
            let vals: &mut Vec<bool> = self.data.get_unchecked_mut(i);
            let val: &mut bool = vals.get_unchecked_mut(j);
            *val == false
        }
    }
}

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
    fn visit(&self, word: &[u8]) -> (bool, bool) {
        let mut cur = &self.child;
        for &i in word {
            let idx = (i-b'a') as usize;
            let next:&Option<Box<Trie>> = unsafe {cur.0.get_unchecked(idx)};
            if next.is_none() {
                return (false,false);
            }
            cur = &next.as_ref().unwrap().child;
        }
        (cur.1, true)
    }
    pub fn search(&self, word: &[u8]) -> bool {
        self.visit(word).0
    }
    pub fn starts_with(&self, word: &[u8]) -> bool {
        self.visit(word).1
    }
}

#[cfg(test)]
mod tests {
    use super::{Trie,Solution};
    #[test]
    fn test_trie() {
        let mut tree = Trie::new();
        assert_eq!(
            tree.search("hello".as_bytes()),
            false
        );
        tree.insert("hello".as_bytes());
        assert_eq!(
            tree.search("hello".as_bytes()),
            true
        );
        assert_eq!(
            tree.starts_with("hello".as_bytes()),
            true
        );
        assert_eq!(
            tree.search("hell".as_bytes()),
            false
        );
        assert_eq!(
            tree.starts_with("hell".as_bytes()),
            true
        );
        assert_eq!(
            tree.search("h".as_bytes()),
            false
        );
        assert_eq!(
            tree.starts_with("h".as_bytes()),
            true
        );
        tree.insert("hel".as_bytes());
        assert_eq!(
            tree.search("hel".as_bytes()),
            true
        );
    }

    #[test]
    fn test_find_words() {
        let expect: Vec<String> = vec![];
        assert_eq!(
          Solution::find_words(
              vec!["aa".chars().collect()],
              vec!["aaa".to_string()]
          ),
          expect
        );
        let actual = Solution::find_words(
            vec![
                "aa".chars().collect(),
            ],
            vec!["a".to_string()]
        );
        let expect = vec!["a".to_string()];
        assert_eq!(actual, expect);
        let mut actual = Solution::find_words(
            vec![
                "oaan".chars().collect(),
                "etae".chars().collect(),
                "ihkr".chars().collect(),
                "iflv".chars().collect()
            ],
            vec!["oath".to_string(), "pea".to_string(), "eat".to_string(), "rain".to_string(),"fiihklv".to_string()]
        );
        actual.sort();
        let mut expect = vec!["eat".to_string(), "oath".to_string(), "fiihklv".to_string()];
        expect.sort();
        assert_eq!(actual, expect);
    }
}