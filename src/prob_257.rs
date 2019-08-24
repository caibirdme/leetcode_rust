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
    pub fn binary_tree_paths(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<String> {
        let mut ans = vec![];
        let mut cur = vec![];
        Self::dfs(&root, &mut cur, &mut ans);
        ans
    }
    fn dfs(root: &Option<Rc<RefCell<TreeNode>>>, cur: &mut Vec<i32>, ans: &mut Vec<String>) {
        if root.is_none() {
            return;
        }
        if Self::is_leaf(root) {
            cur.push(root.as_ref().unwrap().borrow().val);
            let s: Vec<String> = cur.iter().map(|&c| c.to_string()).collect();
            ans.push(s.join("->"));
            cur.pop();
            return;
        }
        let r = root.as_ref().unwrap().borrow();
        cur.push(r.val);
        Self::dfs(&r.left, cur, ans);
        Self::dfs(&r.right, cur, ans);
        cur.pop();
    }
    fn is_leaf(r: &Option<Rc<RefCell<TreeNode>>>) -> bool {
        if r.is_none() {
            return false;
        }
        let q = r.as_ref().unwrap().borrow();
        q.left.is_none() && q.right.is_none()
    }
}

struct Solution;