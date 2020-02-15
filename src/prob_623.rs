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
    pub fn add_one_row(mut root: Option<Rc<RefCell<TreeNode>>>, v: i32, d: i32) -> Option<Rc<RefCell<TreeNode>>> {
        if d == 1 {
            return Some(Rc::new(RefCell::new(TreeNode{
                val: v,
                left: root,
                right: None,
            })));
        }
        Self::dfs(&mut root, 1, d-1, v);
        root
    }
    fn dfs(root: &mut Option<Rc<RefCell<TreeNode>>>, dep: i32, d: i32, v: i32) {
        if root.is_none() {
            return;
        }
        if dep == d {
            let lch = root.as_mut().unwrap().borrow_mut().left.take();
            let rch = root.as_mut().unwrap().borrow_mut().right.take();
            let new_l = Some(Rc::new(RefCell::new(TreeNode{
                val: v,
                left: lch,
                right: None,
            })));
            let new_r = Some(Rc::new(RefCell::new(TreeNode{
                val: v,
                left: None,
                right: rch,
            })));
            root.as_mut().unwrap().borrow_mut().left = new_l;
            root.as_mut().unwrap().borrow_mut().right = new_r;
            return;
        }
        Self::dfs(&mut root.as_mut().unwrap().borrow_mut().left, dep+1, d, v);
        Self::dfs(&mut root.as_mut().unwrap().borrow_mut().right, dep+1, d, v);
    }
}

struct Solution;