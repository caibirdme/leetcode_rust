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
    pub fn find_leaves(mut root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
        let mut ans = vec![];
        if root.is_none() {
            return ans;
        }
        while !Self::is_leaf(&root) {
            ans.push(Self::remove_leaf_node(&mut root));
        }
        ans.push(vec![root.take().unwrap().borrow().val]);
        ans
    }
    fn remove_leaf_node(root: &mut Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        if root.is_none() {
            return vec![];
        }
        let mut ans = vec![];
        let mut cur = root.as_mut().unwrap().borrow_mut();
        if cur.left.is_some() {
            if Self::is_leaf(&cur.left) {
                let lch = cur.left.take();
                ans.push(lch.unwrap().borrow().val);
            } else {
                let mut temp = Self::remove_leaf_node(&mut cur.left);
                ans.append(&mut temp);
            }
        }
        if cur.right.is_some() {
            if Self::is_leaf(&cur.right) {
                let lch = cur.right.take();
                ans.push(lch.unwrap().borrow().val);
            } else {
                let mut temp = Self::remove_leaf_node(&mut cur.right);
                ans.append(&mut temp);
            }
        }
        ans
    }
    fn is_leaf(node: &Option<Rc<RefCell<TreeNode>>>) -> bool {
        if node.is_none() {
            return false;
        }
        node.as_ref().unwrap().borrow().left.is_none() && node.as_ref().unwrap().borrow().right.is_none()
    }
}

struct Solution;