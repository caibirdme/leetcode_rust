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
    pub fn pseudo_palindromic_paths (root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        if root.is_none() {
            return 0;
        }
        let mut count = [0; 10];
        let mut ans = 0;
        Self::dfs(&root, &mut count, &mut ans);
        ans
    }
    fn dfs(r: &Option<Rc<RefCell<TreeNode>>>, count: &mut [i32; 10], ans: &mut i32) {
        if r.is_none() {
            return;
        }
        let br = r.as_ref().unwrap().borrow();
        if br.left.is_none() && br.right.is_none() {
            count[br.val as usize] += 1;
            let mut odd = 0;
            for i in 0..=9 {
                if count[i] & 1 == 1 {
                    odd += 1;
                }
            }
            if odd <= 1 {
                *ans += 1;
            }
            count[br.val as usize] -= 1;
            return
        }
        count[br.val as usize] += 1;
        Self::dfs(&br.left, count, ans);
        Self::dfs(&br.right, count, ans);
        count[br.val as usize] -= 1;
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pseudo_palindromic_paths() {
        let root = Some(Rc::new(RefCell::new(TreeNode{
            val: 1,
            left: Some(Rc::new(RefCell::new(TreeNode{
                val: 9,
                left: None,
                right: Some(Rc::new(RefCell::new(TreeNode{
                    val: 1,
                    left: None,
                    right: None,
                }))),
            }))),
            right: Some(Rc::new(RefCell::new(TreeNode{
                val: 1,
                left: None,
                right: Some(Rc::new(RefCell::new(TreeNode{
                    val: 1,
                    left: Some(Rc::new(RefCell::new(TreeNode{
                        val: 7,
                        left: None,
                        right: Some(Rc::new(RefCell::new(TreeNode{
                            val: 4,
                            left: None,
                            right: None,
                        }))),
                    }))),
                    right: None,
                }))),
            }))),
        })));
        assert_eq!(Solution::pseudo_palindromic_paths(root), 1);
    }
}