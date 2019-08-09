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
    pub fn build_tree(inorder: Vec<i32>, postorder: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        Self::dfs(&inorder[..], &postorder[..])
    }
    fn dfs(inorder: &[i32], postorder: &[i32]) -> Option<Rc<RefCell<TreeNode>>> {
        if inorder.is_empty() {
            return None;
        }
        if inorder.len() == 1 {
            return Some(Rc::new(RefCell::new(TreeNode{
                val: inorder[0],
                left: None,
                right: None,
            })));
        }
        let n = inorder.len();
        let root_val = postorder[n-1];
        let mut p = 0;
        for i in 1..n {
            if inorder[i] == root_val {
                p = i;
                break;
            }
        }
        let lch = Self::dfs(&inorder[..p], &postorder[..p]);
        let rch = Self::dfs(&inorder[p+1..], &postorder[p..n-1]);
        Some(Rc::new(RefCell::new(TreeNode{
            val: root_val,
            left: lch,
            right: rch,
        })))
    }
}

struct Solution;