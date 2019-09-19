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
    pub fn longest_consecutive(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut ans = 0;
        Self::dfs(&root, &mut ans);
        ans
    }
    fn dfs(r: &Option<Rc<RefCell<TreeNode>>>, ans: &mut i32) -> i32 {
        use std::cmp::max;
        if r.is_none() {
            return 0;
        }
        let b = r.as_ref().unwrap().borrow();
        let lch = &b.left;
        let rch = &b.right;
        let l_len = Self::dfs(lch, ans);
        let r_len = Self::dfs(rch, ans);
        let mut cur_len = 1;
        let next = b.val+1;
        if l_len > 0 && lch.as_ref().unwrap().borrow().val == next {
            cur_len = l_len+1;
        }
        if r_len > 0 && rch.as_ref().unwrap().borrow().val == next {
            cur_len = max(cur_len, r_len+1);
        }
        *ans = max(*ans, cur_len);
        cur_len
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_longest_consecutive() {
        let test_cases = vec![
            (Some(Rc::new(RefCell::new(TreeNode{
                val:1,
                left: None,
                right: Some(Rc::new(RefCell::new(TreeNode{
                    val: 3,
                    left: Some(Rc::new(RefCell::new(TreeNode::new(2)))),
                    right: Some(Rc::new(RefCell::new(TreeNode{
                        val: 4,
                        left: None,
                        right: Some(Rc::new(RefCell::new(TreeNode::new(5)))),
                    }))),
                }))),
            }))), 3),
        ];
        for (r, expect) in test_cases {
            assert_eq!(Solution::longest_consecutive(r.clone()), expect, "root: {:?}", r);
        }
    }
}