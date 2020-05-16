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
    pub fn good_nodes(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        if root.is_none() {
            return 0;
        }
        let mut ans = 0;
        let mut m = root.as_ref().unwrap().borrow().val;
        Self::count(&root, m, &mut ans);
        ans
    }
    fn count(r: &Option<Rc<RefCell<TreeNode>>>, m: i32, ans: &mut i32) {
        if r.is_none() {
            return;
        }
        let br = r.as_ref().unwrap().borrow();
        let val = br.val;
        if m <= val {
            *ans += 1;
        }
        let next = m.max(val);
        Self::count(&br.left, next, ans);
        Self::count(&br.right, next, ans);
    }
}

struct Solution;