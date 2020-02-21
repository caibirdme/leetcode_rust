use std::collections::BTreeMap;

struct MyCalendarThree {
    count: BTreeMap<i32, i32>,
}

impl MyCalendarThree {

    fn new() -> Self {
        Self{
            count: BTreeMap::new(),
        }
    }

    fn book(&mut self, start: i32, end: i32) -> i32 {
        *self.count.entry(start).or_insert(0) += 1;
        *self.count.entry(end).or_insert(0) -= 1;
        let mut ans = 0;
        self.count.iter().fold(0, |pre, (_,&v)| {
            let t = pre+v;
            ans = ans.max(t);
            t
        });
        ans
    }
}
