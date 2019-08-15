struct MyQueue {
    a: Vec<i32>,
    b: Vec<i32>,
}


/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MyQueue {

    /** Initialize your data structure here. */
    fn new() -> Self {
        Self{
            a: vec![],
            b: vec![],
        }
    }

    /** Push element x to the back of queue. */
    fn push(&mut self, x: i32) {
        self.a.push(x);
    }

    /** Removes the element from in front of queue and returns that element. */
    fn pop(&mut self) -> i32 {
        if !self.b.is_empty() {
            self.b.pop().unwrap()
        } else {
            while !self.a.is_empty() {
                self.b.push(self.a.pop().unwrap());
            }
            self.b.pop().unwrap()
        }
    }

    /** Get the front element. */
    fn peek(&mut self) -> i32 {
        if !self.b.is_empty() {
            *self.b.last().unwrap()
        } else {
            while !self.a.is_empty() {
                self.b.push(self.a.pop().unwrap());
            }
            *self.b.last().unwrap()
        }
    }

    /** Returns whether the queue is empty. */
    fn empty(&self) -> bool {
        self.a.is_empty() && self.b.is_empty()
    }
}
