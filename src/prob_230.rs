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
    pub fn kth_smallest(root: Option<Rc<RefCell<TreeNode>>>, k: i32) -> i32 {
        let mut ans = -1;
        let mut count = 0;
        Self::dfs(root.as_ref(), k, &mut count, &mut ans);
        ans
    }
    fn dfs(root: Option<&Rc<RefCell<TreeNode>>>, k: i32, count: &mut i32, ans: &mut i32) {
        if root.is_none() {
            return;
        }
        let r = root.unwrap().borrow();
        Self::dfs(r.left.as_ref(), k, count, ans);
        *count += 1;
        if *count == k {
            *ans = r.val;
            return;
        }
        if *count < k {
            Self::dfs(r.right.as_ref(), k, count, ans);
        }
    }
}

struct Solution;