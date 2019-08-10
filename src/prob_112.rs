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
    pub fn has_path_sum(root: Option<Rc<RefCell<TreeNode>>>, sum: i32) -> bool {
        if root.is_none() {
            return false;
        }
        Self::dfs(root.unwrap(), sum)
    }
    fn dfs(root: Rc<RefCell<TreeNode>>, sum: i32) -> bool {
        if root.borrow().left.is_none() && root.borrow().right.is_none() {
            return sum == root.borrow().val;
        }
        let next_val = sum - root.borrow().val;
        let lch = root.borrow_mut().left.take();
        if lch.is_some() && Self::dfs(lch.unwrap(), next_val) {
            return true;
        }
        let rch = root.borrow_mut().right.take();
        if rch.is_some() && Self::dfs(rch.unwrap(), next_val) {
            return true;
        }
        false
    }
}

struct Solution;