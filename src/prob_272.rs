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
    pub fn closest_k_values(root: Option<Rc<RefCell<TreeNode>>>, target: f64, k: i32) -> Vec<i32> {
        if root.is_none() {
            return vec![];
        }
        let mut ans = vec![];
        let mut q = vec![(Rc::clone(root.as_ref().unwrap()), 0)];
        let mut l = 0;
        let mut uk = k as usize;
        while let Some((node,status)) = q.pop() {
            if status == 1 {
                let val = node.borrow().val;
                ans.push(val);
                if ans.len() > uk {
                    if target - ans[l] as f64 > val as f64-target {
                        l += 1;
                    } else {
                        ans.pop();
                        return Vec::from(&ans[l..]);
                    }
                }
                if node.borrow().right.is_some() {
                    q.push((Rc::clone(node.borrow().right.as_ref().unwrap()), 0));
                }
            } else {
                q.push((Rc::clone(&node), 1));
                if node.borrow().left.is_some() {
                    q.push((Rc::clone(node.borrow().left.as_ref().unwrap()), 0));
                }
            }
        }
        Vec::from(&ans[l..])
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_closest_k_values() {
        let r = Some(Rc::new(RefCell::new(TreeNode{
            val: 4,
            left: Some(Rc::new(RefCell::new(TreeNode{
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
            }))),
            right: Some(Rc::new(RefCell::new(TreeNode{
                val: 5,
                left:None,
                right: None,
            }))),
        })));
        assert_eq!(Solution::closest_k_values(r.clone(), 3.714286, 3), vec![3,4,5]);
        assert_eq!(Solution::closest_k_values(r.clone(), 3.714286, 2), vec![3,4]);
    }
}