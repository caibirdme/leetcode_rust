use std::collections::{HashMap, HashSet};

struct DST {
    parent: Vec<usize>,
}

impl DST {
    fn new(n: usize) -> Self {
        Self {
            parent: (0..n).into_iter().collect(),
        }
    }
    fn get_parent(&mut self, u: usize) -> usize {
        if self.parent[u] == u {
            return u;
        }
        let p = self.get_parent(self.parent[u]);
        self.parent[u] = p;
        p
    }
    fn union(&mut self, a: usize, b: usize) {
        let pa = self.get_parent(a);
        let pb = self.get_parent(b);
        self.parent[pa] = pb;
    }
}

impl Solution {
    pub fn num_similar_groups(a: Vec<String>) -> i32 {
        let set: HashSet<String> = a.into_iter().collect();
        let b = set.into_iter().collect::<Vec<String>>();
        let n = b.len();
        if n == 0 {
            return 0;
        }
        let w = b[0].len();
        let mut dst = DST::new(n);
        if n < w*w {
            for i in 0..n-1 {
                for j in i+1..n {
                    if Self::similar(b[i].as_bytes(), b[j].as_bytes()) {
                        dst.union(i,j);
                    }
                }
            }
        } else {
            let mut hash = HashMap::new();
            for (idx,s) in b.iter().enumerate() {
                let mut q = s.clone();
                let t = unsafe {q.as_bytes_mut()};
                for i in 0..w-1 {
                    for j in i+1..w {
                        if t[i] == t[j] {continue;}
                        t.swap(i,j);
                        hash.entry(unsafe {std::str::from_utf8_unchecked(t).to_string()}).or_insert(vec![]).push(idx);
                        t.swap(i,j);
                    }
                }
            }
            for (i, s) in b.iter().enumerate() {
                if let Some(arr) = hash.get(s) {
                    for &j in arr {
                        dst.union(i,j);
                    }
                }
            }
        }
        (0..n).into_iter().filter(|&i| dst.get_parent(i) == i).count() as i32
    }
    fn similar(a: &[u8], b: &[u8]) -> bool {
        let mut t = 0;
        for (&x,&y) in a.iter().zip(b.iter()) {
            if x != y {
                t += 1;
                if t > 2 {
                    return false;
                }
            }
        }
        t == 2
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_num_similar_groups() {
        let test_cases = vec![
            (vec!["abc", "abc","abc", "abc","abc", "abc","abc", "abc","abc", "abc","abc", "abc","abc", "abc"], 1),
            (vec!["tars","rats","arts","star"], 2),
        ];
        for (a, expect) in test_cases {
            assert_eq!(Solution::num_similar_groups(a.iter().map(|v| v.to_string()).collect()), expect, "a: {:?}", a);
        }
    }
}