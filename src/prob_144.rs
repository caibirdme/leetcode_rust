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
    pub fn preorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        if root.is_none() {
            return vec![];
        }
        let mut ans = vec![];
        let mut stack = vec![(root.unwrap(), 0)];
        while !stack.is_empty() {
            let (cur,t) = stack.pop().unwrap();
            if t == 0 {
                ans.push(cur.borrow().val);
                stack.push((Rc::clone(&cur), 1));
                if cur.borrow().left.is_some() {
                    stack.push((Rc::clone(cur.borrow().left.as_ref().unwrap()), 0));
                }
            } else {
                if cur.borrow().right.is_some() {
                    stack.push((Rc::clone(cur.borrow().right.as_ref().unwrap()), 0));
                }
            }
        }
        ans
    }
}

struct Solution;