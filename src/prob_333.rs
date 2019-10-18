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
use std::cmp::max;

impl Solution {
    pub fn largest_bst_subtree(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        if root.is_none() {
            return 0;
        }
        let mut ans = 0;
        Self::dfs(&root, &mut ans);
        ans
    }
    fn dfs(root: &Option<Rc<RefCell<TreeNode>>>, ans: &mut i32) -> (Option<i32>, i32, i32) {
        if root.is_none() {
            return (Some(0), 0, 0);
        }
        let br = root.as_ref().unwrap().borrow();
        let (lch_w, l_min_node, l_max_node) = Self::dfs(&br.left, ans);
        let (rch_w, r_min_node, r_max_node) = Self::dfs(&br.right, ans);
        match (lch_w, rch_w) {
            (Some(lw), Some(rw)) => {
                // leaf node
                if lw == 0 && rw == 0 {
                    *ans = max(*ans, 1);
                    return (Some(1), br.val, br.val);
                } else if lw == 0 {
                    if r_min_node <= br.val {
                        return (None, 0, 0);
                    }
                    *ans = max(*ans, rw+1);
                    return (Some(rw+1), br.val, r_max_node);
                } else if rw == 0 {
                    if l_max_node >= br.val {
                        return (None, 0, 0);
                    }
                    *ans = max(*ans, lw+1);
                    return (Some(lw+1), l_min_node, br.val);
                } else {
                    if l_max_node >= br.val || r_min_node <= br.val {
                        return (None, 0, 0);
                    }
                    let nw = lw+rw+1;
                    *ans = max(*ans, nw);
                    return (Some(nw), l_min_node, r_max_node);
                }
            },
            _ => {
                return (None, 0, 0);
            }
        }
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_largest_bst_subtree() {
        let test_cases = vec![
            (
                Some(Rc::new(RefCell::new(TreeNode{
                    val: 10,
                    left: Some(Rc::new(RefCell::new(TreeNode{
                        val: 5,
                        left: Some(Rc::new(RefCell::new(TreeNode{
                            val: 1,
                            left: None,
                            right: None,
                        }))),
                        right: Some(Rc::new(RefCell::new(TreeNode{
                            val: 8,
                            left: None,
                            right: None,
                        }))),
                    }))),
                    right: Some(Rc::new(RefCell::new(TreeNode{
                        val: 15,
                        left: None,
                        right: Some(Rc::new(RefCell::new(TreeNode{
                            val: 7,
                            left: None,
                            right: None,
                        }))),
                    }))),
                }))),
                3
            ),
            (
                Some(Rc::new(RefCell::new(TreeNode{
                    val: 10,
                    left: Some(Rc::new(RefCell::new(TreeNode{
                        val: 5,
                        left: Some(Rc::new(RefCell::new(TreeNode{
                            val: 1,
                            left: None,
                            right: None,
                        }))),
                        right: Some(Rc::new(RefCell::new(TreeNode{
                            val: 8,
                            left: None,
                            right: None,
                        }))),
                    }))),
                    right: Some(Rc::new(RefCell::new(TreeNode{
                        val: 15,
                        left: None,
                        right: Some(Rc::new(RefCell::new(TreeNode{
                            val: 20,
                            left: None,
                            right: None,
                        }))),
                    }))),
                }))),
                6
            ),
            (
                Some(Rc::new(RefCell::new(TreeNode{
                    val: 4,
                    left: Some(Rc::new(RefCell::new(TreeNode{
                        val: 2,
                        left: Some(Rc::new(RefCell::new(TreeNode{
                            val: 2,
                            left: Some(Rc::new(RefCell::new(TreeNode{
                                val: 2,
                                left: Some(Rc::new(RefCell::new(TreeNode{
                                    val: 1,
                                    left: None,
                                    right: None,
                                }))),
                                right: None,
                            }))),
                            right: None,
                        }))),
                        right: Some(Rc::new(RefCell::new(TreeNode{
                            val: 3,
                            left: None,
                            right: None,
                        }))),
                    }))),
                    right: Some(Rc::new(RefCell::new(TreeNode{
                        val: 7,
                        left: Some(Rc::new(RefCell::new(TreeNode{
                            val: 5,
                            left: None,
                            right: None,
                        }))),
                        right: None,
                    }))),
                }))),
                2
            ),
        ];
        for (root, expect) in test_cases {
            assert_eq!(Solution::largest_bst_subtree(root.clone()), expect, "root: {:?}", root);
        }
    }
}