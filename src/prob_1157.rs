use std::collections::HashMap;

struct MajorityChecker {
    arr: Vec<i32>,
    threshold: i32,
    maybe: Vec<i32>,
    sum: Vec<Vec<i32>>,
}

impl MajorityChecker {
    fn new(arr: Vec<i32>) -> Self {
        let n = arr.len();
        let threshold = ((2*n) as f64).sqrt() as i32;
        let mut hash = HashMap::new();
        for &v in &arr {
            *hash.entry(v).or_insert(0) += 1;
        }
        let mut maybe = vec![];
        let mut sum = vec![];
        for (&k,&v) in hash.iter() {
            if v > threshold {
                maybe.push(k);
                let mut t = vec![0; n+1];
                for i in 0..n {
                    if arr[i] == k {
                        t[i+1] = t[i]+1;
                    } else {
                        t[i+1] = t[i];
                    }
                }
                sum.push(t);
            }
        }
        Self{
            arr,
            threshold,
            maybe,
            sum,
        }
    }

    fn query(&self, left: i32, right: i32, threshold: i32) -> i32 {
        let s = right-left+1;
        if s <= self.threshold {
            return self.count(left,right,threshold);
        }
        let l = left as usize;
        let r = right as usize;
        for (i, &v) in self.maybe.iter().enumerate() {
            if self.sum[i][r+1]-self.sum[i][l] >= threshold {
                return v;
            }
        }
        -1
    }
    fn count(&self, left: i32, right: i32, threshold: i32) -> i32 {
        let mut i = left as usize;
        let mut acc = 0;
        let r = right as usize;
        let mut t = -1;
        while i <= r {
            if acc == 0 {
                t = self.arr[i];
                acc += 1;
                i+=1;
                continue;
            }
            if self.arr[i] == t {
                acc+=1;
            } else {
                acc-=1;
            }
            i+=1;
        }
        if acc == 0 {
            return -1;
        }
        if (left..=right).into_iter().filter(|&i| self.arr[i as usize]==t).count() as i32 >= threshold {
            t
        } else {
            -1
        }
    }
}

struct TreeChecker {
    t: Option<Box<SegmentTree>>,
    pos: HashMap<i32, Vec<usize>>,
    n: i32,
}

struct SegmentTree {
    lch: Option<Box<SegmentTree>>,
    rch: Option<Box<SegmentTree>>,
    val: i32,
    count: i32,
}

impl SegmentTree {
    fn new(arr: &Vec<i32>, l: i32, r: i32) -> Option<Box<Self>> {
        if l == r {
            return Some(Box::new(SegmentTree{
                lch: None,
                rch: None,
                val: arr[l as usize],
                count: 1,
            }));
        }
        let m = (l+r)/2;
        let lch = Self::new(arr, l, m);
        let rch = Self::new(arr, m+1, r);
        let (lv, lc) = {
            let p = lch.as_ref().unwrap();
            (p.val, p.count)
        };
        let (rv, rc) = {
            let p = rch.as_ref().unwrap();
            (p.val, p.count)
        };
        if lv == rv {
            Some(Box::new(SegmentTree{
                lch,
                rch,
                val: lv,
                count: lc+rc,
            }))
        } else {
            if lc >= rc {
                Some(Box::new(SegmentTree{
                    lch,
                    rch,
                    val: lv,
                    count: lc-rc,
                }))
            } else {
                Some(Box::new(SegmentTree{
                    lch,
                    rch,
                    val: rv,
                    count: rc-lc,
                }))
            }
        }
    }
    fn query(&self, l: i32, r: i32, left: i32, right: i32) -> (i32, i32) {
        if l == left && r == right {
            return (self.val, self.count);
        }
        let m = (l+r)/2;
        if right <= m {
            self.lch.as_ref().unwrap().query(l, m, left, right)
        } else if left > m {
            self.rch.as_ref().unwrap().query(m+1, r, left, right)
        } else {
            let (lv, lc) = self.lch.as_ref().unwrap().query(l, m, left, m);
            let (rv, rc) = self.rch.as_ref().unwrap().query(m+1, r, m+1, right);
            if lc >= rc {
                (lv, lc)
            } else {
                (rv, rc)
            }
        }
    }
}

impl TreeChecker {
    fn new(arr: Vec<i32>) -> Self {
        let t = SegmentTree::new(&arr, 0, arr.len() as i32-1);
        let mut pos = HashMap::new();
        for (i, &v) in arr.iter().enumerate() {
            pos.entry(v).or_insert(vec![]).push(i);
        }
        Self{
            t,
            pos,
            n: arr.len() as i32-1,
        }
    }
    fn query(&self, left: i32, right: i32, threshold: i32) -> i32 {
        let (v, _) = self.t.as_ref().unwrap().query(0, self.n, left, right);
        if let Some(pos) = self.pos.get(&v) {
            match (pos.binary_search(&(left as usize)), pos.binary_search(&(right as usize))) {
                (Err(a), Err(b)) => {
                    if b-a >= threshold as usize {
                        return v;
                    }
                },
                (Ok(a), Ok(b)) => {
                    if b-a+1 >= threshold as usize {
                        return v;
                    }
                },
                (Err(a), Ok(b)) => {
                    if b-a+1 >= threshold as usize {
                        return v;
                    }
                },
                (Ok(a), Err(b)) => {
                    if b-a >= threshold as usize {
                        return v;
                    }
                }
            }
        }
        -1
    }
}

#[cfg(test)]
mod tests {
    use super::MajorityChecker;
    use super::TreeChecker;

    #[test]
    fn test_sqrt() {
        let mut t = MajorityChecker::new(vec![1,1,2,2,1,1]);
        let test_cases = vec![
            (2,3,2, 2),
            (0,5,4, 1),
            (0,3,3, -1),
        ];
        for (l,r,threshold, expect) in test_cases {
            assert_eq!(t.query(l,r, threshold), expect, "l:{}, r:{}, threshold:{}", l, r, threshold);
        }
    }
    #[test]
    fn test_tree() {
        let mut t = TreeChecker::new(vec![1,1,2,2,1,1]);
        let test_cases = vec![
            (0,5,4, 1),
            (2,3,2, 2),
            (0,3,3, -1),
        ];
        for (l,r,threshold, expect) in test_cases {
            assert_eq!(t.query(l,r, threshold), expect, "l:{}, r:{}, threshold:{}", l, r, threshold);
        }
    }
}