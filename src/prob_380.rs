use std::collections::HashMap;
use rand::{thread_rng, Rng, rngs::ThreadRng};

struct RandomizedSet {
    map: HashMap<i32, usize>,
    list: Vec<i32>,
    gen: ThreadRng,
}


/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl RandomizedSet {

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
        if self.map.contains_key(&val) {
            return false;
        }
        self.list.push(val);
        self.map.insert(val, self.list.len()-1);
        true
    }

    /** Removes a value from the set. Returns true if the set contained the specified element. */
    fn remove(&mut self, val: i32) -> bool {
        if !self.map.contains_key(&val) {
            return false;
        }
        let &idx = self.map.get(&val).unwrap();
        if idx+1 == self.list.len() {
            self.map.remove(&val);
            self.list.pop();
            return true;
        }
        let last = self.list.pop().unwrap();
        *self.map.get_mut(&last).unwrap() = idx;
        self.list[idx] = last;
        self.map.remove(&val);
        true
    }

    /** Get a random element from the set. */
    fn get_random(&mut self) -> i32 {
        let idx = self.gen.gen_range(0, self.list.len());
        self.list[idx]
    }
}