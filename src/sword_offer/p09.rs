struct CQueue {
    normal: Vec<i32>,
    reverse: Vec<i32>,
}

impl CQueue {

    fn new() -> Self {
        Self{
            normal: vec![],
            reverse: vec![],
        }
    }

    fn append_tail(&mut self, value: i32) {
        self.normal.push(value);
    }

    fn delete_head(&mut self) -> i32 {
        if self.reverse.is_empty() {
            while let Some(v) = self.normal.pop() {
                self.reverse.push(v);
            }
        }
        self.reverse.pop().unwrap_or(-1)
    }
}