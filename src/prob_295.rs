use std::collections::BinaryHeap;
use std::cmp::Reverse;

struct MedianFinder {
    left: BinaryHeap<i32>,
    right: BinaryHeap<Reverse<i32>>
}


/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MedianFinder {

    /** initialize your data structure here. */
    pub fn new() -> Self {
        Self{
            left: BinaryHeap::new(),
            right: BinaryHeap::new(),
        }
    }

    pub fn add_num(&mut self, num: i32) {
        if self.left.is_empty() {
            self.left.push(num);
            return;
        }
        let l_max = *self.left.peek().unwrap();
        if self.left.len() > self.right.len() {
            if num > l_max {
                self.right.push(Reverse(num));
            } else {
                self.left.push(num);
                self.right.push(Reverse(self.left.pop().unwrap()));
            }
        } else {
            if num <= l_max {
                self.left.push(num);
            } else {
                self.right.push(Reverse(num));
                self.left.push(self.right.pop().unwrap().0);
            }
        }

    }

    pub fn find_median(&self) -> f64 {
        if self.left.is_empty() {
            return 0f64;
        }
        if self.left.len() == self.right.len() {
            (*self.left.peek().unwrap() + self.right.peek().unwrap().0) as f64 / 2.0
        } else {
            *self.left.peek().unwrap() as f64
        }
    }
}

/**
 * Your MedianFinder object will be instantiated and called as such:
 * let obj = MedianFinder::new();
 * obj.add_num(num);
 * let ret_2: f64 = obj.find_median();
 */

#[cfg(test)]
mod tests {
    use super::MedianFinder;
    #[test]
    fn test_find_median_from_data_stream() {
        let mut m = MedianFinder::new();
        m.add_num(1);
        m.add_num(2);
        assert_eq!(m.find_median(), 1.5f64);
        m.add_num(3);
        assert_eq!(m.find_median(), 2f64);
        m.add_num(1);
        m.add_num(0);
        assert_eq!(m.find_median(), 1f64);
    }
}