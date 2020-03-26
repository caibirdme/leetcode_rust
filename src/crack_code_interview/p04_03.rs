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
use std::rc::Rc;
use std::cell::RefCell;
impl Solution {
    pub fn list_of_depth(tree: Option<Rc<RefCell<TreeNode>>>) -> Vec<Option<Box<ListNode>>> {
        let mut ans = vec![];
        Self::dfs(&tree, 0, &mut ans);
        ans
    }
    fn dfs(r: &Option<Rc<RefCell<TreeNode>>>, d: usize, ans: &mut Vec<Option<Box<ListNode>>>) {
        if r.is_none() {
            return;
        }
        let br = r.as_ref().unwrap().borrow();
        if d == ans.len() {
            ans.push(Some(Box::new(ListNode{val: br.val, next: None,})));
        } else {
            let head = ans[d].take();
            ans[d] = Some(Box::new(ListNode{val: br.val, next: head}));
        }
        Self::dfs(&br.right, d+1, ans);
        Self::dfs(&br.left, d+1, ans);
    }
}

struct Solution;