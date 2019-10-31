struct SummaryRanges {
    data: Vec<(i32, i32)>
}


/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl SummaryRanges {

    /** Initialize your data structure here. */
    fn new() -> Self {
        Self{data:vec![]}
    }

    fn add_num(&mut self, val: i32) {
        match self.data.binary_search_by(|&(v, _)|  v.cmp(&val)) {
            Err(idx) => {
                if idx == 0 {
                    if self.data.len() == 0 {
                        self.data.push((val, 1));
                    } else if self.data[0].0 == val + 1{
                        self.data[0].0 = val;
                        self.data[0].1 += 1;
                    } else {
                        self.data.insert(0, (val, 1));
                    }
                    return;
                } else if idx == self.data.len() {
                    let max_v = self.data[idx-1].0 + self.data[idx-1].1;
                    if max_v == val {
                        self.data[idx-1].1+=1;
                    } else if val > max_v {
                        self.data.push((val, 1));
                    }
                    return;
                }
                if self.data[idx-1].0 + self.data[idx-1].1 > val {
                    return;
                }
                if self.data[idx].0 == val+1 {
                    self.data[idx].1 += 1;
                    self.data[idx].0 = val;
                } else if self.data[idx-1].0+self.data[idx-1].1 == val {
                    self.data[idx-1].1 += 1;
                } else {
                    self.data.insert(idx, (val,1));
                    return;
                }
                if self.data[idx-1].0+self.data[idx-1].1 == self.data[idx].0 {
                    self.data[idx-1].1 += self.data[idx].1;
                    self.data.remove(idx);
                }
            },
            _ => {}
        }
    }

    fn get_intervals(&self) -> Vec<Vec<i32>> {
        self.data.iter().map(|&(n,l)| vec![n, n+l-1]).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::SummaryRanges;
    #[derive(Debug)]
    enum Operator{
        Add(i32),
        Get(Vec<Vec<i32>>),
    }

    #[test]
    fn test_func() {
        let mut obj = SummaryRanges::new();
        let test_cases = vec![
            Operator::Add(1),
            Operator::Get(vec![vec![1,1]]),
            Operator::Add(3),
            Operator::Add(7),
            Operator::Get(vec![vec![1, 1], vec![3, 3], vec![7, 7]]),
            Operator::Add(2),
            Operator::Get(vec![vec![1,3], vec![7,7]]),
            Operator::Add(6),
            Operator::Get(vec![vec![1,3], vec![6,7]]),
            Operator::Add(4),
            Operator::Add(5),
            Operator::Get(vec![vec![1,7]]),
            Operator::Add(-1),
            Operator::Get(vec![vec![-1,-1], vec![1,7]]),
            Operator::Add(0),
            Operator::Get(vec![vec![-1,7]]),
            Operator::Add(13),
            Operator::Add(15),
            Operator::Get(vec![vec![-1,7], vec![13,13], vec![15,15]]),
            Operator::Add(14),
            Operator::Get(vec![vec![-1,7], vec![13,15]]),
            Operator::Add(14),
            Operator::Get(vec![vec![-1,7], vec![13,15]]),
            Operator::Add(7),
            Operator::Get(vec![vec![-1,7], vec![13,15]]),
            Operator::Add(8),
            Operator::Get(vec![vec![-1,8], vec![13,15]]),

        ];
        for op in test_cases {
            match op {
                Operator::Add(v) => obj.add_num(v),
                Operator::Get(expect) => assert_eq!(obj.get_intervals(), expect),
            }
        }
        let mut new_obj = SummaryRanges::new();
        for i in 0..=2000 {
            new_obj.add_num(i);
        }
        assert_eq!(new_obj.get_intervals(), vec![vec![0, 2000]]);
        obj = SummaryRanges::new();
        for i in 1..=9 {
            obj.add_num(i);
        }
        assert_eq!(obj.get_intervals(), vec![vec![1,9]]);
        obj.add_num(3);
        obj.add_num(5);
        assert_eq!(obj.get_intervals(), vec![vec![1,9]]);
    }
}