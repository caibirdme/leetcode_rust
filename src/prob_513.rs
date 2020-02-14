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
    pub fn find_bottom_left_value(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut ans = 0;
        let mut depth = 0;
        let mut q = vec![(root.as_ref().unwrap().clone(), 1)];
        let mut head = 0;
        while head < q.len() {
            let (cur,d) = q[head].clone();
            head += 1;
            if d > depth {
                depth = d;
                ans = cur.borrow().val;
            }
            let br = cur.borrow();
            if br.left.is_some() {
                q.push((br.left.as_ref().unwrap().clone(), d+1));
            }
            if br.right.is_some() {
                q.push((br.right.as_ref().unwrap().clone(), d+1));
            }
        }
        ans
    }
}

struct Solution;