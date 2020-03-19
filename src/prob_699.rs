use std::collections::{BTreeSet, HashMap};
use std::cmp::max;

struct SegmentTree {
    lch: Option<Box<SegmentTree>>,
    rch: Option<Box<SegmentTree>>,
    val: i32,
    delta: i32,
}

impl SegmentTree {
    fn new(n: usize) -> Self {
        *Self::cons(1, n).unwrap()
    }
    fn new_leaf() -> Option<Box<SegmentTree>> {
        Some(Box::new(SegmentTree{
            lch: None,
            rch: None,
            val:0,
            delta:0,
        }))
    }
    fn cons(l: usize, r: usize) -> Option<Box<SegmentTree>> {
        if l == r {
            return Self::new_leaf();
        }
        let m = (l+r)/2;
        Some(Box::new(SegmentTree{
            lch: Self::cons(l, m),
            rch: Self::cons(m+1, r),
            val: 0,
            delta: 0,
        }))
    }
    fn search(&self, l: usize, r: usize, left: usize, right: usize) -> i32 {
        if l == left && r == right {
            if self.delta > 0 {
                return self.delta;
            }
            return self.val;
        }
        if self.delta > 0 {
            return self.delta;
        }
        let m = (l+r)/2;
        if right <= m {
            self.lch.as_ref().unwrap().search(l, m, left, right)
        } else if left > m {
            self.rch.as_ref().unwrap().search(m+1, r, left, right)
        } else {
            max(
                self.lch.as_ref().unwrap().search(l, m, left, m),
                self.rch.as_ref().unwrap().search(m+1, r, m+1, right),
            )
        }
    }
    fn update(&mut self, l: usize, r: usize, left: usize, right: usize, h: i32) -> i32 {
        if l == left && r == right {
            if l == r {
                self.val = h;
            } else {
                self.delta = h;
            }
            return h;
        }
        let m = (l+r)/2;
        if self.delta > 0 {
            self.lch.as_mut().unwrap().update(l, m, l, m, self.delta);
            self.rch.as_mut().unwrap().update(m+1, r, m+1, r, self.delta);
            self.delta = 0;
        }
        if right <= m {
            let lmax = self.lch.as_mut().unwrap().update(l, m, left, right, h);
            self.val = self.val.max(lmax);
        } else if left > m {
            let rmax = self.rch.as_mut().unwrap().update(m+1, r, left, right, h);
            self.val = self.val.max(rmax);
        } else {
            let lmax = self.lch.as_mut().unwrap().update(l, m, left, m, h);
            let rmax = self.rch.as_mut().unwrap().update(m+1, r, m+1, right, h);
            self.val = max(lmax, rmax);
        }
        self.val
    }
}

impl Solution {
    pub fn falling_squares(positions: Vec<Vec<i32>>) -> Vec<i32> {
        let mut bs = BTreeSet::new();
        for pos in &positions {
            let (x, l) = (pos[0], pos[1]);
            bs.insert(x);
            bs.insert(x+l-1);
        }
        let mut hash = HashMap::new();
        for (i,&v) in bs.iter().enumerate() {
            hash.insert(v, i+1);
        }
        let n = bs.len();
        let mut sgt = SegmentTree::new(n);
        positions.into_iter().map(|pos| {
            let (x, l) = (pos[0], pos[1]);
            let mx = *hash.get(&x).unwrap();
            let my = *hash.get(&(x+l-1)).unwrap();
            let t = sgt.search(1, n, mx, my);
            let v = sgt.update(1, n, mx, my, l+t);
            v
        }).collect()
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_falling_squares() {
        let test_cases = vec![
            (vec![
                vec![9,7],vec![1,9],vec![3,1], vec![1,1],vec![2,2], vec![100,15], vec![15, 13],
            ], vec![7, 16, 17, 17, 19, 19, 20]),
            (vec![
                vec![100, 100], vec![200, 100], vec![199, 2],
            ], vec![100, 100, 102]),
            (vec![
                vec![1,2],vec![2,3],vec![6,1],
            ], vec![2,5,5]),
        ];
        for (positions, expect) in test_cases {
            assert_eq!(Solution::falling_squares(positions.clone()), expect, "positions: {:?}", positions);
        }
    }
}