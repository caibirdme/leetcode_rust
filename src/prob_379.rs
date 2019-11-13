use std::collections::{BinaryHeap, HashSet};
use std::cmp::Reverse;

struct PhoneDirectory {
    pq: BinaryHeap<Reverse<i32>>,
    used: HashSet<i32>,
    maxNumbers: i32,
}


/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl PhoneDirectory {

    /** Initialize your data structure here
        @param maxNumbers - The maximum numbers that can be stored in the phone directory. */
    fn new(maxNumbers: i32) -> Self {
        let mut pq = BinaryHeap::new();
        for i in 0..maxNumbers {
            pq.push(Reverse(i));
        }
        Self{
            pq,
            used: HashSet::new(),
            maxNumbers,
        }
    }

    /** Provide a number which is not assigned to anyone.
        @return - Return an available number. Return -1 if none is available. */
    fn get(&mut self) -> i32 {
        match self.pq.pop() {
            Some(v) => {
                self.used.insert(v.0);
                v.0
            },
            None => {
                -1
            }
        }
    }

    /** Check if a number is available or not. */
    fn check(&self, number: i32) -> bool {
        if number < self.maxNumbers && !self.used.contains(&number) {
            return true;
        }
        false
    }

    /** Recycle or release a number. */
    fn release(&mut self, number: i32) {
        if number >= self.maxNumbers || !self.used.contains(&number){
            return;
        }
        self.used.remove(&number);
        self.pq.push(Reverse(number));
    }
}

#[cfg(test)]
mod tests {
    use super::PhoneDirectory;

    #[test]
    fn test_PhoneDirectory() {
        let mut obj = PhoneDirectory::new(3);
        assert_eq!(obj.get(), 0);
        assert_eq!(obj.get(), 1);
        obj.release(2);
        assert_eq!(obj.check(2), true);
        assert_eq!(obj.get(), 2);
        assert_eq!(obj.get(), -1);
    }
}