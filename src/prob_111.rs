// Definition for a binary tree node.
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
use std::cmp::min;

impl Solution {
    pub fn min_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        if root.is_none() {
            return 0;
        }
        if root.as_ref().unwrap().borrow().left.is_none() && root.as_ref().unwrap().borrow().right.is_none() {
            return 1;
        }
        let cur = root.unwrap();
        let l = {
            let lch = cur.borrow_mut().left.take();
            if lch.is_none() {
                None
            } else {
                Some(Self::min_depth(lch))
            }
        };
        let r = {
            let rch = cur.borrow_mut().right.take();
            if rch.is_none() {
                None
            } else {
                Some(Self::min_depth(rch))
            }
        };
        if l.is_none() {
            r.unwrap() + 1
        } else if r.is_none() {
            l.unwrap() + 1
        } else {
            min(l.unwrap(), r.unwrap()) + 1
        }
    }
}

struct Solution;