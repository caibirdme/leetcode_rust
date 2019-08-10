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
 use std::cmp::{max, min};

 impl Solution {
    pub fn is_balanced(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        if root.is_none() {
            return false;
        }
        Self::dfs(root) != -1
    }
    pub fn dfs(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        if root.is_none() {
            return 0;
        }
        let r = root.unwrap();
        let lh = Self::dfs(r.borrow_mut().left.take());
        if lh == -1 {
            return -1;
        }
        let rh = Self::dfs(r.borrow_mut().right.take());
        if rh == -1 {
            return -1;
        }
        let max_h = max(lh,rh);
        let min_h = min(lh,rh);
        if max_h - min_h > 1 {
            return -1;
        }
        max_h+1
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_balanced() {
        let test_cases = vec![
            (Some(Rc::new(RefCell::new(TreeNode{
                val: 1,
                left: Some(Rc::new(RefCell::new(TreeNode{
                    val: 2,
                    left: Some(Rc::new(RefCell::new(TreeNode{
                        val: 3,
                        left: Some(Rc::new(RefCell::new(TreeNode{
                            val: 4,
                            left: None,
                            right: None,
                        }))),
                        right: Some(Rc::new(RefCell::new(TreeNode{
                            val: 4,
                            left: None,
                            right: None,
                        })))
                    }))),
                    right: Some(Rc::new(RefCell::new(TreeNode{
                        val: 3,
                        left: None,
                        right: None,
                    }))),
                }))),
                right: Some(Rc::new(RefCell::new(TreeNode{
                    val: 2,
                    left: None,
                    right: None,
                }))),
            }))), false, 1),
            (None, false, 0),
        ];
        for (root, ok, idx) in test_cases {
            assert_eq!(Solution::is_balanced(root), ok, "case {}", idx);
        }
    }
}