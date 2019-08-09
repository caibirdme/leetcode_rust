#[derive(Debug, PartialEq, Eq)]
 pub struct TreeNode {
   pub val: i32,
   pub left: Option<Rc<RefCell<TreeNode>>>,
   pub right: Option<Rc<RefCell<TreeNode>>>,
 }

 impl TreeNode {
   #[inline]
   pub fn new(val: i32) -> Self {
     TreeNode {
       val,
       left: None,
       right: None
     }
   }
 }
use std::rc::Rc;
use std::cell::RefCell;
use std::collections::VecDeque;

impl Solution {
    pub fn level_order(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
        if root.is_none() {
            return vec![];
        }
        let mut ans = vec![];
        let mut pre = 0;
        let mut q = VecDeque::new();
        q.push_back((root.unwrap(), 0));
        let mut t:Vec<i32> = vec![];
        while !q.is_empty() {
            let (cur,p) = q.pop_front().unwrap();
            if p != pre {
                ans.push(t.clone());
                t.clear();
                pre = p;
            }
            t.push(cur.borrow().val);
            if cur.borrow().left.is_some() {
                q.push_back((Rc::clone(cur.borrow().left.as_ref().unwrap()), p+1));
            }
            if cur.borrow().right.is_some() {
                q.push_back((Rc::clone(cur.borrow().right.as_ref().unwrap()), p+1));
            }
        }
        if !t.is_empty() {
            ans.push(t);
        }
        ans
    }
}

struct Solution;