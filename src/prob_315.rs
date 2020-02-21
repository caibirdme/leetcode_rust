use std::rc::Rc;
use std::cell::RefCell;
use std::collections::hash_map::{HashMap, Entry};

impl Solution {
    pub fn count_smaller(nums: Vec<i32>) -> Vec<i32> {
        let n = nums.len();
        if n == 0 {
            return vec![];
        }
        if n == 1 {
            return vec![0];
        }
        let tree = LineTree::new(0, n - 1, &nums);
        let mut ans = vec![0; n];
        let last = n - 1;
        for i in (0..=n - 2).rev() {
            ans[i] = tree.get_less_num(i + 1, last, nums[i]) as i32;
        }
        ans
    }
    pub fn bit_index_tree(nums: Vec<i32>) -> Vec<i32> {
        let mut hash = HashMap::new();
        let mut copy = nums.clone();
        copy.sort();
        let mut c = 1;
        for &v in &copy {
            if let Entry::Vacant(entry) = hash.entry(v) {
                entry.insert(c);
                c+=1;
            }
        }
        let mut bits = vec![0; hash.len()+1];
        let mut ans = vec![0; nums.len()];
        for i in (0..nums.len()).rev() {
            let v = *hash.get(&nums[i]).unwrap();
            ans[i] = Self::query_bit(&bits, v-1);
            Self::update_bit(&mut bits, v, 1);
        }
        ans
    }
    fn update_bit(bits: &mut Vec<i32>, mut x: usize, v: i32) {
        while x < bits.len() {
            bits[x] += v;
            x = x + Self::low_bit(x);
        }
    }
    fn query_bit(bits: &Vec<i32>, mut x: usize) -> i32 {
        let mut ans = 0;
        loop {
            ans += bits[x];
            if x == 0 {
                break;
            }
            x -= Self::low_bit(x);
        }
        ans
    }
    fn low_bit(x: usize) -> usize {
        (x^(x-1))&x
    }
    pub fn count_smaller_bst(nums: Vec<i32>) -> Vec<i32> {
        let n = nums.len();
        if n == 0 {
            return vec![];
        }
        if n == 1 {
            return vec![0];
        }
        let mut ans = vec![0; n];
        let mut bst = BSTree::new(nums[n-1]);
        for i in (0..=n-2).rev() {
            let cur = nums[i];
            bst.insert(cur);
            ans[i] = bst.search(cur) as i32;
        }
        ans
    }
}

struct BSTree<T> {
    val: T,
    smaller: usize,
    lch: Option<Box<BSTree<T>>>,
    rch: Option<Box<BSTree<T>>>,
}

impl<T: Ord + Copy> BSTree<T> {
    pub fn new(val: T) -> Self {
        Self{
            val,
            smaller: 0,
            lch: None,
            rch: None,
        }
    }
    pub fn insert(&mut self, val: T) {
        if val.lt(&self.val) {
            self.smaller += 1;
            match self.lch.take() {
                Some(mut lch) => {
                    lch.insert(val);
                    self.lch = Some(lch);
                },
                None => self.lch = Some(Box::new(Self::new(val))),
            }
        } else {
            match self.rch.take() {
                Some(mut rch) => {
                    rch.insert(val);
                    self.rch = Some(rch);
                },
                None => {
                    let mut rch = Self::new(val);
                    self.rch = Some(Box::new(rch));
                }
            }
        }
    }
    pub fn search(&self, val: T) -> usize {
        if self.val.eq(&val) {
            self.smaller
        } else if self.val.ge(&val) {
            self.lch.as_ref().unwrap().search(val)
        } else {
            self.smaller + 1 + self.rch.as_ref().unwrap().search(val)
        }
    }
}

struct Solution;

struct LineTree<T> {
    max: T,
    left: usize,
    right: usize,
    lch: Option<Rc<RefCell<LineTree<T>>>>,
    rch: Option<Rc<RefCell<LineTree<T>>>>,
}

impl<T: Ord + Copy + Eq> LineTree<T> {
    pub fn new(l: usize, r: usize, nums: &Vec<T>) -> Self {
        if l == r {
            Self{
                max: nums[l],
                left: l,
                right: r,
                lch: None,
                rch: None,
            }
        } else {
            let mid = (l+r)/2;
            let lch = Rc::new(RefCell::new(Self::new(l, mid, nums)));
            let rch = Rc::new(RefCell::new(Self::new(mid+1, r, nums)));
            Self{
                max: if lch.borrow().max.gt(&rch.borrow().max) {lch.borrow().max} else {rch.borrow().max},
                left: l,
                right: r,
                lch: Some(lch),
                rch: Some(rch),
            }
        }
    }
    pub fn get_less_num(&self, l: usize, r: usize, t: T) -> usize {
        if l == self.left && r == self.right {
            if self.max.lt(&t) {
                return r-l+1;
            }
        }
        if self.left == self.right {
            if self.max.lt(&t) {
                return 1;
            } else {
                return 0;
            }
        }
        let mid = (self.left+self.right)/2;
        if r<=mid {
            self.lch.as_ref().unwrap().borrow().get_less_num(l, r, t)
        } else if l> mid {
            self.rch.as_ref().unwrap().borrow().get_less_num(l,r,t)
        } else {
            self.lch.as_ref().unwrap().borrow().get_less_num(l,mid,t) + self.rch.as_ref().unwrap().borrow().get_less_num(mid+1, r, t)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;
    #[test]
    fn test_count_smaller() {
        assert_eq!(
            Solution::count_smaller_bst(vec![-1]),
            vec![0]
        );
        assert_eq!(
            Solution::count_smaller_bst(vec![5,2,6,1]),
            vec![2,1,1,0]
        );
        assert_eq!(
            Solution::count_smaller_bst(vec![2,1]),
            vec![1,0]
        );
        assert_eq!(
            Solution::count_smaller_bst(vec![2,1,1,1,1,1]),
            vec![5,0,0,0,0,0]
        );
        assert_eq!(
            Solution::count_smaller_bst(vec![3,1,2,1,1,1]),
            vec![5,0,3,0,0,0]
        );
        assert_eq!(
            Solution::count_smaller_bst(vec![6,5,4,3,2,1]),
            vec![5,4,3,2,1,0]
        );
    }
}