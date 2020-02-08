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
use std::cmp::max;

impl Solution {
    pub fn diameter_of_binary_tree(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut ans = 0;
        Self::dsf(&root, &mut ans);
        ans
    }
    fn dsf(r: &Option<Rc<RefCell<TreeNode>>>, ans: &mut i32) -> i32 {
        if r.is_none() {
            return 0;
        }
        let lch = Self::dsf(&r.as_ref().unwrap().borrow().left, ans);
        let rch = Self::dsf(&r.as_ref().unwrap().borrow().right, ans);
        let max_len = max(lch,rch)+1;
        *ans = max(*ans, lch+rch);
        max_len
    }
}

struct Solution;