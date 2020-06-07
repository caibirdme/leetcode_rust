struct BrowserHistory {
    q: Vec<String>,
    idx: usize,
}

impl BrowserHistory {

    fn new(homepage: String) -> Self {
        Self{
            q: vec![homepage],
            idx: 0,
        }
    }

    fn visit(&mut self, url: String) {
        self.q.truncate(self.idx+1);
        self.q.push(url);
        self.idx += 1;
    }

    fn back(&mut self, steps: i32) -> String {
        let d = self.idx.min(steps as usize);
        self.idx -= d;
        return self.q[self.idx].clone();
    }

    fn forward(&mut self, steps: i32) -> String {
        let d = self.idx+steps as usize;
        if d >= self.q.len() {
            self.idx = self.q.len()-1;
        } else {
            self.idx = d;
        }
        return self.q[self.idx].clone();
    }
}