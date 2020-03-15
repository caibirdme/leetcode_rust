struct CustomStack {
    data: Vec<i32>,
    cap: usize,
}


/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl CustomStack {

    fn new(maxSize: i32) -> Self {
        Self{
            data: vec![],
            cap: maxSize as usize,
        }
    }

    fn push(&mut self, x: i32) {
        if self.data.len() < self.cap {
            self.data.push(x);
        }
    }

    fn pop(&mut self) -> i32 {
        self.data.pop().unwrap_or(-1)
    }

    fn increment(&mut self, k: i32, val: i32) {
        for i in 0..(k as usize).min(self.data.len()) {
            self.data[i] += val;
        }
    }
}
