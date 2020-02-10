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
    pub fn merge_trees(mut t1: Option<Rc<RefCell<TreeNode>>>, t2: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        if t1.is_none() {
            return t2;
        } else if t2.is_none() {
            return t1;
        }
        Self::dfs(&mut t1, &t2);
        t1
    }
    fn dfs(t1: &mut Option<Rc<RefCell<TreeNode>>>, t2: &Option<Rc<RefCell<TreeNode>>>) {
        if t2.is_none() {
            return;
        }
        t1.as_mut().unwrap().borrow_mut().val += t2.as_ref().unwrap().borrow().val;
        {
            let lch1 = &mut t1.as_mut().unwrap().borrow_mut().left;
            let lch2 = &t2.as_ref().unwrap().borrow().left;
            if lch1.is_none() {
                *lch1 = lch2.clone();
            } else {
                Self::dfs(lch1, lch2);
            }
        }
        let rch1 = &mut t1.as_mut().unwrap().borrow_mut().right;
        let rch2 = &t2.as_ref().unwrap().borrow().right;
        if rch1.is_none() {
            *rch1 = rch2.clone();
        } else {
            Self::dfs(rch1, rch2);
        }
    }
}

struct Solution;

 #[cfg(test)]
 mod tests {
     use super::Solution;

     #[test]
     fn test_merge_trees() {
         assert_eq!(1+1, 2);
     }
 }