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
    pub fn balance_bst(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        let arr = Self::traverse(root);
        Self::build_tree(&arr)
    }
    fn build_tree(data: &[i32]) -> Option<Rc<RefCell<TreeNode>>> {
        if data.is_empty() {
            return None;
        }
        let n = data.len();
        if n == 1 {
            return Some(Rc::new(RefCell::new(TreeNode{
                val: data[0],
                left: None,
                right: None,
            })));
        }
        let r = n / 2;
        Some(Rc::new(RefCell::new(TreeNode{
            val: data[r],
            left: Self::build_tree(&data[..r]),
            right: Self::build_tree(&data[r+1..]),
        })))
    }
    fn traverse(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        if root.is_none() {
            return vec![];
        }
        let mut ans = vec![];
        let mut q = vec![(Rc::clone(root.as_ref().unwrap()), 0)];
        while let Some((node, status)) = q.pop() {
            if status == 0 {
                q.push((Rc::clone(&node), 1));
                if node.borrow().left.is_some() {
                    q.push((Rc::clone(node.borrow().left.as_ref().unwrap()), 0));
                }
            } else {
                ans.push(node.borrow().val);
                if node.borrow().right.is_some() {
                    q.push((Rc::clone(node.borrow().right.as_ref().unwrap()), 0));
                }
            }
        }
        ans
    }
}

struct Solution;