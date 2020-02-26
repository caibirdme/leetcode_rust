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
    pub fn recover_tree(root: &mut Option<Rc<RefCell<TreeNode>>>) {
        if root.is_none() {
            return;
        }
        let mut pre = None;
        let mut x = None;
        let mut y = None;
        let mut count = 0;
        Self::inorder(&root, &mut count, &mut pre, &mut x, &mut y);
        count = 0;
        Self::dfs(root, &mut count, x.unwrap(), y.unwrap());
    }
    fn dfs(r: &mut Option<Rc<RefCell<TreeNode>>>, count: &mut i32, x: (i32, i32), y: (i32, i32)) {
        if r.is_none() {
            return;
        }
        if *count > y.0 {
            return;
        }
        Self::dfs(&mut r.as_mut().unwrap().borrow_mut().left, count, x, y);
        *count += 1;
        if *count == x.0 {
            r.as_mut().unwrap().borrow_mut().val = y.1;
        } else if *count == y.0 {
            r.as_mut().unwrap().borrow_mut().val = x.1;
        }
        Self::dfs(&mut r.as_mut().unwrap().borrow_mut().right, count, x, y);
    }
    fn inorder(r: &Option<Rc<RefCell<TreeNode>>>, count: &mut i32, pre: &mut Option<i32>, x: &mut Option<(i32, i32)>, y: &mut Option<(i32, i32)>) {
        if r.is_none() {
            return;
        }
        let br = r.as_ref().unwrap().borrow();
        Self::inorder(&br.left, count,pre, x, y);
        *count += 1;
        if let Some(ref v) = pre {
            if br.val < *v {
                *y = Some((*count, br.val));
                if x.is_none() {
                    *x = Some((*count-1, *v));
                } else {
                    return;
                }
            }
        }
        *pre = Some(br.val);
        Self::inorder(&br.right, count,pre, x, y);
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_recover_tree() {
        let mut tree = Some(Rc::new(RefCell::new(TreeNode{
            val: 2,
            left: Some(Rc::new(RefCell::new(TreeNode{
                val: 3,
                left: None,
                right: None,
            }))),
            right: Some(Rc::new(RefCell::new(TreeNode{
                val: 1,
                left: None,
                right: None,
            }))),
        })));
        Solution::recover_tree(&mut tree);
        assert_eq!(tree, Some(Rc::new(RefCell::new(TreeNode{
            val: 2,
            left: Some(Rc::new(RefCell::new(TreeNode{
                val: 1,
                left: None,
                right: None,
            }))),
            right: Some(Rc::new(RefCell::new(TreeNode{
                val: 3,
                left: None,
                right: None,
            }))),
        }))));
    }
}