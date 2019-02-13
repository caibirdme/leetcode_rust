struct MyStack {
    data: Vec<i32>
}


/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MyStack {

    /** Initialize your data structure here. */
    fn new() -> Self {
        Self{data:vec![]}
    }

    /** Push element x onto stack. */
    fn push(&mut self, x: i32) {
        self.data.push(x);
    }

    /** Removes the element on top of the stack and returns that element. */
    fn pop(&mut self) -> i32 {
        self.data.remove(self.data.len()-1)
    }

    /** Get the top element. */
    fn top(&self) -> i32 {
        *self.data.last().unwrap()
    }

    /** Returns whether the stack is empty. */
    fn empty(&self) -> bool {
        self.data.is_empty()
    }
}