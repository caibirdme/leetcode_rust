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
use std::cmp::max;
impl Solution {
    pub fn rob(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        if root.is_none() {
            return 0;
        }
        let (a,b) = Self::dfs(root.as_ref());
        max(a,b)
    }
    fn dfs(root: Option<&Rc<RefCell<TreeNode>>>) -> (i32, i32) {
        match root {
            None => (0,0),
            Some(cur) => {
                let (a,b) = Self::dfs(cur.borrow().left.as_ref());
                let (c,d) = Self::dfs(cur.borrow().right.as_ref());
                (cur.borrow().val+b+d, max(a,b)+max(c,d))
            }
        }
    }
}

 struct Solution;