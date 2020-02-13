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
    pub fn build_tree(preorder: Vec<i32>, inorder: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        Self::construct(&preorder, &inorder)
    }
    fn construct(pre: &[i32], ino: &[i32]) -> Option<Rc<RefCell<TreeNode>>> {
        if pre.is_empty() {
            return None;
        }
        let cur = pre[0];
        let p = ino.iter().position(|v| *v == cur).unwrap();
        let lch = Self::construct(&pre[1..1+p], &ino[..p]);
        let rch = Self::construct(&pre[1+p..], &ino[1+p..]);
        Some(Rc::new(RefCell::new(TreeNode{
            val: cur,
            left: lch,
            right: rch,
        })))
    }
}

struct Solution;