use std::collections::VecDeque;
use std::cmp::Ordering;

struct MaxQueue {
    data: VecDeque<i32>,
    q: Vec<i32>,
    idx: usize,
}

impl MaxQueue {

    fn new() -> Self {
        Self{
            data: VecDeque::new(),
            q: vec![],
            idx: 0,
        }
    }

    fn max_value(&self) -> i32 {
        if self.idx < self.q.len() {
            self.q[self.idx]
        } else {
            -1
        }
    }

    fn push_back(&mut self, value: i32) {
        self.data.push_back(value);
        if self.q.is_empty() || self.idx == self.q.len(){
            self.q.push(value);
            return;
        }
        if value > self.q[self.idx] {
            self.q.clear();
            self.idx = 0;
            self.q.push(value);
            return;
        }
        if let Err(pos) = self.q[self.idx..].binary_search_by(|&probe| {
            if value > probe {
                Ordering::Greater
            } else {
                Ordering::Less
            }
        }) {
            self.q.truncate(self.idx+pos);
            self.q.push(value);
        }
    }

    fn pop_front(&mut self) -> i32 {
        if let Some(v) = self.data.pop_front() {
            if v == self.q[self.idx] {
                self.idx+=1;
            }
            v
        } else {
            -1
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_max_queue() {
        let mut obj = MaxQueue::new();
        obj.push_back(1);
        obj.push_back(2);
        assert_eq!(obj.max_value(), 2);
        assert_eq!(obj.pop_front(), 1);
        assert_eq!(obj.max_value(), 2);
        assert_eq!(obj.pop_front(), 2);
        assert_eq!(obj.max_value(), -1);
        assert_eq!(obj.pop_front(), -1);
        obj.push_back(5);
        obj.push_back(4);
        obj.push_back(3);
        assert_eq!(obj.max_value(),5);
        assert_eq!(obj.pop_front(), 5);
        assert_eq!(obj.max_value(), 4);
        assert_eq!(obj.pop_front(), 4);
        assert_eq!(obj.max_value(), 3);
        obj.push_back(6);
        assert_eq!(obj.max_value(), 6);
        assert_eq!(obj.pop_front(), 3);
        assert_eq!(obj.max_value(), 6);
        assert_eq!(obj.pop_front(), 6);
        assert_eq!(obj.pop_front(), -1);
        assert_eq!(obj.max_value(), -1);
        obj.push_back(6);
        assert_eq!(obj.max_value(), 6);
        obj.push_back(4);
        obj.push_back(4);
        obj.push_back(4);
        obj.push_back(9);
        obj.push_back(4);
        obj.push_back(3);
        assert_eq!(obj.pop_front(), 6);
        assert_eq!(obj.max_value(), 9);
        assert_eq!(obj.pop_front(), 4);
        assert_eq!(obj.pop_front(), 4);
        assert_eq!(obj.pop_front(), 4);
        assert_eq!(obj.max_value(), 9);
        assert_eq!(obj.pop_front(), 9);
        assert_eq!(obj.max_value(), 4);
        assert_eq!(obj.pop_front(), 4);
        assert_eq!(obj.pop_front(), 3);

    }
}