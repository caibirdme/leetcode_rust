use std::collections::{HashMap, HashSet};
use rand::{thread_rng, Rng, rngs::ThreadRng};

struct RandomizedCollection {
    map: HashMap<i32, HashSet<usize>>,
    list: Vec<i32>,
    gen: ThreadRng,
}


/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl RandomizedCollection {

    /** Initialize your data structure here. */
    fn new() -> Self {
        Self{
            map: HashMap::new(),
            list: vec![],
            gen: thread_rng(),
        }
    }

    /** Inserts a value to the set. Returns true if the set did not already contain the specified element. */
    fn insert(&mut self, val: i32) -> bool {
        let ok = {
            if self.map.contains_key(&val) {
                false
            } else {
                true
            }
        };

        self.list.push(val);
        self.map.entry(val).or_insert(HashSet::new()).insert(self.list.len()-1);
        ok
    }

    fn remove(&mut self, val: i32) -> bool {
        if !self.map.contains_key(&val) {
            return false;
        }
        let set = self.map.get_mut(&val).unwrap();
        let idx: usize = set.iter().take(1).sum();
        set.remove(&idx);
        if set.is_empty() {
            self.map.remove(&val);
        }
        if idx+1 == self.list.len() {
            self.list.pop();
            return true;
        }
        let last = self.list.pop().unwrap();
        self.list[idx] = last;
        let p = self.map.get_mut(&last).unwrap();
        p.remove(&self.list.len());
        p.insert(idx);
        true
    }

    /** Get a random element from the set. */
    fn get_random(&mut self) -> i32 {
        let mut v = self.gen.gen_range(0, self.list.len());
        self.list[v]
    }
}

#[cfg(test)]
mod tests {
    use super::RandomizedCollection;
    use crate::prob_381::tests::Op::Get;

    enum Op {
        Insert(i32, bool),
        Remove(i32, bool),
        Get(Vec<i32>),
    }

    #[test]
    fn test_RandomizedCollection() {
        let test_cases = vec![
            Op::Insert(10, true),
            Op::Insert(10, false),
            Op::Insert(20, true),
            Op::Insert(20, false),
            Op::Insert(30, true),
            Op::Insert(30, false),
            Op::Remove(10, true),
            Op::Remove(10, true),
            Op::Remove(30, true),
            Op::Remove(30, true),
            Get(vec![20]),
            Get(vec![20]),
            Get(vec![20]),
            Get(vec![20]),
            Get(vec![20]),
            Op::Insert(10, true),
            Op::Get(vec![10, 20]),
            Op::Remove(30, false),
            Op::Remove(20, true),
            Op::Remove(20, true),
            Op::Get(vec![10,]),
        ];
        let mut obj = RandomizedCollection::new();
        for cmd in test_cases {
            match cmd {
                Op::Get(arr) => {
                    let x = obj.get_random();
                    assert_eq!(arr.contains(&x), true);
                },
                Op::Insert(x, ok) => assert_eq!(obj.insert(x), ok),
                Op::Remove(x, ok) => assert_eq!(obj.remove(x), ok),
            }
        }
    }
}