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
    pub fn count_nodes(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let d = Self::get_depth(&root);
        let ans = 2f64.powi(d) as i32-1;
        let mut l = 1;
        let right = 2f64.powi(d-1) as i32;
        let mut r = right;
        let mut maybe = 0;
        while l <= r {
            let mid = (l+r)/2;
            if Self::check(&root, mid, 1, right) {
                maybe = mid;
                l = mid+1;
            } else {
                r = mid-1;
            }
        }
        ans + maybe
    }
    fn check(root: &Option<Rc<RefCell<TreeNode>>>, t: i32, l: i32, r: i32) -> bool {
        if l == r {
            return root.is_some();
        }
        let mid = (l+r)/2;
        if t<=mid {
            Self::check(&root.as_ref().unwrap().borrow().left, t, l, mid)
        } else {
            Self::check(&root.as_ref().unwrap().borrow().right, t, mid+1, r)
        }
    }



    fn get_depth(r: &Option<Rc<RefCell<TreeNode>>>) -> i32 {
        if r.is_none() {
            return 0;
        }
        1 + Self::get_depth(&r.as_ref().unwrap().borrow().left)
    }
}









struct Solution;

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_foo() {
        assert_eq!(1+1, 2);
    }
}