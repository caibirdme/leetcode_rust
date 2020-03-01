// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
  pub val: i32,
  pub next: Option<Box<ListNode>>
}

impl ListNode {
  #[inline]
  fn new(val: i32) -> Self {
    ListNode {
      next: None,
      val
    }
  }
}
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
    pub fn is_sub_path(head: Option<Box<ListNode>>, root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        if head.is_none() || root.is_none() {
            return false;
        }
        let mut ans = false;
        Self::pre_order(&root, &head, &mut ans);
        ans
    }
    fn pre_order(r: &Option<Rc<RefCell<TreeNode>>>, head: &Option<Box<ListNode>>, ans: &mut bool) {
        if *ans || r.is_none() {
            return;
        }
        let br = r.as_ref().unwrap().borrow();
        if Self::check(r, head) {
            *ans = true;
            return;
        }
        Self::pre_order(&br.left, head, ans);
        Self::pre_order(&br.right, head, ans);
    }
    fn check(r: &Option<Rc<RefCell<TreeNode>>>, head: &Option<Box<ListNode>>) -> bool {
        if head.is_none() {
            return true;
        }
        if r.is_none() {
            return false;
        }
        let br = r.as_ref().unwrap().borrow();
        if br.val == head.as_ref().unwrap().val {
            Self::check(&br.left, &head.as_ref().unwrap().next) || Self::check(&br.right, &head.as_ref().unwrap().next)
        } else {
            false
        }
    }
}

struct Solution;