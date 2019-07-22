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
    pub fn is_symmetric(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        if let Some(ref root_node) = root {
            let br = root_node.borrow();
            let lch = br.left.as_ref();
            let rch = br.right.as_ref();
            Self::dfs(lch, rch)
        } else {
            return true;
        }
    }
    fn is_leaf(node: &Rc<RefCell<TreeNode>>) -> bool {
        let br = node.borrow();
        return br.left.is_none() && br.right.is_none()
    }
    fn dfs(lch: Option<&Rc<RefCell<TreeNode>>>, rch: Option<&Rc<RefCell<TreeNode>>>) -> bool {
        if lch.is_none() && rch.is_none() {
            return true;
        }
        if lch.is_none() || rch.is_none() {
            return false;
        }
        let lch = lch.unwrap();
        let rch = rch.unwrap();
        let a = Self::is_leaf(lch);
        let b = Self::is_leaf(rch);
        if a && b {
            return lch.as_ref().borrow().val == rch.as_ref().borrow().val;
        } else if a || b {
            return false;
        }
        let br_lch = lch.borrow();
        let br_rch = rch.borrow();
        if br_lch.val != br_rch.val {
            return false;
        }
        let lch_of_lch = br_lch.left.as_ref();
        let rch_of_lch = br_lch.right.as_ref();
        let lch_of_rch = br_rch.left.as_ref();
        let rch_of_rch = br_rch.right.as_ref();

        Self::dfs(lch_of_lch, rch_of_rch) && Self::dfs(rch_of_lch, lch_of_rch)
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;


    #[test]
    fn test_is_symmetric() {
        let test_cases = vec![
            (Some(Rc::new(RefCell::new(TreeNode{
                val: 1,
                left: Some(Rc::new(RefCell::new(TreeNode{
                    val: 2,
                    left: None,
                    right: Some(Rc::new(RefCell::new(TreeNode{
                        val: 3,
                        left: Some(Rc::new(RefCell::new(TreeNode{
                            val: -7,
                            left: None,
                            right: None,
                        }))),
                        right: None,
                    }))),
                }))),
                right: Some(Rc::new(RefCell::new(TreeNode{
                    val: 2,
                    left: Some(Rc::new(RefCell::new(TreeNode{
                        val: 4,
                        left: None,
                        right: Some(Rc::new(RefCell::new(TreeNode{
                            val: -7,
                            left: None,
                            right: None,
                        }))),
                    }))),
                    right: None,
                }))),
            }))), false),
            (None, true),
            (Some(Rc::new(RefCell::new(TreeNode{val:1,left:None, right:None}))), true),
            (Some(Rc::new(RefCell::new(TreeNode{
                val: 1,
                left: Some(Rc::new(RefCell::new(TreeNode{
                    val: 2,
                    left: None,
                    right: None,
                }))),
                right: Some(Rc::new(RefCell::new(TreeNode{
                    val: 2,
                    left: None,
                    right: None,
                }))),
            }))), true),
            (Some(Rc::new(RefCell::new(TreeNode{
                val: 1,
                left: Some(Rc::new(RefCell::new(TreeNode{
                    val: 2,
                    left: None,
                    right: None,
                }))),
                right: Some(Rc::new(RefCell::new(TreeNode{
                    val: 3,
                    left: None,
                    right: None,
                }))),
            }))), false),
            (Some(Rc::new(RefCell::new(TreeNode{
                val: 1,
                left: Some(Rc::new(RefCell::new(TreeNode{
                    val: 2,
                    left: Some(Rc::new(RefCell::new(TreeNode{
                        val: 3,
                        left: None,
                        right: None,
                    }))),
                    right: None,
                }))),
                right: Some(Rc::new(RefCell::new(TreeNode{
                    val: 2,
                    left: None,
                    right: Some(Rc::new(RefCell::new(TreeNode{
                        val: 3,
                        left: None,
                        right: None,
                    }))),
                }))),
            }))), true),
            (Some(Rc::new(RefCell::new(TreeNode{
                val: 1,
                left: Some(Rc::new(RefCell::new(TreeNode{
                    val: 2,
                    left: Some(Rc::new(RefCell::new(TreeNode{
                        val: 3,
                        left: None,
                        right: None,
                    }))),
                    right: Some(Rc::new(RefCell::new(TreeNode{
                        val: 4,
                        left: None,
                        right: None,
                    }))),
                }))),
                right: Some(Rc::new(RefCell::new(TreeNode{
                    val: 2,
                    left: Some(Rc::new(RefCell::new(TreeNode{
                        val: 4,
                        left: None,
                        right: None,
                    }))),
                    right: Some(Rc::new(RefCell::new(TreeNode{
                        val: 3,
                        left: None,
                        right: None,
                    }))),
                }))),
            }))), true),
            (Some(Rc::new(RefCell::new(TreeNode{
                val: 1,
                left: Some(Rc::new(RefCell::new(TreeNode{
                    val: 2,
                    left: Some(Rc::new(RefCell::new(TreeNode{
                        val: 3,
                        left: None,
                        right: None,
                    }))),
                    right: Some(Rc::new(RefCell::new(TreeNode{
                        val: 4,
                        left: None,
                        right: None,
                    }))),
                }))),
                right: Some(Rc::new(RefCell::new(TreeNode{
                    val: 2,
                    left: Some(Rc::new(RefCell::new(TreeNode{
                        val: 4,
                        left: None,
                        right: None,
                    }))),
                    right: Some(Rc::new(RefCell::new(TreeNode{
                        val: 4,
                        left: None,
                        right: None,
                    }))),
                }))),
            }))), false),
        ];
        for (idx, (root, ok)) in test_cases.into_iter().enumerate() {
            assert_eq!(Solution::is_symmetric(root), ok, "case {} failed", idx);
        }
    }
}