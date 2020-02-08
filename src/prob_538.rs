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
    pub fn convert_bst(mut root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        let mut sum = 0;
        Self::dfs(&mut root, &mut sum);
        root
    }
    fn dfs(r: &mut Option<Rc<RefCell<TreeNode>>>, sum: &mut i32) {
        if r.is_none() {
            return;
        }
        {
            let rch = &mut r.as_mut().unwrap().borrow_mut().right;
            if rch.is_some() {
                Self::dfs(rch, sum);
            }
        }
        *sum += r.as_ref().unwrap().borrow().val;
        r.as_mut().unwrap().borrow_mut().val = *sum;
        let lch = &mut r.as_mut().unwrap().borrow_mut().left;
        if lch.is_some() {
            Self::dfs(lch, sum);
        }
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_convert_bst() {
        let root = Some(Rc::new(RefCell::new(TreeNode{
            val: 5,
            left: Some(Rc::new(RefCell::new(TreeNode{
                val: 2,
                left: None,
                right: None,
            }))),
            right: Some(Rc::new(RefCell::new(TreeNode{
                val: 13,
                left: None,
                right: None,
            }))),
        })));
        let expect = Some(Rc::new(RefCell::new(TreeNode{
            val: 18,
            left: Some(Rc::new(RefCell::new(TreeNode{
                val: 20,
                left: None,
                right: None,
            }))),
            right: Some(Rc::new(RefCell::new(TreeNode{
                val: 13,
                left: None,
                right: None,
            }))),
        })));
        assert_eq!(Solution::convert_bst(root), expect);
    }
}