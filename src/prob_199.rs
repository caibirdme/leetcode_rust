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
    pub fn right_side_view(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        if root.is_none() {
            return vec![];
        }
        let mut ans = vec![];
        Self::pre_order(0, &root, &mut ans);
        ans
    }
    fn pre_order(dep: usize, r: &Option<Rc<RefCell<TreeNode>>>, ans: &mut Vec<i32>) {
        if r.is_none() {
            return;
        }
        let br = r.as_ref().unwrap().borrow();
        if dep >= ans.len() {
            ans.push(br.val);
        } else {
            ans[dep] = br.val;
        }
        Self::pre_order(dep+1, &br.left, ans);
        Self::pre_order(dep+1, &br.right, ans);
    }
}

struct Solution;