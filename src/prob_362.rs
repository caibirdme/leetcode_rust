use std::collections::VecDeque;

struct HitCounter {
    dq: VecDeque<i32>,
    total: i32,
}


/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl HitCounter {

    /** Initialize your data structure here. */
    fn new() -> Self {
        Self{
            dq: VecDeque::new(),
            total: 0,
        }
    }

    /** Record a hit.
        @param timestamp - The current timestamp (in seconds granularity). */
    fn hit(&mut self, timestamp: i32) {
        self.dq.push_back(timestamp);
        self.total += 1;
    }

    /** Return the number of hits in the past 5 minutes.
        @param timestamp - The current timestamp (in seconds granularity). */
    fn get_hits(&mut self, timestamp: i32) -> i32 {
        while let Some(&v) = self.dq.front() {
            if timestamp - v >= 300 {
                self.dq.pop_front();
                self.total -= 1;
            } else {
                break;
            }
        }
        self.total
    }
}