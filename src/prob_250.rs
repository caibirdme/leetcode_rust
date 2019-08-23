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
    pub fn count_unival_subtrees(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        if root.is_none() {
            return 0;
        }
        let mut ans = 0;
        Self::dfs(&root, &mut ans);
        ans
    }
    fn dfs(root: &Option<Rc<RefCell<TreeNode>>>, ans: &mut i32) -> bool {
        match root {
            None => return true,
            Some(r) => {
                let n = r.borrow();
                let lch = &n.left;
                let rch = &n.right;
                let l_ok = Self::dfs(lch, ans);
                let r_ok = Self::dfs(rch, ans);
                if !l_ok || !r_ok {
                    return false;
                }
                if lch.is_none() && rch.is_none() {
                    *ans += 1;
                    return true;
                }
                match (lch, rch) {
                    (Some(_), Some(_)) => {
                        let lval = lch.as_ref().unwrap().borrow().val;
                        let rval = rch.as_ref().unwrap().borrow().val;
                        if lval != rval || lval != n.val {
                            return false;
                        }
                    },
                    (None, Some(_)) => {
                        let rval = rch.as_ref().unwrap().borrow().val;
                        if rval != n.val {
                            return false;
                        }
                    },
                    (Some(_), None) => {
                        let lval = lch.as_ref().unwrap().borrow().val;
                        if lval != n.val {
                            return false;
                        }
                    },
                    _ => unreachable!(),
                }
                *ans += 1;
                return true;
            }
        }
    }
}

struct Solution;