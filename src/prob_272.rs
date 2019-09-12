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

impl Solution {
    pub fn closest_k_values(root: Option<Rc<RefCell<TreeNode>>>, target: f64, k: i32) -> Vec<i32> {
        let mut less = vec![];
        let mut larger = vec![];
        Self::larger_than_reverse(&root, target, &mut larger);
        Self::less_equal(&root, target, &mut less);
        let mut ans = vec![];
        while ans.len() < k as usize {
            if !less.is_empty() && !larger.is_empty() {
                let (&l,&r) = (less.last().unwrap(), larger.last().unwrap());
                if (l as f64-target).abs() < (r as f64 - target).abs() {
                    ans.push(l);
                    less.pop();
                } else {
                    ans.push(r);
                    larger.pop();
                }
            }else if !less.is_empty() {
                ans.push(less.pop().unwrap());
            } else {
                ans.push(larger.pop().unwrap());
            }
        }
        ans
    }
    fn larger_than_reverse(root: &Option<Rc<RefCell<TreeNode>>>, target: f64, q: &mut Vec<i32>) {
        if root.is_none() {
            return;
        }
        let cur = root.as_ref().unwrap().borrow();
        Self::larger_than_reverse(&cur.right, target, q);
        if cur.val as f64 <= target {
            return;
        }
        q.push(cur.val);
        Self::larger_than_reverse(&cur.left, target, q);
    }
    fn less_equal(root: &Option<Rc<RefCell<TreeNode>>>, target: f64, q: &mut Vec<i32>) {
        if root.is_none() {
            return;
        }
        let cur = root.as_ref().unwrap().borrow();
        Self::less_equal(&cur.left, target, q);
        if cur.val as f64 > target {
            return;
        }
        q.push(cur.val);
        Self::less_equal(&cur.right, target, q);
    }
}

struct Solution;