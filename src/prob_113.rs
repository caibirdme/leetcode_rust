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
    pub fn path_sum(root: Option<Rc<RefCell<TreeNode>>>, sum: i32) -> Vec<Vec<i32>> {
        if root.is_none() {
            return vec![];
        }
        let mut ans = vec![];
        let mut cur = vec![];
        Self::dfs(root.unwrap(), sum, &mut cur, &mut ans);
        ans
    }
    fn dfs(root: Rc<RefCell<TreeNode>>, sum: i32, cur: &mut Vec<i32>, ans: &mut Vec<Vec<i32>>) {
        if root.borrow().left.is_none() && root.borrow().right.is_none() {
            if sum == root.borrow().val {
                let mut v = cur.clone();
                v.push(sum);
                ans.push(v);
            }
            return;
        }
        let next_val = sum - root.borrow().val;
        cur.push(root.borrow().val);
        let lch = root.borrow_mut().left.take();
        if lch.is_some() {
            Self::dfs(lch.unwrap(), next_val, cur, ans);
        }
        let rch = root.borrow_mut().right.take();
        if rch.is_some() {
            Self::dfs(rch.unwrap(), next_val, cur, ans);
        }
        cur.pop();
    }
}

struct Solution;