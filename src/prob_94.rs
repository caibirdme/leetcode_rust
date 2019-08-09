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
    pub fn inorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        if root.is_none() {
            return vec![];
        }
        let mut ans = vec![];
        let mut stack = vec![(root.unwrap(), 0)];
        while !stack.is_empty() {
            let (cur,t) = stack.pop().unwrap();
            if t == 0 && cur.borrow().left.is_some() {
                let lch = Rc::clone(cur.borrow().left.as_ref().unwrap());
                stack.push((cur, 1));
                stack.push((lch, 0));
            } else {
                ans.push(cur.borrow().val);
                if cur.borrow().right.is_some() {
                    let rch = Rc::clone(cur.borrow().right.as_ref().unwrap());
                    stack.push((rch, 0));
                }
            }
        }
        ans
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use super::*;
    use std::rc::Rc;
    use std::cell::RefCell;


    #[test]
    fn test_inorder_traversal() {
        let test_cases = vec![
            (Some(Rc::new(RefCell::new(TreeNode{
                val: 1,
                left: None,
                right: Some(Rc::new(RefCell::new(TreeNode{
                    val: 2,
                    right: None,
                    left: Some(Rc::new(RefCell::new(TreeNode{
                        val: 3,
                        left: None,
                        right: None,
                    }))),
                }))),
            }))), vec![1, 3, 2]),
            (Some(Rc::new(RefCell::new(TreeNode{
                val: 10,
                left: None,
                right: None,
            }))), vec![10]),
            (Some(Rc::new(RefCell::new(TreeNode{
                val: 10,
                left: Some(Rc::new(RefCell::new(TreeNode{
                    val: 20,
                    left: None,
                    right: None,
                }))),
                right: None,
            }))), vec![20, 10]),
            (Some(Rc::new(RefCell::new(TreeNode{
                val: 10,
                left: Some(Rc::new(RefCell::new(TreeNode{
                    val: 20,
                    left: None,
                    right: None,
                }))),
                right: Some(Rc::new(RefCell::new(TreeNode{
                    val: 30,
                    left: None,
                    right: None,
                }))),
            }))), vec![20, 10, 30]),
        ];
        for (root, expect) in test_cases {
            assert_eq!(Solution::inorder_traversal(root), expect);
        }
    }
}